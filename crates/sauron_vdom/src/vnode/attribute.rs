use crate::Value;
use crate::Callback;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute<EVENT, MSG> {
    pub name: &'static str,
    pub value: AttribValue<EVENT, MSG>,
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
    pub fn new(name: &'static str, value: AttribValue<EVENT, MSG>) -> Self {
        Attribute { name, value }
    }

    pub fn with_name_value(name: &'static str, value: Value) -> Self {
        Attribute {
            name,
            value: value.into(),
        }
    }

    pub(in super) fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Attribute<EVENT, MSG2>
    where
        MSG2: 'static,
    {
        Attribute::new(self.name, self.value.map_callback(cb))
    }

    pub(in super) fn is_event(&self) -> bool {
        self.value.is_event()
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
