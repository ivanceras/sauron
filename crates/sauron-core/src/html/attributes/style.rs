use crate::html::attributes::Value;
use std::fmt;

/// Creat a style attribute
#[macro_export]
macro_rules! style {
    ( $($arg: tt)* ) => {
        $crate::attributes::attr("style", $crate::jss::style!{$($arg)*})
    };
}

/// css styles
/// style can be converted into an attribute
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// style name such as border, width, etc
    pub name: String,
    /// value of the style
    pub value: Value,
}

impl Style {
    /// create a style with name and value
    pub fn new(name: impl ToString, value: impl Into<Value>) -> Self {
        Style {
            name: name.to_string(),
            value: value.into(),
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.value)
    }
}
