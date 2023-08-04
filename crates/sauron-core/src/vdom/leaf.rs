//! Leaf node for html dom tree
use std::fmt;

/// A leaf node value of html dom tree
pub enum Leaf {
    /// Text variant of a virtual node
    Text(String),
    /// A safe html variant
    SafeHtml(String),
    /// A comment node
    Comment(String),
    /// doctype: html, math, svg
    /// <https://www.w3.org/QA/2002/04/valid-dtd-list.html>
    DocType(String),
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
}

impl fmt::Debug for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(v) => write!(f, "Text({v})"),
            Self::SafeHtml(v) => write!(f, "SafeHtml({v})"),
            Self::Comment(v) => write!(f, "Comment({v})"),
            Self::DocType(v) => write!(f, "DocType({v})"),
        }
    }
}

impl Clone for Leaf {
    fn clone(&self) -> Self {
        match self {
            Self::Text(v) => Self::Text(v.clone()),
            Self::SafeHtml(v) => Self::SafeHtml(v.clone()),
            Self::Comment(v) => Self::Comment(v.clone()),
            Self::DocType(v) => Self::DocType(v.clone()),
        }
    }
}

impl PartialEq for Leaf {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(v), Self::Text(o)) => v == o,
            (Self::SafeHtml(v), Self::SafeHtml(o)) => v == o,
            (Self::Comment(v), Self::Comment(o)) => v == o,
            (Self::DocType(v), Self::DocType(o)) => v == o,
            _ => false,
        }
    }
}

/// create a text leaf
pub fn text(s: impl ToString) -> Leaf {
    Leaf::Text(s.to_string())
}

/// create a safe html leaf
pub fn safe_html(s: impl ToString) -> Leaf {
    Leaf::SafeHtml(s.to_string())
}

/// create a comment leaf
pub fn comment(s: impl ToString) -> Leaf {
    Leaf::Comment(s.to_string())
}

/// create a doctype leaf
pub fn doctype(s: impl ToString) -> Leaf {
    Leaf::DocType(s.to_string())
}
