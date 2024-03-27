//! utility functions for extracting templates from a view
//!
use crate::dom::document;
use crate::dom::dom_node::DomInner;
use crate::dom::DomAttr;
use crate::dom::DomNode;
use crate::vdom;
use crate::vdom::Attribute;
use crate::vdom::Leaf;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::intern;

pub(crate) fn create_dom_node_no_listeners<MSG>(
    parent_node: Option<DomNode>,
    vnode: &vdom::Node<MSG>,
) -> DomNode {
    match vnode {
        vdom::Node::Element(element_node) => {
            create_element_node_no_listeners(parent_node, element_node)
        }
        vdom::Node::Leaf(leaf_node) => create_leaf_node_no_listeners(parent_node, leaf_node),
    }
}

fn create_fragment_node_no_listeners<MSG>(
    parent_node: Option<DomNode>,
    nodes: &[vdom::Node<MSG>],
) -> DomNode {
    let fragment = document().create_document_fragment();
    let dom_node = DomNode {
        inner: DomInner::Fragment {
            fragment,
            children: Rc::new(RefCell::new(vec![])),
        },
        parent: Rc::new(RefCell::new(parent_node)),
    };
    let children: Vec<DomNode> = nodes
        .into_iter()
        .map(|node| create_dom_node_no_listeners(Some(dom_node.clone()), &node))
        .collect();
    for child in children.into_iter() {
        dom_node.append_child(child).expect("append child");
    }
    dom_node
}

fn create_leaf_node_no_listeners<MSG>(parent_node: Option<DomNode>, leaf: &Leaf<MSG>) -> DomNode {
    match leaf {
        Leaf::Text(txt) => DomNode {
            inner: DomInner::Text(RefCell::new(document().create_text_node(txt))),
            parent: Rc::new(RefCell::new(parent_node)),
        },

        Leaf::Comment(comment) => DomNode {
            inner: DomInner::Comment(document().create_comment(comment)),
            parent: Rc::new(RefCell::new(parent_node)),
        },

        Leaf::SafeHtml(_safe_html) => {
            panic!("safe html must have already been dealt in create_element node");
        }
        Leaf::DocType(_doctype) => {
            panic!(
                "It looks like you are using doctype in the middle of an app,
                    doctype is only used in rendering"
            );
        }
        Leaf::Fragment(nodes) => create_fragment_node_no_listeners(parent_node, nodes),
        // NodeList that goes here is only possible when it is the root_node,
        // since node_list as children will be unrolled into as child_elements of the parent
        // We need to wrap this node_list into doc_fragment since root_node is only 1 element
        Leaf::NodeList(node_list) => create_fragment_node_no_listeners(parent_node, node_list),
        Leaf::StatefulComponent(_lc) => {
            unreachable!("Component should not be created here")
        }
        Leaf::StatelessComponent(_comp) => {
            unreachable!("stateless component should not be here")
        }
        Leaf::TemplatedView(_) => todo!(),
    }
}

fn create_element_node_no_listeners<MSG>(
    parent_node: Option<DomNode>,
    elm: &vdom::Element<MSG>,
) -> DomNode {
    let document = document();
    let element = if let Some(namespace) = elm.namespace() {
        document
            .create_element_ns(Some(intern(namespace)), intern(elm.tag()))
            .expect("Unable to create element")
    } else {
        document
            .create_element(intern(elm.tag()))
            .expect("create element")
    };
    // TODO: dispatch the mount event recursively after the dom node is mounted into
    // the root node
    let attrs = Attribute::merge_attributes_of_same_name(elm.attributes().iter());
    let attrs = attrs
        .iter()
        .map(|a| DomAttr::convert_attr_except_listener(a))
        .collect::<Vec<_>>();

    for att in attrs {
        DomAttr::set_element_dom_attr_except_listeners(&element, att);
    }

    let dom_node = DomNode {
        inner: DomInner::Element {
            element,
            listeners: Rc::new(RefCell::new(None)),
            children: Rc::new(RefCell::new(vec![])),
        },
        parent: Rc::new(RefCell::new(parent_node)),
    };
    let children: Vec<DomNode> = elm
        .children()
        .iter()
        .map(|child| create_dom_node_no_listeners(Some(dom_node.clone()), child))
        .collect();
    for child in children.into_iter() {
        dom_node.append_child(child).expect("append child");
    }
    dom_node
}
