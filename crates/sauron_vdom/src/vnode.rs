use std::fmt;
pub mod builder;
pub mod event;
mod value;

use crate::Callback;
pub use event::Event;
pub use value::Value;

pub use attribute::{
    AttribValue,
    Attribute,
};
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
pub enum Node<T, EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
{
    Element(Element<T, EVENT, MSG>),
    Text(Text),
}

impl<T, EVENT, MSG> Node<T, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    pub fn map_msg<F, MSG2>(self, func: F) -> Node<T, EVENT, MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let cb = Callback::from(func);
        self.map_callback(cb)
    }

    /// map_callback the return of the callback from MSG to MSG2
    fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> Node<T, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        match self {
            Node::Element(element) => Node::Element(element.map_callback(cb)),
            Node::Text(text) => Node::Text(Text::new(text.text)),
        }
    }

    fn to_pretty_string(&self, indent: i32) -> String
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
    pub fn take_element(self) -> Option<Element<T, EVENT, MSG>> {
        match self {
            Node::Element(element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Get a mutable reference to the element
    pub fn as_element_mut(&mut self) -> Option<&mut Element<T, EVENT, MSG>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            Node::Text(_) => None,
        }
    }

    pub fn as_element_ref(&self) -> Option<&Element<T, EVENT, MSG>> {
        match *self {
            Node::Element(ref element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Append children to this element
    pub fn add_children(mut self, children: Vec<Node<T, EVENT, MSG>>) -> Self {
        if let Some(element) = self.as_element_mut() {
            element.add_children(children);
        }
        self
    }

    /// add attributes to the node
    pub fn add_attributes(
        mut self,
        attributes: Vec<Attribute<EVENT, MSG>>,
    ) -> Self {
        if let Some(elm) = self.as_element_mut() {
            elm.add_attributes(attributes);
        }
        self
    }

    /// get the attributes of this node
    pub fn get_attributes(&self) -> Vec<Attribute<EVENT, MSG>> {
        match *self {
            Node::Element(ref element) => element.attributes(),
            Node::Text(_) => vec![],
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

impl<T, EVENT, MSG> fmt::Display for Node<T, EVENT, MSG>
where
    T: ToString,
    EVENT: 'static,
    MSG: 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_pretty_string(0))
    }
}
