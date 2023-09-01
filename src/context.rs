use std::collections::HashMap;

/// Context represents a chain of values
#[derive(Default)]
pub struct Context<'a> {
    fallback: Option<&'a Context<'a>>,
    data: HashMap<&'a str, String>,
}

impl<'a> Context<'a> {
    /// Returns a new empty context having self as its fallback.
    pub fn clone(&'a self) -> Self {
        Self {
            fallback: Some(self),
            data: HashMap::default(),
        }
    }

    /// Adds a new value to the context.
    pub fn with_value(mut self, key: &'a str, value: String) -> Self {
        self.data.insert(key, value);
        self
    }

    /// Returns the value of the given key if, and only if, that key exists somewhere
    /// in the context's chain.
    pub fn value(&self, key: &str) -> Option<&str> {
        self.data
            .get(key)
            .map(|value| value.as_str())
            .or_else(|| self.fallback.and_then(|ctx| ctx.value(key)))
    }
}
