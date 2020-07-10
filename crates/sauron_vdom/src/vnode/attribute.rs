use crate::{
    Callback,
    Value,
};
use std::{
    fmt,
    fmt::Write,
};

/// These are the attributes of an element
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute<ATT, EVENT, MSG>
where
    ATT: Clone,
{
    /// the attribute name
    pub name: ATT,
    /// the attribute value, which could be a simple value, and event or a function call
    pub value: AttribValue<EVENT, MSG>,
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub namespace: Option<&'static str>,
}

impl<ATT, EVENT, MSG> Attribute<ATT, EVENT, MSG>
where
    ATT: Clone,
{
    /// create an attribute from Callback
    pub fn from_callback(name: ATT, cb: Callback<EVENT, MSG>) -> Self {
        Attribute {
            name,
            value: cb.into(),
            namespace: None,
        }
    }

    /// create an attribute from Value type
    pub fn from_value(name: ATT, value: Value) -> Self {
        Attribute {
            name,
            value: value.into(),
            namespace: None,
        }
    }
}

/// The value of this attribute with 3 variants
#[derive(Debug, Clone, PartialEq)]
pub enum AttribValue<EVENT, MSG> {
    /// normal attribute value
    Value(Value),
    /// function call such as value, checked, innerHTML
    FuncCall(Value),
    /// callback such as used in oninput, onclick
    Callback(Callback<EVENT, MSG>),
}

impl<ATT, EVENT, MSG> Attribute<ATT, EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
    ATT: PartialEq + Ord + ToString + Clone,
{
    /// map/transform the callback of this attribute where MSG becomes MSG2
    pub(super) fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Attribute<ATT, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        Attribute {
            name: self.name,
            value: self.value.map_callback(cb),
            namespace: self.namespace,
        }
    }

    /// check whether this attribute is an event listener
    /// TODO: rename this to is_event_listener
    pub fn is_event(&self) -> bool {
        self.value.is_event()
    }

    /// check whether this attribute is a value
    pub fn is_value(&self) -> bool {
        self.value.is_value()
    }

    /// check whether this attribute is a func call value
    /// such as `inner_html` etc.
    pub fn is_func_call(&self) -> bool {
        self.value.is_func_call()
    }

    /// transform the callback of this attribute where EVENT becomes EVENT2
    pub fn reform<F, EVENT2>(self, func: F) -> Attribute<ATT, EVENT2, MSG>
    where
        F: Fn(EVENT2) -> EVENT + 'static,
        EVENT2: 'static,
    {
        Attribute {
            name: self.name,
            value: self.value.reform(func),
            namespace: self.namespace,
        }
    }

    /// returns a reference to the value of this attribute
    pub fn get_value(&self) -> Option<&Value> {
        self.value.get_value()
    }

    /// returns the reference to the callback of this attribute
    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        self.value.get_callback()
    }

    /// consume the attribute and take the callback
    pub fn take_callback(self) -> Option<Callback<EVENT, MSG>> {
        self.value.take_callback()
    }

    /// create a nice string representation of this attribute
    pub fn render(&self, buffer: &mut dyn Write) -> fmt::Result
    where
        ATT: ToString,
    {
        if self.is_value() {
            if let Some(_ns) = self.namespace {
                //TODO: the xlink part of this namespace should be passed by the calling function
                write!(
                    buffer,
                    r#"xlink:{}="{}""#,
                    self.name.to_string(),
                    self.value
                )?;
            } else {
                write!(
                    buffer,
                    r#"{}="{}""#,
                    self.name.to_string(),
                    self.value
                )?;
            }
        }
        Ok(())
    }
}

impl<EVENT, MSG> AttribValue<EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
{
    fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> AttribValue<EVENT, MSG2>
    where
        MSG2: 'static,
    {
        match self {
            AttribValue::Value(value) => AttribValue::Value(value),
            AttribValue::FuncCall(value) => AttribValue::FuncCall(value),
            AttribValue::Callback(existing) => {
                AttribValue::Callback(existing.map_callback(cb))
            }
        }
    }

    fn reform<F, EVENT2>(self, func: F) -> AttribValue<EVENT2, MSG>
    where
        F: Fn(EVENT2) -> EVENT + 'static,
        EVENT2: 'static,
    {
        match self {
            AttribValue::Value(value) => AttribValue::Value(value),
            AttribValue::FuncCall(value) => AttribValue::FuncCall(value),
            AttribValue::Callback(cb) => AttribValue::Callback(cb.reform(func)),
        }
    }

    fn is_value(&self) -> bool {
        match self {
            AttribValue::Value(_) => true,
            _ => false,
        }
    }

    fn is_event(&self) -> bool {
        match self {
            AttribValue::Callback(_) => true,
            _ => false,
        }
    }

    fn is_func_call(&self) -> bool {
        match self {
            AttribValue::FuncCall(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        match self {
            AttribValue::Value(_) => None,
            AttribValue::FuncCall(_) => None,
            AttribValue::Callback(cb) => Some(cb),
        }
    }

    pub(crate) fn take_callback(self) -> Option<Callback<EVENT, MSG>> {
        match self {
            AttribValue::Value(_) => None,
            AttribValue::FuncCall(_) => None,
            AttribValue::Callback(cb) => Some(cb),
        }
    }

    /// returh the value if it is a Value variant
    pub fn get_value(&self) -> Option<&Value> {
        match self {
            AttribValue::Value(value) => Some(value),
            AttribValue::FuncCall(value) => Some(value),
            AttribValue::Callback(_) => None,
        }
    }
}

impl<EVENT, MSG> fmt::Display for AttribValue<EVENT, MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttribValue::Value(value) => write!(f, "{}", value),
            AttribValue::FuncCall(_) => write!(f, ""),
            AttribValue::Callback(_) => write!(f, ""),
        }
    }
}

impl<EVENT, MSG> From<Callback<EVENT, MSG>> for AttribValue<EVENT, MSG> {
    fn from(cb: Callback<EVENT, MSG>) -> Self {
        AttribValue::Callback(cb)
    }
}

impl<EVENT, MSG> From<Value> for AttribValue<EVENT, MSG> {
    fn from(value: Value) -> Self {
        AttribValue::Value(value)
    }
}
