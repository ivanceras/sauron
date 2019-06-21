use std::{
    collections::BTreeMap,
    fmt,
};

pub mod builder;
pub mod event;
mod value;

use crate::Callback;
pub use builder::Attribute;
pub use event::Event;
pub use value::Value;

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
#[derive(Debug, PartialEq, Clone)]
pub enum Node<T, EVENT, MSG> {
    Element(Element<T, EVENT, MSG>),
    Text(Text),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Element<T, EVENT, MSG> {
    pub tag: T,
    pub attrs: BTreeMap<&'static str, Value>,
    pub events: BTreeMap<&'static str, Callback<EVENT, MSG>>,
    pub children: Vec<Node<T, EVENT, MSG>>,
    pub namespace: Option<&'static str>,
}

impl<T, EVENT, MSG> Node<T, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    /// map the return of the callback from MSG to MSG2
    pub fn map<F, MSG2>(self, func: F) -> Node<T, EVENT, MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static + Clone,
    {
        match self {
            Node::Element(element) => Node::Element(element.map(func)),
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
}

impl<T, EVENT, MSG> Element<T, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    /// map the return of the callback from MSG to MSG2
    pub fn map<F, MSG2>(self, func: F) -> Element<T, EVENT, MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static + Clone,
    {
        Element {
            tag: self.tag,
            attrs: self.attrs,
            namespace: self.namespace,
            children: self
                .children
                .into_iter()
                .map(|child| child.map(func.clone()))
                .collect(),
            events: self.events.into_iter().fold(
                BTreeMap::new(),
                |mut acc, (event, cb)| {
                    acc.insert(event, cb.map(func.clone()));
                    acc
                },
            ),
        }
    }

    /// check if the children of this node is only 1 and it is a text node
    fn is_children_a_node_text(&self) -> bool {
        self.children.len() == 1 && self.children[0].is_text_node()
    }

    /// make a pretty string representation of this node
    fn to_pretty_string(&self, indent: i32) -> String
    where
        T: ToString,
    {
        let mut buffer = String::new();
        buffer += &format!("<{}", self.tag.to_string());

        for (attr, value) in self.attrs.iter() {
            buffer += &format!(r#" {}="{}""#, attr, value);
        }
        buffer += ">";

        // do not indent if it is only text child node
        if self.is_children_a_node_text() {
            buffer += &self.children[0].to_pretty_string(indent);
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.children.iter() {
                buffer += &format!(
                    "\n{}{}",
                    padd(indent + 1),
                    child.to_pretty_string(indent + 1)
                );
            }
        }
        // do not make a new line it if is only a text child node or it has no child nodes
        if !(self.is_children_a_node_text() || self.children.is_empty()) {
            buffer += &format!("\n{}", padd(indent));
        }
        buffer += &format!("</{}>", self.tag.to_string());
        buffer
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Text {
    pub text: String,
}

impl<T, EVENT, MSG> Element<T, EVENT, MSG>
where
    T: Clone,
    MSG: Clone,
    EVENT: Clone,
{
    #[inline]
    pub fn new(tag: T) -> Self {
        Self::with_children(tag, [])
    }

    /// Create a Element using the supplied tag name
    #[inline]
    pub fn with_children<C>(tag: T, children: C) -> Self
    where
        C: AsRef<[Node<T, EVENT, MSG>]>,
    {
        Self::with_children_and_maybe_ns(tag, children, None)
    }

    pub fn with_children_and_maybe_ns<C>(
        tag: T,
        children: C,
        ns: Option<&'static str>,
    ) -> Self
    where
        C: AsRef<[Node<T, EVENT, MSG>]>,
    {
        Element {
            tag,
            attrs: BTreeMap::new(),
            events: BTreeMap::new(),
            children: children.as_ref().to_vec(),
            namespace: ns,
        }
    }
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

impl<T, EVENT, MSG> From<Element<T, EVENT, MSG>> for Node<T, EVENT, MSG> {
    fn from(v: Element<T, EVENT, MSG>) -> Self {
        Node::Element(v)
    }
}

/// make a blank string with indented padd
fn padd(n: i32) -> String {
    let mut buffer = String::new();
    for _ in 0..n {
        buffer += "    ";
    }
    buffer
}
