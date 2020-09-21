use crate::prelude::{
    Style,
    Value,
};

/// Values of an attribute can be in these variants
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    /// an argument value, to be called as parameter, the function is called to the element
    FunctionCall(Value),
    /// a simple value, wrapper of primitive types
    Simple(Value),
    /// style values
    Style(Vec<Style>),
    /// no value
    Empty,
}

impl AttributeValue {
    /// create an attribute from Vec<Style>
    pub fn from_styles(styles: Vec<Style>) -> Self {
        AttributeValue::Style(styles)
    }

    /// create an attribute value from simple value
    pub fn from_value(value: Value) -> Self {
        AttributeValue::Simple(value)
    }

    /// create an attribute from a function `name` with arguments `value`
    pub fn function_call(value: Value) -> Self {
        AttributeValue::FunctionCall(value)
    }

    /// return the value if it is a Simple variant
    pub fn get_simple(&self) -> Option<&Value> {
        match self {
            AttributeValue::Simple(v) => Some(&v),
            _ => None,
        }
    }

    /// return the function call argument value if it is a FunctionCall variant
    pub fn get_function_call_value(&self) -> Option<&Value> {
        match self {
            AttributeValue::FunctionCall(v) => Some(&v),
            _ => None,
        }
    }

    /// returns true if this attribute value is a style
    pub fn is_style(&self) -> bool {
        match self {
            AttributeValue::Style(_) => true,
            _ => false,
        }
    }

    /// return the styles if the attribute value is a style
    pub fn as_style(&self) -> Option<&Vec<Style>> {
        match self {
            AttributeValue::Style(styles) => Some(styles),
            _ => None,
        }
    }

    /// return true if this is a function call
    pub fn is_function_call(&self) -> bool {
        match self {
            AttributeValue::FunctionCall(_) => true,
            _ => false,
        }
    }

    /// returns true if this attribute value is empty
    pub fn is_empty(&self) -> bool {
        match self {
            AttributeValue::Empty => true,
            _ => false,
        }
    }
}
