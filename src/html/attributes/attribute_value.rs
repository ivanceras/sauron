use crate::{
    prelude::{
        Callback,
        Style,
        Value,
    },
    Event,
};

/// Values of an attribute can be in these variants
#[derive(Debug)]
pub enum AttributeValue<MSG> {
    /// an event listener
    Callback(Callback<Event, MSG>),
    /// an argument value, to be called as parameter, the function is called to the element
    FunctionCall(Value),
    /// a simple value, wrapper of primitive types
    Simple(Value),
    /// style values
    Style(Vec<Style>),
    /// no value
    Empty,
}

impl<MSG> PartialEq for AttributeValue<MSG> {
    /// all callbacks will have to return equal
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AttributeValue::Simple(this), AttributeValue::Simple(other)) => {
                this == other
            }
            (AttributeValue::Style(this), AttributeValue::Style(other)) => {
                this == other
            }
            _ => true,
        }
    }
}

impl<MSG> AttributeValue<MSG> {
    /// create an attribute from Vec<Style>
    pub fn from_styles(styles: Vec<Style>) -> Self {
        AttributeValue::Style(styles)
    }

    /// create an attribute value from simple value
    pub fn from_value(value: Value) -> Self {
        AttributeValue::Simple(value)
    }

    /// create an attribute value from callback
    pub fn from_callback(cb: Callback<Event, MSG>) -> Self {
        AttributeValue::Callback(cb)
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

    /// return the callback if it is a callback variant
    pub fn get_callback(&self) -> Option<&Callback<Event, MSG>> {
        match self {
            AttributeValue::Callback(cb) => Some(&cb),
            _ => None,
        }
    }
}
