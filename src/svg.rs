pub use sauron_vdom::builder::{
    attr,
    element,
    element_ns,
};

pub mod attributes;
pub mod tags;

pub use tags::commons::*;

pub(in crate) const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

pub fn svg_element<MSG>(
    tag: &'static str,
    attrs: Vec<crate::Attribute<MSG>>,
    children: Vec<crate::Node<MSG>>,
) -> crate::Node<MSG> {
    crate::html::html_element_ns(tag, SVG_NAMESPACE, attrs, children)
}
