use crate::vnode::Value;
use std::fmt;

/// css styles
/// style can be converted into an attribute
/// ie:
/// ```
/// style="..."
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Style<ATT> {
    /// style name such as border, width, etc
    pub name: ATT,
    /// value of the style
    pub value: Value,
}

impl<ATT> Style<ATT> {
    /// create a style with name and value
    pub fn new(name: ATT, value: Value) -> Self {
        Style { name, value }
    }
}

impl<ATT> fmt::Display for Style<ATT>
where
    ATT: ToString,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name.to_string(), self.value)
    }
}
