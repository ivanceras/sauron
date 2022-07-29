//! Leaf node for html dom tree
use crate::vdom::Node;
use std::fmt;

/// A leaf node value of html dom tree
pub enum Leaf<MSG> {
    /// Text variant of a virtual node
    Text(String),
    /// A safe html variant
    SafeHtml(String),
    /// A comment node
    Comment(String),
    /// a vec of nodes
    Fragment(Vec<Node<MSG>>),
    /// doctype: html, math, svg
    /// <https://www.w3.org/QA/2002/04/valid-dtd-list.html>
    DocType(String),
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

impl<MSG> fmt::Debug for Leaf<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(v) => write!(f, "Text({})", v),
            Self::SafeHtml(v) => write!(f, "SafeHtml({})", v),
            Self::Comment(v) => write!(f, "Comment({})", v),
            Self::Fragment(v) => {
                write!(f, "Fragment:")?;
                f.debug_list().entries(v).finish()
            }
            Self::DocType(v) => write!(f, "DocType({})", v),
        }
    }
}

impl<MSG> Clone for Leaf<MSG> {
    fn clone(&self) -> Self {
        match self {
            Self::Text(v) => Self::Text(v.clone()),
            Self::SafeHtml(v) => Self::SafeHtml(v.clone()),
            Self::Comment(v) => Self::Comment(v.clone()),
            Self::Fragment(v) => Self::Fragment(v.clone()),
            Self::DocType(v) => Self::DocType(v.clone()),
        }
    }
}

impl<MSG> PartialEq for Leaf<MSG> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(v), Self::Text(o)) => v == o,
            (Self::SafeHtml(v), Self::SafeHtml(o)) => v == o,
            (Self::Comment(v), Self::Comment(o)) => v == o,
            (Self::Fragment(v), Self::Fragment(o)) => v == o,
            (Self::DocType(v), Self::DocType(o)) => v == o,
            _ => false,
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

/// create a fragment leaf
pub fn fragment<MSG>(nodes: impl IntoIterator<Item = Node<MSG>>) -> Leaf<MSG> {
    Leaf::Fragment(nodes.into_iter().collect())
}

/// create a doctype leaf
pub fn doctype<MSG>(s: impl ToString) -> Leaf<MSG> {
    Leaf::DocType(s.to_string())
}
