//! Leaf node for html dom tree

use std::borrow::Cow;

/// A leaf node value of html dom tree
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Leaf {
    /// Text variant of a virtual node
    Text(Cow<'static, str>),
    /// A safe html variant
    SafeHtml(Cow<'static, str>),
    /// A comment node
    Comment(Cow<'static, str>),
    /// doctype: html, math, svg
    /// <https://www.w3.org/QA/2002/04/valid-dtd-list.html>
    DocType(Cow<'static, str>),
}

impl Leaf {
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
        match self{
            Self::Text(v) => matches!(v, Cow::Borrowed(_)),
            Self::SafeHtml(v) => matches!(v, Cow::Borrowed(_)),
            Self::Comment(v) => matches!(v, Cow::Borrowed(_)),
            Self::DocType(v) => matches!(v, Cow::Borrowed(_)),
        }
    }
}

impl From<&'static str> for Leaf {
    fn from(s: &'static str) -> Self {
        Self::Text(Cow::from(s))
    }
}

impl From<String> for Leaf {
    fn from(s: String) -> Self {
        Self::Text(Cow::from(s))
    }
}


