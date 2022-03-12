//! Leaf node for html dom tree

use super::Element;
use std::fmt;

/// A leaf node value of html dom tree
pub enum Leaf<MSG> {
    /// Text variant of a virtual node
    Text(String),
    /// A safe html variant
    SafeHtml(String),
    /// A comment node
    Comment(String),
    /// A custom element
    CustomElement(Element<MSG>),
}

impl<MSG> PartialEq for Leaf<MSG> {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (Self::Text(this), Self::Text(other)) => this == other,
            (Self::SafeHtml(this), Self::SafeHtml(other)) => this == other,
            (Self::Comment(this), Self::Comment(other)) => this == other,
            (Self::CustomElement(this), Self::CustomElement(other)) => {
                this == other
            }
            _ => false,
        }
    }
}

impl<MSG> fmt::Debug for Leaf<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(this) => this.fmt(f),
            Self::SafeHtml(this) => this.fmt(f),
            Self::Comment(this) => this.fmt(f),
            Self::CustomElement(this) => this.fmt(f),
        }
    }
}

impl<MSG> Clone for Leaf<MSG> {
    fn clone(&self) -> Self {
        match self {
            Self::Text(this) => Self::Text(this.clone()),
            Self::SafeHtml(this) => Self::SafeHtml(this.clone()),
            Self::Comment(this) => Self::Comment(this.clone()),
            Self::CustomElement(this) => Self::CustomElement(this.clone()),
        }
    }
}

impl<MSG> Leaf<MSG> {
    /// returns true if this a text node
    pub fn is_text(&self) -> bool {
        match self {
            Self::Text(_) => true,
            _ => false,
        }
    }
    /// returns true if this is a safe html text node
    pub fn is_safe_html(&self) -> bool {
        match self {
            Self::SafeHtml(_) => true,
            _ => false,
        }
    }

    /// unwrap the text content if this a text node,
    /// panics if it is not a text node
    pub fn unwrap_text(&self) -> &str {
        match self {
            Self::Text(ref text) => text,
            _ => panic!("node is not a text"),
        }
    }

    /// unwrap the text content if this a text node,
    /// panics if it is not a text node
    pub fn unwrap_safe_html(&self) -> &str {
        match self {
            Self::SafeHtml(ref html) => html,
            _ => panic!("node is not a text"),
        }
    }
}

/// create a text leaf
pub fn text<MSG>(s: impl ToString) -> Leaf<MSG> {
    Leaf::Text(s.to_string())
}

/// create a safe html leaf
pub fn safe_html<MSG>(s: impl ToString) -> Leaf<MSG> {
    Leaf::SafeHtml(s.to_string())
}

/// create a comment leaf
pub fn comment<MSG>(s: impl ToString) -> Leaf<MSG> {
    Leaf::Comment(s.to_string())
}
