use crate::vdom::Node;

/// additional traits for mt_dom::Node
pub trait NodeTrait {
    /// returns true if this is a text node
    fn is_text(&self) -> bool;

    /// returns true if this is text node with safe html as the content
    fn is_safe_html(&self) -> bool;

    /// unwrap the text content of the text node, panics if it is not a text node
    fn unwrap_text(&self) -> &str;

    /// unwrap the html text content of this node, panics if it is not a safe html node
    fn unwrap_safe_html(&self) -> &str;
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

    fn unwrap_text(&self) -> &str {
        match self {
            Self::Leaf(ref leaf) => leaf.unwrap_text(),
            _ => panic!("not a leaf node"),
        }
    }

    fn unwrap_safe_html(&self) -> &str {
        match self {
            Self::Leaf(ref leaf) => leaf.unwrap_safe_html(),
            _ => panic!("not a leaf node"),
        }
    }
}
