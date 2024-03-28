use crate::dom::document;
use crate::dom::dom_node::intern;
use crate::dom::dom_node::DomInner;
use crate::dom::now;
use crate::dom::Application;
use crate::dom::DomAttr;
use crate::dom::DomAttrValue;
use crate::dom::DomNode;
use crate::dom::GroupedDomAttrValues;
use crate::dom::Program;
use crate::dom::StatelessModel;
use crate::vdom;
use crate::vdom::Attribute;
use crate::vdom::AttributeValue;
use crate::vdom::Leaf;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;

thread_local! {
    static TEMPLATE_LOOKUP: RefCell<HashMap<TypeId, DomNode>> = RefCell::new(HashMap::new());
}

/// if the template is already registered, return the dom template
/// if not, create the dom template and add it
pub fn register_template<MSG>(
    type_id: TypeId,
    parent_node: Option<DomNode>,
    vdom_template: &vdom::Node<MSG>,
) -> DomNode {
    if let Some(template) = lookup_template(type_id) {
        template
    } else {
        let template = create_dom_node_no_listeners(parent_node, &vdom_template);
        add_template(type_id, &template);
        template
    }
}

pub fn add_template(type_id: TypeId, template: &DomNode) {
    TEMPLATE_LOOKUP.with_borrow_mut(|map| {
        if map.contains_key(&type_id) {
            // already added
        } else {
            map.insert(type_id, template.deep_clone());
        }
    })
}

/// lookup for the template
pub fn lookup_template(type_id: TypeId) -> Option<DomNode> {
    TEMPLATE_LOOKUP.with_borrow(|map| {
        if let Some(existing) = map.get(&type_id) {
            // TODO: have to traverse the real children and convert each into DomNode
            Some(existing.deep_clone())
        } else {
            None
        }
    })
}

#[cfg(feature = "with-debug")]
#[derive(Clone, Copy, Default, Debug)]
pub struct Section {
    lookup: f64,
    diffing: f64,
    convert_patch: f64,
    apply_patch: f64,
    total: f64,
    len: usize,
}

#[allow(unused)]
#[cfg(feature = "with-debug")]
impl Section {
    pub fn average(&self) -> Section {
        let div = self.len as f64;
        Section {
            lookup: self.lookup / div,
            diffing: self.diffing / div,
            convert_patch: self.convert_patch / div,
            apply_patch: self.apply_patch / div,
            total: self.total / div,
            len: self.len,
        }
    }

    pub fn percentile(&self) -> Section {
        let div = 100.0 / self.total;
        Section {
            lookup: self.lookup * div,
            diffing: self.diffing * div,
            convert_patch: self.convert_patch * div,
            apply_patch: self.apply_patch * div,
            total: self.total * div,
            len: self.len,
        }
    }
}

#[cfg(feature = "with-debug")]
thread_local!(pub static TIME_SPENT: RefCell<Vec<Section>> = RefCell::new(vec![]));

#[cfg(feature = "with-debug")]
pub fn add_time_trace(section: Section) {
    TIME_SPENT.with_borrow_mut(|v| {
        v.push(section);
    })
}

#[allow(unused)]
#[cfg(feature = "with-debug")]
fn total(values: &[Section]) -> Section {
    let len = values.len();
    let mut sum = Section::default();
    for v in values.iter() {
        sum.lookup += v.lookup;
        sum.diffing += v.diffing;
        sum.convert_patch += v.convert_patch;
        sum.apply_patch += v.apply_patch;
        sum.total += v.total;
        sum.len = len;
    }
    sum
}

#[allow(unused)]
#[cfg(feature = "with-debug")]
pub fn total_time_spent() -> Section {
    TIME_SPENT.with_borrow(|values| total(values))
}

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
    let children = nodes
        .into_iter()
        .map(|node| create_dom_node_no_listeners(Some(dom_node.clone()), &node));
    dom_node.append_children(children);
    dom_node
}

fn create_leaf_node_no_listeners<MSG>(parent_node: Option<DomNode>, leaf: &Leaf<MSG>) -> DomNode {
    match leaf {
        Leaf::Text(txt) => DomNode {
            inner: DomInner::Text(RefCell::new(document().create_text_node(txt))),
            parent: Rc::new(RefCell::new(parent_node)),
        },
        Leaf::Symbol(symbol) => DomNode {
            inner: DomInner::Symbol(Rc::new(RefCell::new(symbol.clone()))),
            parent: Rc::new(RefCell::new(parent_node)),
        },
        Leaf::Comment(comment) => DomNode {
            inner: DomInner::Comment(document().create_comment(comment)),
            parent: Rc::new(RefCell::new(parent_node)),
        },
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
        .map(|a| convert_attr_except_listener(a))
        .collect::<Vec<_>>();

    for att in attrs {
        set_element_dom_attr_except_listeners(&element, att);
    }

    let dom_node = DomNode {
        inner: DomInner::Element {
            element,
            listeners: Rc::new(RefCell::new(None)),
            children: Rc::new(RefCell::new(vec![])),
        },
        parent: Rc::new(RefCell::new(parent_node)),
    };
    let children = elm
        .children()
        .iter()
        .map(|child| create_dom_node_no_listeners(Some(dom_node.clone()), child));
    dom_node.append_children(children);
    dom_node
}

pub(crate) fn convert_attr_except_listener<MSG>(attr: &Attribute<MSG>) -> DomAttr {
    DomAttr {
        namespace: attr.namespace,
        name: attr.name,
        value: attr
            .value
            .iter()
            .filter_map(|v| convert_attr_value_except_listener(v))
            .collect(),
    }
}

/// Note: Used only templates
/// set the lement with dom attr except for the event listeners
pub(crate) fn set_element_dom_attr_except_listeners(element: &web_sys::Element, attr: DomAttr) {
    let attr_name = intern(attr.name);
    let attr_namespace = attr.namespace;

    let GroupedDomAttrValues {
        listeners: _,
        plain_values,
        styles,
    } = attr.group_values();
    DomAttr::set_element_style(element, attr_name, styles);
    DomAttr::set_element_simple_values(element, attr_name, attr_namespace, plain_values);
}

fn convert_attr_value_except_listener<MSG>(
    attr_value: &AttributeValue<MSG>,
) -> Option<DomAttrValue> {
    match attr_value {
        AttributeValue::Simple(v) => Some(DomAttrValue::Simple(v.clone())),
        AttributeValue::Style(v) => Some(DomAttrValue::Style(v.clone())),
        AttributeValue::EventListener(_v) => None,
        AttributeValue::Empty => None,
    }
}

impl<APP> Program<APP>
where
    APP: Application,
{
    pub(crate) fn create_stateless_component_with_template(
        &self,
        parent_node: Option<DomNode>,
        comp: &StatelessModel<APP::MSG>,
    ) -> DomNode {
        #[cfg(feature = "with-debug")]
        let t1 = now();
        let comp_view = &comp.view;
        let vdom_template = comp_view.template();
        #[cfg(feature = "with-debug")]
        let t2 = now();
        let skip_diff = comp_view.skip_diff();
        match (vdom_template, skip_diff) {
            (Some(vdom_template), Some(skip_diff)) => {
                //TODO: something is wrong with the chain of elements here
                //from base node to it's children
                // disabling template for stateless component for now
                let template = register_template(comp.type_id, parent_node, &vdom_template);
                //log::info!("template: {}", template.render_to_string());
                let real_comp_view = comp_view.unwrap_template_ref();
                let patches =
                    self.create_patches_with_skip_diff(&vdom_template, &real_comp_view, &skip_diff);
                //log::info!("stateless component patches: {:#?}", patches);
                #[cfg(feature = "with-debug")]
                let t3 = now();
                let dom_patches = self
                    .convert_patches(&template, &patches)
                    .expect("convert patches");
                //log::info!("dom patches: {:#?}", dom_patches);
                #[cfg(feature = "with-debug")]
                let t4 = now();
                self.apply_dom_patches(dom_patches).expect("patch template");
                #[cfg(feature = "with-debug")]
                let t5 = now();

                #[cfg(feature = "with-debug")]
                add_time_trace(Section {
                    lookup: t2 - t1,
                    diffing: t3 - t2,
                    convert_patch: t4 - t3,
                    apply_patch: t5 - t4,
                    total: t5 - t1,
                    ..Default::default()
                });
                //log::info!("the patched template is now: {:#?}", template);
                //log::info!("the patched template is now: {}", template.render_to_string());
                template
            }
            _ => unreachable!("should have template and skip_diff"),
        }
    }

    pub(crate) fn create_initial_view_with_template(&self) -> DomNode {
        log::info!("Creating initial view..");
        let app_view = self.app_context.app.borrow().view();
        let vdom_template = app_view.template();
        let skip_diff = app_view.skip_diff();
        let real_view = app_view.unwrap_template();
        match (vdom_template, skip_diff) {
            (Some(vdom_template), Some(skip_diff)) => {
                let patches =
                    self.create_patches_with_skip_diff(&vdom_template, &real_view, &skip_diff);
                let type_id = TypeId::of::<APP>();
                let dom_template = register_template(type_id, None, &vdom_template);
                let dom_patches = self
                    .convert_patches(&dom_template, &patches)
                    .expect("convert patches");
                let _new_template_node = self
                    .apply_dom_patches(dom_patches)
                    .expect("template patching");
                //Self::dispatch_mount_event_to_children(&dom_template, 2, 0);
                dom_template
            }
            _ => unreachable!("must have a template and skip_diff"),
        }
    }
}

impl DomNode {
    pub(crate) fn deep_clone(&self) -> DomNode {
        DomNode {
            inner: self.inner.deep_clone(),
            parent: self.parent.clone(),
        }
    }
}

impl DomInner {
    fn deep_clone(&self) -> Self {
        match self {
            Self::Element {
                element,
                listeners,
                children,
            } => {
                let element = element.clone_node_with_deep(true).expect("deep_clone");
                Self::Element {
                    element: element.unchecked_into(),
                    listeners: listeners.clone(),
                    children: Rc::new(RefCell::new(
                        children.borrow().iter().map(|c| c.deep_clone()).collect(),
                    )),
                }
            }
            Self::Text(text_node) => {
                let text_node = text_node
                    .borrow()
                    .clone_node_with_deep(true)
                    .expect("deep_clone");
                Self::Text(RefCell::new(text_node.unchecked_into()))
            }
            Self::Symbol(symbol) => Self::Text(Rc::new(RefCell::new(symbol.clone()))),
            Self::Comment(comment_node) => {
                let comment_node = comment_node.clone_node_with_deep(true).expect("deep_clone");
                Self::Comment(comment_node.unchecked_into())
            }
            Self::Fragment { fragment, children } => {
                let fragment = fragment.clone_node_with_deep(true).expect("deep_clone");
                Self::Fragment {
                    fragment: fragment.unchecked_into(),
                    children: Rc::new(RefCell::new(
                        children.borrow().iter().map(|c| c.deep_clone()).collect(),
                    )),
                }
            }
            Self::StatefulComponent(_) => unreachable!("can not deep clone stateful component"),
        }
    }
}
