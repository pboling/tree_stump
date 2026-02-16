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
    /// The name of the currently set language, used to look up from the global map
    language_name: RefCell<Option<String>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            raw_parser: RefCell::new(tree_sitter::Parser::new()),
            language_name: RefCell::new(None),
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
                result.map_or_else(
                    |e| Err(build_error(e.to_string())),
                    |_| {
                        // Store the language name for later lookup
                        *self.language_name.borrow_mut() = Some(lang);
                        Ok(true)
                    },
                )
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

    pub fn build_query(&self, source: String) -> Result<Query, magnus::Error> {
        let lang_name = self.language_name.borrow();
        let lang_name = lang_name.as_ref().ok_or_else(|| {
            build_error("No language set on parser")
        })?;

        let languages = LANG_LANGUAGES.get_or_init(|| Mutex::new(HashMap::new()));
        let languages = languages.lock().map_err(|e| {
            build_error(format!("Failed to acquire language lock: {}", e))
        })?;

        let language = languages.get(lang_name).ok_or_else(|| {
            build_error(format!("Language {} is no longer registered", lang_name))
        })?;

        // Clone the language to pass to Query::new
        Query::new(language, source)
    }
}
