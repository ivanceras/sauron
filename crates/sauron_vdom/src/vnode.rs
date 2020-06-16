use std::fmt;
pub mod builder;
pub mod event;
mod value;

use crate::Callback;
pub use event::Event;
pub use value::Value;

pub use attribute::{AttribValue, Attribute};
pub use element::Element;

mod attribute;
mod element;

/// This is the core data structure of the library.
/// Any tree can be represented by `Node`.
/// The `T` is generic instead of just using plain `&'static str`
/// in order for this library to be used not only in html based widget
/// but can also be used to represent native GUI widgets
/// in various platforms.
///
/// Note: Clone is necessary for the aesthetics in the construction of node through series of function
/// calls.
/// Without Clone, the user code would look like these:
/// ```ignore
///     div(&[class("some-class"), &[text("Some text")])
/// ```
/// as compared to
/// ```ignore
///     div([class("some-class"), [text("some text)])
/// ```
/// Cloning is only done once, and happens when constructing the views into a node tree.
/// Cloning also allows flexibility such as adding more children into an existing node/element.
#[derive(Debug, Clone, PartialEq)]
pub enum Node<T, ATT, EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
    ATT: Clone,
{
    Element(Element<T, ATT, EVENT, MSG>),
    Text(Text),
}

impl<T, ATT, EVENT, MSG> Node<T, ATT, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
    ATT: PartialEq + Ord + ToString + Clone,
{
    pub fn map_msg<F, MSG2>(self, func: F) -> Node<T, ATT, EVENT, MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let cb = Callback::from(func);
        self.map_callback(cb)
    }

    /// map_callback the return of the callback from MSG to MSG2
    fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Node<T, ATT, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        match self {
            Node::Element(element) => Node::Element(element.map_callback(cb)),
            Node::Text(text) => Node::Text(Text::new(text.text)),
        }
    }

    fn to_pretty_string(&self, indent: usize) -> String
    where
        T: ToString,
    {
        match self {
            Node::Element(element) => element.to_pretty_string(indent),
            Node::Text(text) => format!("{}", text),
        }
    }

    fn is_text_node(&self) -> bool {
        match self {
            Node::Element(_) => false,
            Node::Text(_) => true,
        }
    }

    /// consume the element
    pub fn take_element(self) -> Option<Element<T, ATT, EVENT, MSG>> {
        match self {
            Node::Element(element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Get a mutable reference to the element
    pub fn as_element_mut(
        &mut self,
    ) -> Option<&mut Element<T, ATT, EVENT, MSG>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            Node::Text(_) => None,
        }
    }

    pub fn as_element_ref(&self) -> Option<&Element<T, ATT, EVENT, MSG>> {
        match *self {
            Node::Element(ref element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Append children to this element
    pub fn add_children(
        mut self,
        children: Vec<Node<T, ATT, EVENT, MSG>>,
    ) -> Self {
        if let Some(element) = self.as_element_mut() {
            element.add_children(children);
        }
        self
    }

    /// add attributes to the node
    pub fn add_attributes(
        mut self,
        attributes: Vec<Attribute<ATT, EVENT, MSG>>,
    ) -> Self {
        if let Some(elm) = self.as_element_mut() {
            elm.add_attributes(attributes);
        }
        self
    }

    /// get the attributes of this node
    pub fn get_attributes(&self) -> Vec<Attribute<ATT, EVENT, MSG>> {
        match *self {
            Node::Element(ref element) => element.attributes(),
            Node::Text(_) => vec![],
        }
    }

    /// returns the tag of this node if it is an element
    pub fn tag(&self) -> Option<&T> {
        if let Some(e) = self.as_element_ref() {
            Some(&e.tag)
        } else {
            None
        }
    }

    /// returns the text content if it is a text node
    pub fn text(&self) -> Option<&str> {
        match self {
            Node::Text(text) => Some(&text.text),
            Node::Element(_) => None,
        }
    }

    /// returns the text if this node has only one child and is a text.
    /// includes: h1, h2..h6, p,
    pub fn eldest_child_text(&self) -> Option<&str> {
        if let Some(element) = self.as_element_ref() {
            element.eldest_child_text()
        } else {
            None
        }
    }

    pub fn get_children(&self) -> Option<&[Node<T, ATT, EVENT, MSG>]> {
        if let Some(element) = self.as_element_ref() {
            Some(element.get_children())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Text {
    pub text: String,
}

impl Text {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Text { text: s.into() }
    }
}

// Turn a Text into an HTML string
impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl<T, ATT, EVENT, MSG> fmt::Display for Node<T, ATT, EVENT, MSG>
where
    T: ToString,
    EVENT: 'static,
    MSG: 'static,
    ATT: PartialEq + Ord + ToString + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_pretty_string(0))
    }
}
