use crate::{html::attributes::Style, vdom::Callback, vdom::Value};

use derive_where::derive_where;

/// Values of an attribute can be in these variants
#[derive_where(Clone, Debug)]
pub enum AttributeValue<MSG> {
    /// an argument value, to be called as parameter, the function is called to the element
    FunctionCall(Value),
    /// a simple value, wrapper of primitive types
    Simple(Value),
    /// style values
    Style(Vec<Style>),
    /// Event Callback
    EventListener(Callback<MSG>),
    /// no value
    Empty,
}

/// This is written manually, so we don't push
/// constraint on MSG to be PartialEq
/// and also, derive_where can not equate on event listeners
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

impl<MSG> Eq for AttributeValue<MSG> {}

impl<MSG> From<Callback<MSG>> for AttributeValue<MSG> {
    fn from(listener: Callback<MSG>) -> Self {
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

    pub(crate) fn is_static_str(&self) -> bool {
        match self {
            Self::Simple(v) => v.is_static_str(),
            Self::Style(values) => values.iter().all(|v| v.is_static_str()),
            Self::Empty => true,
            _ => false,
        }
    }

    /// return the styles if the attribute value is a style
    pub fn as_event_listener(&self) -> Option<&Callback<MSG>> {
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
