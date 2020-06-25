//! Provides functions and macros to build svg elements
pub use sauron_vdom::builder::{
    attr,
    element,
    element_ns,
};

pub mod attributes;
pub mod tags;

pub use tags::commons::*;

pub(in crate) const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// creates an svg element with the tag, attributes and children.
/// Example:
/// ```rust,ignore
/// svg_element("circle", vec![cx(1.0), cy(1.0), r(1.0)], vec![]);
/// ```
///
pub fn svg_element<MSG>(
    tag: &'static str,
    attrs: Vec<crate::Attribute<MSG>>,
    children: Vec<crate::Node<MSG>>,
) -> crate::Node<MSG> {
    crate::html::html_element_ns(tag, SVG_NAMESPACE, attrs, children)
}
