use crate::vdom;
use crate::vdom::map_msg::AttributeMapMsg;

/// extract the template from a node
pub(crate) fn extract_template<MSG>(node: &vdom::Node<MSG>) -> vdom::Node<MSG> where MSG: 'static{

    match node {
        vdom::Node::Element(elm) => mt_dom::element_ns(
            elm.namespace,
            elm.tag,
            elm.attributes()
                .iter()
                .filter(|att| att.is_static_str())
                .cloned(),
            elm.children().iter().map(extract_template),
            elm.self_closing,
        ),
        vdom::Node::Fragment(nodes) => vdom::Node::Fragment(
            nodes
                .iter()
                .map(|node| extract_template(node))
                .collect(),
        ),
        vdom::Node::Leaf(leaf) => vdom::Node::Leaf(leaf.clone()),
        vdom::Node::NodeList(_node_list) => unreachable!("This has been unrolled"),
    }
}
