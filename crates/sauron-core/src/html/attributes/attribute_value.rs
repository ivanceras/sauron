use crate::{
    html::attributes::{Style, Value},
    vdom::Listener,
};
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
            AttributeValue::FunctionCall(this) => AttributeValue::FunctionCall(this.clone()),
            AttributeValue::Simple(this) => AttributeValue::Simple(this.clone()),
            AttributeValue::Style(this) => AttributeValue::Style(this.clone()),
            AttributeValue::EventListener(this) => AttributeValue::EventListener(this.clone()),
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
            (AttributeValue::FunctionCall(this), AttributeValue::FunctionCall(other)) => {
                this == other
            }
            (AttributeValue::Simple(this), AttributeValue::Simple(other)) => this == other,
            (AttributeValue::Style(this), AttributeValue::Style(other)) => this == other,
            (AttributeValue::EventListener(this), AttributeValue::EventListener(other)) => {
                this == other
            }
            (AttributeValue::Empty, AttributeValue::Empty) => true,
            (_, _) => false,
        }
    }
}

impl<MSG> From<Listener<MSG>> for AttributeValue<MSG> {
    fn from(listener: Listener<MSG>) -> Self {
        Self::EventListener(listener)
    }
}

impl<MSG, V> From<V> for AttributeValue<MSG>
where
    V: Into<Value>,
{
    fn from(v: V) -> Self {
        Self::Simple(Into::<Value>::into(v))
    }
}

impl<MSG> AttributeValue<MSG> {
    /// create an attribute from Vec<Style>
    pub fn from_styles(styles: impl IntoIterator<Item = Style>) -> Self {
        Self::Style(styles.into_iter().collect())
    }

    /// create an attribute from a function `name` with arguments `value`
    pub fn function_call(value: Value) -> Self {
        Self::FunctionCall(value)
    }

    /// return the value if it is a Simple variant
    pub fn get_simple(&self) -> Option<&Value> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    /// return the &str value if the value is str value
    pub fn as_str(&self) -> Option<&str> {
        if let Some(simple) = self.get_simple() {
            simple.as_str()
        } else {
            None
        }
    }

    /// return the function call argument value if it is a FunctionCall variant
    pub fn get_function_call_value(&self) -> Option<&Value> {
        match self {
            Self::FunctionCall(v) => Some(v),
            _ => None,
        }
    }

    /// returns true if this attribute value is a style
    pub fn is_style(&self) -> bool {
        matches!(self, Self::Style(_))
    }

    /// return the styles if the attribute value is a style
    pub fn as_event_listener(&self) -> Option<&Listener<MSG>> {
        match self {
            Self::EventListener(cb) => Some(cb),
            _ => None,
        }
    }

    /// return the styles if the attribute value is a style
    pub fn as_style(&self) -> Option<&Vec<Style>> {
        match self {
            Self::Style(styles) => Some(styles),
            _ => None,
        }
    }

    /// return true if this is a function call
    pub fn is_function_call(&self) -> bool {
        matches!(self, Self::FunctionCall(_))
    }

    /// returns true if this attribute value is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}
