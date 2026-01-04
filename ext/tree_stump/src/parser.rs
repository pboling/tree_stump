use crate::query::Query;
use crate::tree::Tree;
use crate::util::build_error;
use crate::LANG_LANGUAGES;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[magnus::wrap(class = "TreeStump::Parser")]
pub struct Parser {
    raw_parser: RefCell<tree_sitter::Parser>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            raw_parser: RefCell::new(tree_sitter::Parser::new()),
        }
    }

    pub fn set_language(&self, lang: String) -> Result<bool, magnus::Error> {
        let languages = LANG_LANGUAGES.get_or_init(|| Mutex::new(HashMap::new()));
        let languages = languages.lock().map_err(|e| {
            build_error(format!("Failed to acquire language lock: {}", e))
        })?;
        let language = languages.get(&lang);
        match language {
            Some(language) => {
                // tree-sitter 0.26+ set_language takes impl Into<LanguageRef<'_>>
                let result = self.raw_parser.borrow_mut().set_language(language);
                result.map_or_else(|e| Err(build_error(e.to_string())), |_| Ok(true))
            }
            None => Err(build_error(format!("Language {} is not registered", lang))),
        }
    }

    pub fn parse(&self, source: String) -> Result<Tree, magnus::Error> {
        let tree = self.raw_parser.borrow_mut().parse(source, None);

        match tree {
            Some(tree) => Ok(Tree::from(Arc::new(tree))),
            None => Err(build_error("Failed to parse")),
        }
    }

    pub fn reset(&self) {
        self.raw_parser.borrow_mut().reset();
    }

    // NOTE: timeout_micros and set_timeout_micros were removed in tree-sitter 0.26
    // The timeout functionality is no longer available in the C/Rust API

    fn language(&self) -> Option<tree_sitter::LanguageRef<'static>> {
        // tree-sitter 0.24+ returns LanguageRef
        // We need to use unsafe to extend the lifetime since the language outlives the parser borrow
        self.raw_parser.borrow().language().map(|lang_ref| {
            // SAFETY: The language is stored globally and outlives all parsers
            unsafe { std::mem::transmute::<tree_sitter::LanguageRef<'_>, tree_sitter::LanguageRef<'static>>(lang_ref) }
        })
    }

    pub fn build_query(&self, source: String) -> Result<Query, magnus::Error> {
        let lang = self.language();
        lang.map_or_else(
            || Err(build_error("Failed to get language from parser")),
            |lang| Query::new(&lang, source),
        )
    }
}
