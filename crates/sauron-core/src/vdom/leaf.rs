//! Leaf node for html dom tree
//!

/// A leaf node value of html dom tree
#[derive(PartialEq, Debug, Clone)]
pub enum Leaf {
    /// Text variant of a virtual node
    Text(String),
    /// A safe html variant
    SafeHtml(String),
    /// A comment node
    Comment(String),
}

impl Leaf {
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
