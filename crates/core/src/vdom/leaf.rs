//! Leaf node for html dom tree
use std::borrow::Cow;
use std::fmt;
use std::any::TypeId;
use crate::vdom::{Attribute,Node};

/// A leaf node value of html dom tree
pub enum Leaf<MSG> {
    /// Text variant of a virtual node
    Text(Cow<'static, str>),
    /// A safe html variant
    SafeHtml(Cow<'static, str>),
    /// A comment node
    Comment(Cow<'static, str>),
    /// doctype: html, math, svg
    /// <https://www.w3.org/QA/2002/04/valid-dtd-list.html>
    DocType(Cow<'static, str>),
    /// Component leaf
    /// Note: we can not use the Box<dyn Component> here
    /// since it will not be possible to map_msg the Component
    /// instead we just use the type_id for looking up it's instantiated Component in a v-table.
    Component{
        /// component type id
        type_id: TypeId, 
        /// component attributes
        attrs: Vec<Attribute<MSG>>,
        /// component children
        children: Vec<Node<MSG>>,
    },
}


impl<MSG> Clone for Leaf<MSG> {
    fn clone(&self) -> Self {
        match self {
            Self::Text(v) => Self::Text(v.clone()),
            Self::SafeHtml(v) => Self::SafeHtml(v.clone()),
            Self::Comment(v) => Self::Comment(v.clone()),
            Self::DocType(v) => Self::DocType(v.clone()),
            Self::Component{type_id, attrs, children} => Self::Component{
                type_id: type_id.clone(), 
                attrs: attrs.clone(),
                children: children.clone(),
            }
        }
    }
}

impl<MSG> fmt::Debug for Leaf<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(v) => write!(f, "Text({v})"),
            Self::SafeHtml(v) => write!(f, "SafeHtml({v})"),
            Self::Comment(v) => write!(f, "Comment({v})"),
            Self::DocType(v) => write!(f, "DocType({v}"),
            Self::Component{..} => write!(f, "Component(..)"),
        }
    }
}

impl<MSG> PartialEq for Leaf<MSG> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(v), Self::Text(o)) => v == o,
            (Self::SafeHtml(v), Self::SafeHtml(o)) => v == o,
            (Self::Comment(v), Self::Comment(o)) => v == o,
            (Self::DocType(v), Self::DocType(o)) => v == o,
            (Self::Component{type_id,..}, Self::Component{type_id: o_tid,..}) => type_id==o_tid,
            _ => false,
        }
    }
}

impl<MSG> Eq for Leaf<MSG> {}

impl<MSG> Leaf<MSG> {
    /// returns true if this a text node
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// returns true if this is a safe html text node
    pub fn is_safe_html(&self) -> bool {
        matches!(self, Self::SafeHtml(_))
    }

    /// return the text content if it is a text node
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(ref text) => Some(text),
            _ => None,
        }
    }

    /// return the text content if this a text node,
    pub fn as_safe_html(&self) -> Option<&str> {
        match self {
            Self::SafeHtml(ref html) => Some(html),
            _ => None,
        }
    }

    /// returns true if the content on the leaf is just static str
    pub(crate) fn is_static_str(&self) -> bool {
        match self {
            Self::Text(v) => matches!(v, Cow::Borrowed(_)),
            Self::SafeHtml(v) => matches!(v, Cow::Borrowed(_)),
            Self::Comment(v) => matches!(v, Cow::Borrowed(_)),
            Self::DocType(v) => matches!(v, Cow::Borrowed(_)),
            Self::Component{..} => false,
        }
    }
}

impl<MSG> From<&'static str> for Leaf<MSG> {
    fn from(s: &'static str) -> Self {
        Self::Text(Cow::from(s))
    }
}

impl<MSG> From<String> for Leaf<MSG> {
    fn from(s: String) -> Self {
        Self::Text(Cow::from(s))
    }
}
