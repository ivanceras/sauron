//! special attributes which is treated differently
//!
//!
use super::{attr, Attribute, Value};
use crate::{Element, Node};

/// Special Node attributes that are treated differently
/// such as key and skip which both greatly affects the diffing algorithm
///
pub trait Special {
    /// return the value of "key" attribute
    fn get_key(&self) -> Option<&Value> {
        self.get_value("key")
    }

    /// return the boolean value of the "focus" attribute of this node
    fn is_focused(&self) -> bool {
        self.get_value("focus")
            .map(|v| v.as_bool())
            .flatten()
            .unwrap_or(false)
    }

    /// get the first attribute value with this attribute name
    fn get_value<'a>(&'a self, att_name: &'static str) -> Option<&'a Value>;
}

impl<MSG> Special for Node<MSG> {
    fn get_value<'a>(&'a self, att_name: &'static str) -> Option<&'a Value> {
        self.get_attribute_value(&att_name)
            .map(|att_values| {
                att_values.first().map(|v| v.get_simple()).flatten()
            })
            .flatten()
    }
}

impl<MSG> Special for Element<MSG> {
    fn get_value<'a>(&'a self, att_name: &'static str) -> Option<&'a Value> {
        self.get_attribute_value(&att_name)
            .map(|att_values| {
                att_values.first().map(|v| v.get_simple()).flatten()
            })
            .flatten()
    }
}

/// key attributes is used to match
/// old element and new element when diffing
pub fn key<V, MSG>(v: V) -> Attribute<MSG>
where
    V: Into<Value>,
{
    attr("key", v)
}

/// if the value is true, then the diffing of this element
/// and its descendants are skip entirely
pub fn skip<MSG>(v: bool) -> Attribute<MSG> {
    attr("skip", v)
}

/// if the value is true, then this node is made to replace the old
/// node it matches
pub fn replace<MSG>(v: bool) -> Attribute<MSG> {
    attr("replace", v)
}
