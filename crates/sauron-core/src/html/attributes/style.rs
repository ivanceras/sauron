use crate::html::attributes::Value;
use std::fmt;
use std::borrow::Cow;

/// css styles
/// style can be converted into an attribute
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// style name such as border, width, etc
    pub name: Cow<'static, str>,
    /// value of the style
    pub value: Value,
}

impl Style {
    /// create a style with name and value
    pub fn new(name: impl Into<Cow<'static, str>>, value: impl Into<Value>) -> Self {
        Style {
            name: Cow::from(name.into()),
            value: value.into(),
        }
    }

    /// returns true if both the name and value is static str
    pub(crate) fn is_static_str(&self) -> bool {
        matches!(self.name, Cow::Borrowed(_)) && self.value.is_static_str()
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.value)
    }
}
