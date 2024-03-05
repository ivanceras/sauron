use super::{AttributeName, Namespace, Tag};
use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::vdom::Leaf;
use derive_where::derive_where;

pub use attribute::Attribute;
pub use element::Element;
pub use attribute_value::AttributeValue;

pub(crate) mod attribute;
pub(crate) mod attribute_value;
mod element;

/// represents a node in a virtual dom
/// A node could be an element which can contain one or more children of nodes.
/// A node could also be just a text node which contains a string
///
/// Much of the types are Generics
///
/// Namespace - is the type for the namespace, this will be &'static str when used in html based virtual dom implementation
/// Tag - is the type for the element tag, this will be &'static str when used in html based virtual
/// dom impmenentation
/// AttributeName - is the type for the attribute name, this will be &'static str when used in html based
/// virtual dom implementation
/// AttributeValue - is the type for the value of the attribute, this will be String, f64, or just another
/// generics that suits the implementing library which used mt-dom for just dom-diffing purposes
#[derive_where(Clone, Debug, PartialEq, Eq)]
pub enum Node<MSG> {
    /// Element variant of a virtual node
    Element(Element<MSG>),
    /// A node containing nodes, this will be unrolled together with the rest of the children of
    /// the node
    NodeList(Vec<Node<MSG>>),
    /// A document fragment node, will be created using fragment node and attached to the dom
    Fragment(Vec<Node<MSG>>),
    /// A Leaf node
    Leaf(Leaf),
}



#[derive(Debug, Copy, Clone)]
pub enum Error {
    AddChildrenNotAllowed,
    AttributesNotAllowed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::AddChildrenNotAllowed => {
                write!(f, "Adding children on this node variant is not allowed")
            }
            Self::AttributesNotAllowed => {
                write!(
                    f,
                    "Adding or setting attibutes on this node variant is not allowed"
                )
            }
        }
    }
}

///TODO: use core::error when it will go out of nightly
impl std::error::Error for Error {}

impl<MSG> Node<MSG> {
    /// consume self and return the element if it is an element variant
    /// None if it is a text node
    pub fn take_element(self) -> Option<Element<MSG>> {
        match self {
            Node::Element(element) => Some(element),
            _ => None,
        }
    }

    /// returns a reference to the Leaf if the node is a Leaf variant
    pub fn leaf(&self) -> Option<&Leaf> {
        match self {
            Node::Leaf(leaf) => Some(leaf),
            _ => None,
        }
    }

    /// returns true if the node is an element variant
    pub fn is_element(&self) -> bool {
        matches!(self, Node::Element(_))
    }

    /// returns true if the node is a Leaf
    pub fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf(_))
    }

    /// returns true if the Node is a fragment variant
    pub fn is_fragment(&self) -> bool {
        matches!(self, Node::Fragment(_))
    }

    /// Get a mutable reference to the element, if this node is an element node
    pub fn element_mut(&mut self) -> Option<&mut Element<MSG>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            _ => None,
        }
    }

    /// returns a reference to the element if this is an element node
    pub fn element_ref(&self) -> Option<&Element<MSG>> {
        match *self {
            Node::Element(ref element) => Some(element),
            _ => None,
        }
    }

    /// Consume a mutable self and add a children to this node it if is an element
    /// will have no effect if it is a text node.
    /// This is used in building the nodes in a builder pattern
    pub fn with_children(
        mut self,
        children: impl IntoIterator<Item = Node<MSG>>,
    ) -> Self {
        if let Some(element) = self.element_mut() {
            element.add_children(children);
        } else {
            panic!("Can not add children to a text node");
        }
        self
    }

    /// add children but not consume self
    pub fn add_children(
        &mut self,
        children: impl IntoIterator<Item = Node<MSG>>,
    ) -> Result<(), Error> {
        if let Some(element) = self.element_mut() {
            element.add_children(children);
            Ok(())
        } else {
            Err(Error::AddChildrenNotAllowed)
        }
    }

    /// add attributes to the node and returns itself
    /// this is used in view building
    pub fn with_attributes(
        mut self,
        attributes: impl IntoIterator<Item = Attribute<MSG>>,
    ) -> Self {
        if let Some(elm) = self.element_mut() {
            elm.add_attributes(attributes);
        } else {
            panic!("Can not add attributes to a text node");
        }
        self
    }

    /// add attributes using a mutable reference to self
    pub fn add_attributes(
        &mut self,
        attributes: impl IntoIterator<Item = Attribute<MSG>>,
    ) -> Result<(), Error> {
        if let Some(elm) = self.element_mut() {
            elm.add_attributes(attributes);
            Ok(())
        } else {
            Err(Error::AttributesNotAllowed)
        }
    }

    /// get the attributes of this node
    /// returns None if it is a text node
    pub fn attributes(&self) -> Option<&[Attribute<MSG>]> {
        match *self {
            Node::Element(ref element) => Some(element.attributes()),
            _ => None,
        }
    }

    /// returns the tag of this node if it is an element
    /// otherwise None if it is a text node
    pub fn tag(&self) -> Option<&Tag> {
        if let Some(e) = self.element_ref() {
            Some(&e.tag)
        } else {
            None
        }
    }

    /// return the children of this node if it is an element
    /// returns None if it is a text node
    pub fn children(&self) -> &[Node<MSG>] {
        if let Some(element) = self.element_ref() {
            element.children()
        } else {
            &[]
        }
    }

    /// Return the count of the children of this node
    pub fn children_count(&self) -> usize {
        self.children().len()
    }

    /// return the children of this node if it is an element
    /// returns None if it is a text node
    pub fn children_mut(&mut self) -> Option<&mut [Node<MSG>]> {
        if let Some(element) = self.element_mut() {
            Some(element.children_mut())
        } else {
            None
        }
    }

    /// Removes an child node  from this element and returns it.
    ///
    /// The removed child is replaced by the last child of the element's children.
    ///
    /// # Panics
    /// Panics if this is a text node
    ///
    pub fn swap_remove_child(&mut self, index: usize) -> Node<MSG> {
        match self {
            Node::Element(element) => element.swap_remove_child(index),
            _ => panic!("text has no child"),
        }
    }

    /// Swaps the 2 child node in this element
    ///
    /// # Arguments
    /// * a - The index of the first child node
    /// * b - The index of the second child node
    ///
    /// # Panics
    /// Panics if both `a` and `b` are out of bounds
    /// Panics if this is a text node
    pub fn swap_children(&mut self, a: usize, b: usize) {
        match self {
            Node::Element(element) => element.swap_children(a, b),
            _ => panic!("text has no child"),
        }
    }

    /// Returns the total number of nodes on this node tree, that is counting the direct and
    /// indirect child nodes of this node.
    pub fn node_count(&self) -> usize {
        1 + self.descendant_node_count()
    }

    /// only count the descendant node
    pub fn descendant_node_count(&self) -> usize {
        let mut cnt = 0;
        if let Node::Element(element) = self {
            for child in element.children.iter() {
                cnt += child.node_count();
            }
        }
        cnt
    }

    /// remove the existing attributes and set with the new value
    pub fn set_attributes(
        &mut self,
        attributes: impl IntoIterator<Item = Attribute<MSG>>,
    ) -> Result<(), Error> {
        if let Some(elm) = self.element_mut() {
            elm.set_attributes(attributes);
            Ok(())
        } else {
            Err(Error::AttributesNotAllowed)
        }
    }

    /// merge to existing attributes if the attribute name already exist
    pub fn merge_attributes(
        mut self,
        attributes: impl IntoIterator<Item = Attribute<MSG>>,
    ) -> Self {
        if let Some(elm) = self.element_mut() {
            elm.merge_attributes(attributes);
        }
        self
    }

    /// returh the attribute values of this node which match the attribute name `name`
    pub fn attribute_value(&self, name: &AttributeName) -> Option<Vec<&AttributeValue<MSG>>> {
        if let Some(elm) = self.element_ref() {
            elm.attribute_value(name)
        } else {
            None
        }
    }

}

/// create a virtual node with tag, attrs and children
/// # Example
/// ```rust
/// use sauron::{Node,vdom::element,attr};
///
/// let div:Node<()> = element(
///          "div",
///          vec![attr("class", "container")],
///          vec![],
///      );
/// ```
#[inline]
pub fn element<MSG>(
    tag: Tag,
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
) -> Node<MSG> {
    element_ns(None, tag, attrs, children, false)
}

/// create a virtual node with namespace, tag, attrs and children
/// # Example
/// ```rust
/// use sauron::{Node, vdom::element_ns,attr};
///
/// let svg: Node<()> = element_ns(
///         Some("http://www.w3.org/2000/svg"),
///          "svg",
///          vec![attr("width","400"), attr("height","400")],
///          vec![],
///          false
///      );
/// ```
pub fn element_ns<MSG>(
    namespace: Option<Namespace>,
    tag: Tag,
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
    self_closing: bool,
) -> Node<MSG> {
    Node::Element(Element::new(namespace, tag, attrs, children, self_closing))
}

/// create a leaf node
pub fn leaf<MSG>(leaf: impl Into<Leaf>) -> Node<MSG> {
    Node::Leaf(leaf.into())
}

/// create a node list
pub fn node_list<MSG>(nodes: impl IntoIterator<Item = Node<MSG>>) -> Node<MSG> {
    Node::NodeList(nodes.into_iter().collect())
}

/// create fragment node
pub fn fragment<MSG>(nodes: impl IntoIterator<Item = Node<MSG>>) -> Node<MSG> {
    Node::Fragment(nodes.into_iter().collect())
}
