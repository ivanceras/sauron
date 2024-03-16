//! Leaf node for html dom tree
use crate::dom::StatefulModel;
use crate::dom::StatelessModel;
use std::borrow::Cow;
use std::fmt;

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
    /// Stateful Component leaf
    StatefulComponent(StatefulModel<MSG>),
    /// Stateless Component leaf
    StatelessComponent(StatelessModel<MSG>),
}

impl<MSG> Clone for Leaf<MSG> {
    fn clone(&self) -> Self {
        match self {
            Self::Text(v) => Self::Text(v.clone()),
            Self::SafeHtml(v) => Self::SafeHtml(v.clone()),
            Self::Comment(v) => Self::Comment(v.clone()),
            Self::DocType(v) => Self::DocType(v.clone()),
            Self::StatefulComponent(v) => Self::StatefulComponent(v.clone()),
            Self::StatelessComponent(v) => Self::StatelessComponent(v.clone()),
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
            Self::StatefulComponent(v) => write!(f, "StatefulComponent({:?})", v.type_id),
            Self::StatelessComponent(v) => write!(f, "StatelessComponent({:?})", v.type_id),
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
            (Self::StatefulComponent(v), Self::StatefulComponent(o)) => v.type_id == o.type_id,
            (Self::StatelessComponent(v), Self::StatelessComponent(o)) => v == o,
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

    /// mape the msg of this Leaf such that `Leaf<MSG>` becomes `Leaf<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> Leaf<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        match self {
            Self::Text(v) => Leaf::Text(v),
            Self::SafeHtml(v) => Leaf::SafeHtml(v),
            Self::Comment(v) => Leaf::Comment(v),
            Self::DocType(v) => Leaf::DocType(v),
            Self::StatefulComponent(v) => Leaf::StatefulComponent(v.map_msg(cb)),
            Self::StatelessComponent(v) => Leaf::StatelessComponent(v.map_msg(cb)),
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
