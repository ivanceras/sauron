use crate::vdom;
use crate::vdom::{Leaf, Node};

/// extract the template from a node
pub(crate) fn extract_template<MSG>(node: &Node<MSG>) -> vdom::Node<MSG>
where
    MSG: 'static,
{
    match node {
        Node::Element(elm) => vdom::element_ns(
            elm.namespace,
            elm.tag,
            elm.attributes()
                .iter()
                .filter(|att| att.is_static_str())
                .cloned(),
            elm.children().iter().map(extract_template),
            elm.self_closing,
        ),
        Node::Fragment(nodes) => {
            Node::Fragment(nodes.iter().map(|node| extract_template(node)).collect())
        }
        Node::Leaf(leaf) => {
            if leaf.is_static_str() {
                Node::Leaf(leaf.clone())
            } else {
                match leaf {
                    Leaf::Text(_) => Node::Leaf(Leaf::Text("".into())),
                    Leaf::SafeHtml(_) => Node::Leaf(Leaf::SafeHtml("".into())),
                    Leaf::Comment(_) => Node::Leaf(Leaf::Comment("".into())),
                    Leaf::DocType(_) => Node::Leaf(Leaf::DocType("".into())),
                    Leaf::Component{..} => {
                        //TODO: we save the template to a template registry
                        todo!()
                    }
                }
            }
        }
        Node::NodeList(_node_list) => unreachable!("This has been unrolled"),
    }
}
