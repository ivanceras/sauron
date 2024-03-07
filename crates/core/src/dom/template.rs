use crate::vdom;
use crate::vdom::{Leaf, Node};
 use web_sys::Element;
use crate::dom::DomAttr;
use wasm_bindgen::intern;
use crate::dom::GroupedDomAttrValues;
use crate::vdom::Attribute;
use crate::dom::document;
use crate::dom::dom_node;
use wasm_bindgen::JsCast;

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

pub fn build_template<MSG>(node: &vdom::Node<MSG>) -> web_sys::Node{
    create_dom_node_without_listeners(node)
}

pub fn create_dom_node_without_listeners<MSG>(vnode: &vdom::Node<MSG>) -> web_sys::Node {
    match vnode {
        vdom::Node::Leaf(leaf_node) => create_leaf_node_without_listeners(leaf_node),
        vdom::Node::Element(element_node) => {
            let created_node = create_element_node_without_listeners(element_node);
            for child in element_node.children().iter() {
                let child_node = create_dom_node_without_listeners(child);
                created_node.append_child(&child_node).expect("append child");
            }
            created_node
        }
        vdom::Node::Fragment(nodes) => create_document_fragment(nodes),
        // NodeList that goes here is only possible when it is the root_node,
        // since node_list as children will be unrolled into as child_elements of the parent
        // We need to wrap this node_list into doc_fragment since root_node is only 1 element
        vdom::Node::NodeList(node_list) => create_document_fragment(node_list),
    }
}

    fn create_document_fragment<MSG>(nodes: &[vdom::Node<MSG>]) -> web_sys::Node {
        let doc_fragment = document().create_document_fragment();
        for vnode in nodes {
            let created_node = create_dom_node_without_listeners(vnode);
            doc_fragment.append_child(&created_node).expect("append child");
        }
        doc_fragment.into()
    }

    fn create_leaf_node_without_listeners<MSG>(leaf: &Leaf<MSG>) ->web_sys::Node {
        match leaf {
            Leaf::Text(txt) => dom_node::create_text_node(txt).into(),
            Leaf::Comment(comment) => document().create_comment(comment).into(),
            Leaf::SafeHtml(_safe_html) => {
                panic!("safe html must have already been dealt in create_element node");
            }
            Leaf::DocType(_doctype) => {
                panic!(
                    "It looks like you are using doctype in the middle of an app,
                    doctype is only used in rendering"
                );
            }
            Leaf::Component {
                type_id,
                comp,
                attrs,
                children,
            } => {
                let template = comp.template();
                log::info!("template: {:?}", template);
                // The program needs to have a registry of Component
                // indexed by their type_id
                let comp_node = create_dom_node_without_listeners::<MSG>(&crate::html::div(
                    [crate::html::attributes::class("component")],
                    [],
                ));
                for child in children.iter() {
                    let child_dom = create_dom_node_without_listeners(&child);
                    comp_node
                        .append_child(&child_dom)
                        .expect("must append child node of component");
                }
                comp_node
            }
        }
    }


fn create_element_node_without_listeners<MSG>(velem: &vdom::Element<MSG>) -> web_sys::Node {
    let document = document();

    let element = if let Some(namespace) = velem.namespace() {
        document
            .create_element_ns(Some(intern(namespace)), intern(velem.tag()))
            .expect("Unable to create element")
    } else {
        dom_node::create_element(velem.tag())
    };

    let attrs = Attribute::merge_attributes_of_same_name(velem.attributes());

        let attrs = attrs
            .iter()
            .map(|a| DomAttr::convert_attr_except_listener(a))
            .collect::<Vec<_>>();

    for att in attrs{
        DomAttr::set_element_dom_attr_except_listeners(&element, att);
    }

    element.into()
}
