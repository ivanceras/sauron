use crate::{
    Callback,
    Value,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute<EVENT, MSG> {
    pub name: &'static str,
    pub value: AttribValue<EVENT, MSG>,
    pub namespace: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttribValue<EVENT, MSG> {
    Value(Value),
    Callback(Callback<EVENT, MSG>),
}

impl<EVENT, MSG> Attribute<EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
{
    pub(super) fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Attribute<EVENT, MSG2>
    where
        MSG2: 'static,
    {
        Attribute {
            name: self.name,
            value: self.value.map_callback(cb),
            namespace: self.namespace,
        }
    }

    pub fn is_event(&self) -> bool {
        self.value.is_event()
    }

    pub fn reform<F, EVENT2>(self, func: F) -> Attribute<EVENT2, MSG>
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

    pub fn get_value(&self) -> Option<&Value> {
        self.value.get_value()
    }

    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        self.value.get_callback()
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
            AttribValue::Callback(cb) => AttribValue::Callback(cb.reform(func)),
        }
    }

    fn is_event(&self) -> bool {
        match self {
            AttribValue::Value(_) => false,
            AttribValue::Callback(_) => true,
        }
    }

    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        match self {
            AttribValue::Value(_) => None,
            AttribValue::Callback(cb) => Some(cb),
        }
    }

    pub fn get_value(&self) -> Option<&Value> {
        match self {
            AttribValue::Value(value) => Some(value),
            AttribValue::Callback(_) => None,
        }
    }
}

impl<EVENT, MSG> fmt::Display for AttribValue<EVENT, MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttribValue::Value(value) => write!(f, "{}", value),
            AttribValue::Callback(cb) => write!(f, "{:?}", cb),
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
