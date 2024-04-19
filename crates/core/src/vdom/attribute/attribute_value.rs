use crate::vdom::ComponentEventCallback;
use crate::{html::attributes::Style, vdom::EventCallback, vdom::Value};
use derive_where::derive_where;

/// Values of an attribute can be in these variants
#[derive_where(Clone, Debug)]
pub enum AttributeValue<MSG> {
    /// a simple value, wrapper of primitive types
    Simple(Value),
    /// style values
    Style(Vec<Style>),
    /// Event EventCallback
    EventListener(EventCallback<MSG>),
    /// Component Event Listener
    ComponentEventListener(ComponentEventCallback),
    /// no value
    Empty,
}

/// This is written manually, so we don't push
/// constraint on MSG to be PartialEq
/// and also, derive_where can not equate on event listeners
impl<MSG> PartialEq for AttributeValue<MSG> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AttributeValue::Simple(this), AttributeValue::Simple(other)) => this == other,
            (AttributeValue::Style(this), AttributeValue::Style(other)) => this == other,
            (AttributeValue::EventListener(this), AttributeValue::EventListener(other)) => {
                this == other
            }
            (AttributeValue::ComponentEventListener(this), AttributeValue::ComponentEventListener(other)) => {
                this == other
            }
            (AttributeValue::Empty, AttributeValue::Empty) => true,
            (_, _) => false,
        }
    }
}

impl<MSG> Eq for AttributeValue<MSG> {}

impl<MSG> From<EventCallback<MSG>> for AttributeValue<MSG> {
    fn from(listener: EventCallback<MSG>) -> Self {
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

    /// returns true if this attribute value is a style
    pub fn is_style(&self) -> bool {
        matches!(self, Self::Style(_))
    }

    /// return the styles if the attribute value is a style
    pub fn as_event_listener(&self) -> Option<&EventCallback<MSG>> {
        match self {
            Self::EventListener(cb) => Some(cb),
            _ => None,
        }
    }

    /// return true if this is an event listener
    pub fn is_event_listener(&self) -> bool {
        matches!(self, Self::EventListener(_))
    }

    /// return the styles if the attribute value is a style
    pub fn as_style(&self) -> Option<&Vec<Style>> {
        match self {
            Self::Style(styles) => Some(styles),
            _ => None,
        }
    }

    /// returns true if this attribute value is the filler empty attribute
    pub fn is_just_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}
