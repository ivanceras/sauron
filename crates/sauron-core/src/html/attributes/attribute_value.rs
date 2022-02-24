use crate::prelude::{Style, Value};
use crate::vdom::Listener;
use std::fmt::{self, Debug};

/// Values of an attribute can be in these variants
pub enum AttributeValue<MSG> {
    /// an argument value, to be called as parameter, the function is called to the element
    FunctionCall(Value),
    /// a simple value, wrapper of primitive types
    Simple(Value),
    /// style values
    Style(Vec<Style>),
    /// Event Listener
    EventListener(Listener<MSG>),
    /// no value
    Empty,
}

/// This is written manually, so we don't push
/// constraint on MSG to be Clone
impl<MSG> Clone for AttributeValue<MSG> {
    fn clone(&self) -> Self {
        match self {
            AttributeValue::FunctionCall(this) => {
                AttributeValue::FunctionCall(this.clone())
            }
            AttributeValue::Simple(this) => {
                AttributeValue::Simple(this.clone())
            }
            AttributeValue::Style(this) => AttributeValue::Style(this.clone()),
            AttributeValue::EventListener(this) => {
                AttributeValue::EventListener(this.clone())
            }
            AttributeValue::Empty => AttributeValue::Empty,
        }
    }
}

/// This is written manually, so we don't push
/// constraint on MSG to be Debug
impl<MSG> Debug for AttributeValue<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttributeValue::FunctionCall(this) => this.fmt(f),
            AttributeValue::Simple(this) => this.fmt(f),
            AttributeValue::Style(this) => this.fmt(f),
            AttributeValue::EventListener(this) => this.fmt(f),
            AttributeValue::Empty => write!(f, "Empty"),
        }
    }
}

/// This is written manually, so we don't push
/// constraint on MSG to be PartialEq
impl<MSG> PartialEq for AttributeValue<MSG> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                AttributeValue::FunctionCall(this),
                AttributeValue::FunctionCall(other),
            ) => this == other,
            (AttributeValue::Simple(this), AttributeValue::Simple(other)) => {
                this == other
            }
            (AttributeValue::Style(this), AttributeValue::Style(other)) => {
                this == other
            }
            (
                AttributeValue::EventListener(this),
                AttributeValue::EventListener(other),
            ) => this == other,
            (AttributeValue::Empty, AttributeValue::Empty) => true,
            (_, _) => false,
        }
    }
}

impl<MSG> AttributeValue<MSG> {
    /// create an attribute from Vec<Style>
    pub fn from_styles(styles: impl IntoIterator<Item = Style>) -> Self {
        AttributeValue::Style(styles.into_iter().collect())
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
            AttributeValue::Simple(v) => Some(v),
            _ => None,
        }
    }

    /// return the function call argument value if it is a FunctionCall variant
    pub fn get_function_call_value(&self) -> Option<&Value> {
        match self {
            AttributeValue::FunctionCall(v) => Some(v),
            _ => None,
        }
    }

    /// returns true if this attribute value is a style
    pub fn is_style(&self) -> bool {
        matches!(self, AttributeValue::Style(_))
    }

    /// return the styles if the attribute value is a style
    pub fn as_event_listener(&self) -> Option<&Listener<MSG>> {
        match self {
            AttributeValue::EventListener(cb) => Some(cb),
            _ => None,
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
        matches!(self, AttributeValue::FunctionCall(_))
    }

    /// returns true if this attribute value is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, AttributeValue::Empty)
    }
}
