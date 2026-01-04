use std::borrow::Cow;

use magnus::{
    gc::register_mark_object,
    value::{InnerValue, Lazy},
    ExceptionClass, Ruby,
};

static ERROR_CLASS: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    let ex = ExceptionClass::from_value(
        ruby.eval("TreeStump::Error").expect("TreeStump::Error class not defined")
    ).expect("Failed to convert to ExceptionClass");
    register_mark_object(ex);
    ex
});

pub fn build_error(message: impl Into<Cow<'static, str>>) -> magnus::Error {
    match Ruby::get() {
        Ok(ruby) => {
            let error_class = ERROR_CLASS.get_inner_with(&ruby);
            magnus::Error::new(error_class, message)
        }
        Err(_) => {
            // Fallback when Ruby isn't initialized - use deprecated function
            // This case should rarely happen in practice
            #[allow(deprecated)]
            magnus::Error::new(magnus::exception::runtime_error(), message)
        }
    }
}
