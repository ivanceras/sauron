use crate::vdom::Node;

/// additional traits for mt_dom::Node
pub trait NodeTrait {
    /// returns true if this is a text node
    fn is_text(&self) -> bool;

    /// returns true if this is text node with safe html as the content
    fn is_safe_html(&self) -> bool;

    /// return the text content if it is a text node
    fn as_text(&self) -> Option<&str>;

    /// returns the html text content of this node
    fn as_safe_html(&self) -> Option<&str>;
}

impl<MSG> NodeTrait for Node<MSG> {
    /// returns true if this is a text node
    fn is_text(&self) -> bool {
        match self {
            Self::Leaf(leaf) => leaf.is_text(),
            _ => false,
        }
    }

    /// returns true if this is a safe html text node
    fn is_safe_html(&self) -> bool {
        match self {
            Self::Leaf(leaf) => leaf.is_safe_html(),
            _ => false,
        }
    }

    fn as_text(&self) -> Option<&str> {
        match self {
            Self::Leaf(ref leaf) => leaf.as_text(),
            _ => None,
        }
    }

    fn as_safe_html(&self) -> Option<&str> {
        match self {
            Self::Leaf(ref leaf) => leaf.as_safe_html(),
            _ => None,
        }
    }
}
