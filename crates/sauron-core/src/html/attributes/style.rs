use crate::prelude::Value;
use std::fmt;

/// css styles
/// style can be converted into an attribute
/// ie:
/// ```ignore,no_run
/// style="..."
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// style name such as border, width, etc
    pub name: &'static str,
    /// value of the style
    pub value: Value,
}

impl Style {
    /// create a style with name and value
    pub fn new(name: &'static str, value: Value) -> Self {
        Style { name, value }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.value)
    }
}
