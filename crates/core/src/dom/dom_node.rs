#![allow(unused)]
use crate::dom::component::register_template;
use crate::dom::component::StatelessModel;
#[cfg(feature = "with-debug")]
use crate::dom::now;
use crate::dom::program::EventClosures;
use crate::dom::DomAttr;
use crate::dom::GroupedDomAttrValues;
use crate::dom::StatefulComponent;
use crate::dom::StatefulModel;
use crate::html::lookup;
use crate::vdom::AttributeName;
use crate::vdom::TreePath;
use crate::{
    dom::document,
    dom::events::MountEvent,
    dom::{Application, Program},
    vdom,
    vdom::{Attribute, Leaf},
};
use indexmap::IndexMap;
use js_sys::Function;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{self, Element, Node, Text};
use std::cell::Ref;

/// data attribute name used in assigning the node id of an element with events
pub(crate) const DATA_VDOM_ID: &str = "data-vdom-id";

pub(crate) type EventClosure = Closure<dyn FnMut(web_sys::Event)>;
pub type NamedEventClosures = IndexMap<&'static str, EventClosure>;

thread_local!(static NODE_ID_COUNTER: Cell<usize> = Cell::new(1));

#[allow(unused)]
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

/// A counter part of the vdom Node
/// This is needed, so that we can
/// 1. Keep track of event closure and drop them when nodes has been removed
/// 2. Custom removal of children nodes on a stateful component
///
#[derive(Clone)]
pub struct DomNode {
    inner: DomInner,
    //TODO: don't really need to have reference to the parent
    //as RemoveNode patch can just be called with Node::remove
    //though remove doesn't return the node to be removed
    //
    //MoveAfterNode and  with insertAdjacentElement
    parent: Rc<RefCell<Option<DomNode>>>,
}
#[derive(Clone)]
pub enum DomInner {
    /// a reference to an element node
    Element {
        /// the reference to the actual element
        element: web_sys::Element,
        /// the listeners of this element, which we will drop when this element is removed
        listeners: Rc<Option<NamedEventClosures>>,
        /// keeps track of the children nodes
        /// this needs to be synced with the actual element children
        children: Rc<RefCell<Vec<DomNode>>>,
    },
    /// text node
    Text(RefCell<web_sys::Text>),
    /// comment node
    Comment(web_sys::Comment),
    /// Fragment node
    Fragment {
        ///
        fragment: web_sys::DocumentFragment,
        ///
        children: Rc<RefCell<Vec<DomNode>>>,
    },
    /// StatefulComponent
    StatefulComponent(Rc<RefCell<dyn StatefulComponent>>),
}

impl From<web_sys::Node> for DomNode{

    fn from(node: web_sys::Node) -> Self {
        let element: web_sys::Element = node.dyn_into().expect("must be an element");
        DomNode{
            inner: DomInner::Element{element, 
                listeners: Rc::new(None),
                children: Rc::new(RefCell::new(vec![])),
            },
            parent: Rc::new(RefCell::new(None)),
        }
    }
}

impl DomNode{

    fn children(&self) -> Option<Ref<'_, Vec<DomNode>>>{
        match &self.inner{
            DomInner::Element{children,..} => Some(children.borrow()),
            DomInner::Fragment{children,..} => Some(children.borrow()),
            _ => None,
        }
    }

    pub(crate) fn tag(&self) -> Option<String>{
        match &self.inner{
            DomInner::Element{element,..} => Some(element.tag_name().to_lowercase()),
            _ => None
        }
    }

    fn as_node(&self) -> web_sys::Node{
        match &self.inner{
            DomInner::Element{element,..} => element.clone().unchecked_into(),
            DomInner::Fragment{fragment,..} => fragment.clone().unchecked_into(),
            DomInner::Text(text_node) => text_node.borrow().clone().unchecked_into(),
            DomInner::Comment(comment_node) => comment_node.clone().unchecked_into(),
            DomInner::StatefulComponent(_) => todo!("for stateful component.."),
        }
    }

    fn as_element(&self) -> web_sys::Element{
        match &self.inner{
            DomInner::Element{element,..} => element.clone().unchecked_into(),
            DomInner::Fragment{fragment,..} => fragment.clone().unchecked_into(),
            DomInner::Text(text_node) => text_node.borrow().clone().unchecked_into(),
            DomInner::Comment(comment_node) => comment_node.clone().unchecked_into(),
            DomInner::StatefulComponent(_) => todo!("for stateful component.."),
        }
    }


    fn set_parent(&self, parent_node: &DomNode){
        *self.parent.borrow_mut() = Some(parent_node.clone());
    }

    pub(crate) fn append_child(&self, child: DomNode) -> Result<(), JsValue> {
        match &self.inner{
            DomInner::Element{element,children,..} => {
                log::info!("appending to {}", self.render_to_string());
                log::info!("child: {}", child.render_to_string());
                element.append_child(&child.as_node()).expect("append child");
                children.borrow_mut().push(child.clone());
                Ok(())
            }
            _ => unreachable!("appending should only be called to Element node"),
        }
    }

    pub(crate) fn insert_before(&self, for_insert: DomNode) -> Result<Option<DomNode>, JsValue> {
        let DomInner::Element{element: target_element,..} = &self.inner else {
            unreachable!("target element should be an element");
        };
        let parent_target = self.parent.borrow();
        let DomInner::Element{element: parent_element,..} = &parent_target.as_ref().expect("must have a parent").inner else {
            unreachable!("parent must be an element");
        };
        let DomInner::Element{element: for_insert,..} = &for_insert.inner else{
            unreachable!("for insert must be an element");
        };
        parent_element
            .insert_before(&for_insert, Some(&target_element))
            .expect("must remove target node");
        Ok(None)
    }

    pub(crate) fn insert_after(&self, for_insert: &DomNode) -> Result<Option<DomNode>, JsValue> {
        let target_element = match &self.inner{
            DomInner::Element{element,..} => element,
            _ => unreachable!("target element should be an element"),
        };
        match &for_insert.inner{
            DomInner::Element{element,..} => {
                target_element
                    .insert_adjacent_element(intern("afterend"), &element)?;
                Ok(None)
            }
            _ => unreachable!("unexpected variant to be inserted after.."),
        }
    }

    pub(crate) fn replace_child(&self, child: &DomNode, replacement: DomNode) -> Option<DomNode>{
        log::debug!("atttempt to remove child..");
        match &self.inner{
            DomInner::Element{element,children,..} => {
                let mut child_index = None;
                for (i,c) in children.borrow().iter().enumerate(){
                    if c.as_node() == child.as_node(){
                        log::info!("This is the child to be removed at: {}", i);
                        child_index = Some(i);
                    }
                }
                if let Some(child_index) = child_index{
                    let child = children.borrow_mut().remove(child_index);
                    child.as_element().replace_with_with_node_1(&replacement.as_node()).expect("must replace child");
                    replacement.set_parent(self);
                    children.borrow_mut().insert(child_index, replacement);
                    Some(child)
                }else{
                    log::info!("can not find child to be removed...");
                    //None
                    unreachable!("must find the child...");
                }
            }
            _ => todo!(),
        }
    }

    pub(crate) fn replace_node(&self, mut replacement: Vec<DomNode>) -> Result<Option<DomNode>, JsValue> {
        let first_node = replacement.pop().expect("must have a first node");
        log::info!("replacing with {}", first_node.render_to_string());
        match &self.inner{
            DomInner::Text(text_node) => {
                log::info!("replacing text node...");
                //*text_node.borrow_mut() = first_node.as_node().unchecked_into();
                if let Some(parent) = self.parent.borrow().as_ref(){
                    parent.replace_child(self, first_node);
                }else{
                    log::info!("no parent for {}", self.render_to_string());
                    unreachable!("There should be parent of this...");
                }
            }
            _ => todo!(),
        }
        log::info!("self is now: {}", self.render_to_string());
        for node in replacement.into_iter() {
            log::info!("inserting the rest..");
            self.insert_before(node)?;
        }
        Ok(None)
    }

    /// clones this DomNode
    pub(crate) fn deep_clone(&self) -> Result<DomNode, JsValue>{
        todo!()
    }
}

impl fmt::Debug for DomNode{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DomNode")
    }
}

impl<APP> Program<APP>
where
    APP: Application + 'static,
{
    fn into_dom_node(&self, parent_node: Option<DomNode>, node: &vdom::Node<APP::MSG>) -> DomNode {
        match node {
            vdom::Node::Element(elm) => {
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


                let listeners = self.set_element_dom_attrs(
                    &element,
                    attrs
                        .iter()
                        .map(|a| self.convert_attr(a))
                        .collect::<Vec<_>>(),
                );
                let dom_node = DomNode{
                    inner: DomInner::Element {
                        element,
                        listeners: Rc::new(listeners),
                        children: Rc::new(RefCell::new(vec![])),
                    },
                    parent: Rc::new(RefCell::new(parent_node)),
                };
                let children: Vec<DomNode> = elm
                    .children()
                    .iter()
                    .map(|child| self.into_dom_node(Some(dom_node.clone()), child))
                    .collect();
                for child in children.into_iter(){
                    dom_node.append_child(child);
                }
                dom_node
            }
            vdom::Node::Leaf(leaf) => match leaf {
                Leaf::Text(txt) => {
                    DomNode{
                        inner: DomInner::Text(RefCell::new(document().create_text_node(txt))),
                        parent: Rc::new(RefCell::new(parent_node)),
                    }
                }
                Leaf::Comment(comment) => {
                    DomNode{
                        inner: DomInner::Comment(document().create_comment(comment)),
                        parent: Rc::new(RefCell::new(parent_node)),
                    }
                }
                Leaf::Fragment(nodes) => {
                    let fragment = document().create_document_fragment();
                    let dom_node = DomNode{
                        inner: DomInner::Fragment { fragment, children: Rc::new(RefCell::new(vec![])) },
                        parent: Rc::new(RefCell::new(parent_node)),
                    };
                    let children:Vec<DomNode> = nodes.iter().map(|node| self.into_dom_node(Some(dom_node.clone()), node)).collect();
                    for child in children.into_iter(){
                        dom_node.append_child(child);
                    }
                    dom_node
                }
                // NodeList that goes here is only possible when it is the root_node,
                // since node_list as children will be unrolled into as child_elements of the parent
                // We need to wrap this node_list into doc_fragment since root_node is only 1 element
                Leaf::NodeList(nodes) => {
                    let fragment = document().create_document_fragment();
                    let dom_node = DomNode{
                        inner: DomInner::Fragment { fragment, children: Rc::new(RefCell::new(vec![])) },
                        parent: Rc::new(RefCell::new(parent_node)),
                    };
                    let children:Vec<DomNode> = nodes.iter().map(|node| self.into_dom_node(Some(dom_node.clone()), node)).collect();
                    for child in children.into_iter(){
                        dom_node.append_child(child);
                    }
                    dom_node
                }
                Leaf::StatefulComponent(comp) => todo!(),
                Leaf::StatelessComponent(comp) => {
                    todo!("just like element")
                }
                Leaf::TemplatedView(view) => {
                    unreachable!("template view should not be created: {:#?}", view)
                }
                Leaf::SafeHtml(_) => unreachable!("must be converted throught html parse already"),
                Leaf::DocType(_) => unreachable!("doc type is never converted"),
            },
        }
    }
}

impl DomNode {
    fn inner_html(&self) -> Option<String> {
        match &self.inner {
            DomInner::Element { element, .. } => Some(element.inner_html()),
            _ => None,
        }
    }

    pub(crate) fn create_text_node(txt: &str) -> Text {
        document().create_text_node(txt)
    }

    /// create dom element from tag name
    pub(crate) fn create_element(tag: &'static str) -> web_sys::Element {
        document()
            .create_element(intern(tag))
            .expect("create element")
    }

    /// create a web_sys::Element with the specified tag
    fn create_element_with_tag(tag: &'static str) -> (&'static str, web_sys::Element) {
        let elm = document().create_element(intern(tag)).unwrap();
        (tag, elm)
    }

    pub(crate) fn render_to_string(&self) -> String {
        let mut buffer = String::new();
        self.render(&mut buffer).expect("must render");
        buffer
    }

    fn render(&self, buffer: &mut dyn fmt::Write) -> fmt::Result {
        match &self.inner {
            DomInner::Text(text_node) => {
                let text = text_node.borrow().whole_text().expect("whole text");
                write!(buffer, "{text}")?;
                Ok(())
            }
            DomInner::Element {
                element, children, ..
            } => {
                let tag = element.tag_name().to_lowercase();

                write!(buffer, "<{tag}")?;
                let attrs = element.attributes();
                let attrs_len = attrs.length();
                for i in 0..attrs_len {
                    let attr = attrs.item(i).expect("attr");
                    write!(buffer, " {}=\"{}\"", attr.local_name(), attr.value())?;
                }
                if lookup::is_self_closing(&tag) {
                    write!(buffer, "/>")?;
                } else {
                    write!(buffer, ">")?;
                }

                for child in children.borrow().iter() {
                    child.render(buffer)?;
                }
                if !lookup::is_self_closing(&tag) {
                    write!(buffer, "</{tag}>")?;
                }
                Ok(())
            }
            _ => todo!("for other else"),
        }
    }
}

#[cfg(feature = "with-interning")]
#[inline(always)]
pub fn intern(s: &str) -> &str {
    wasm_bindgen::intern(s)
}

#[cfg(not(feature = "with-interning"))]
#[inline(always)]
pub fn intern(s: &str) -> &str {
    s
}

/// This is the value of the data-sauron-vdom-id.
/// Used to uniquely identify elements that contain closures so that the DomUpdater can
/// look them up by their unique id.
/// When the DomUpdater sees that the element no longer exists it will drop all of it's
/// Rc'd Closures for those events.
fn create_unique_identifier() -> usize {
    NODE_ID_COUNTER.with(|x| {
        let val = x.get();
        x.set(val + 1);
        val
    })
}

/// A node along with all of the closures that were created for that
/// node's events and all of it's child node's events.
impl<APP> Program<APP>
where
    APP: Application,
{
    /*
    fn create_document_fragment(&self, nodes: &[vdom::Node<APP::MSG>]) -> Node {
        let doc_fragment = document().create_document_fragment();
        for vnode in nodes {
            let created_node = self.create_dom_node(vnode);
            Self::append_child_and_dispatch_mount_event(&doc_fragment, &created_node)
        }
        doc_fragment.into()
    }
    */

    /*
    fn create_leaf_node(&self, leaf: &Leaf<APP::MSG>) -> Node {
        match leaf {
            Leaf::Text(txt) => DomNode::create_text_node(txt).into(),
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
            Leaf::Fragment(nodes) => self.create_document_fragment(nodes),
            // NodeList that goes here is only possible when it is the root_node,
            // since node_list as children will be unrolled into as child_elements of the parent
            // We need to wrap this node_list into doc_fragment since root_node is only 1 element
            Leaf::NodeList(node_list) => self.create_document_fragment(node_list),
            Leaf::StatefulComponent(comp) => self.create_stateful_component(comp),
            Leaf::StatelessComponent(comp) => self.create_stateless_component(comp),
            Leaf::TemplatedView(view) => {
                unreachable!("template view should not be created: {:#?}", view)
            }
        }
    }
    */

    /*
    /// TODO: register the template if not yet
    /// pass a program to leaf component and mount itself and its view to the program
    /// There are 2 types of children components of Stateful Component
    /// - Internal children
    /// - External children
    /// Internal children is managed by the Stateful Component
    /// while external children are managed by the top level program.
    /// The external children can be diffed, and send the patches to the StatefulComponent
    ///   - The TreePath of the children starts at the external children location
    /// The attributes affects the Stateful component state.
    /// The attributes can be diff and send the patches to the StatefulComponent
    ///  - Changes to the attributes will call on attribute_changed of the StatefulComponent
    fn create_stateful_component(&self, comp: &StatefulModel<APP::MSG>) -> Node {
        let comp_node = self.create_dom_node(&crate::html::div(
            [crate::html::attributes::class("component")]
                .into_iter()
                .chain(comp.attrs.clone().into_iter()),
            [],
        ));
        // the component children is manually appended to the StatefulComponent
        // here to allow the conversion of dom nodes with its event
        // listener and removing the generics msg
        for child in comp.children.iter() {
            let child_dom = self.create_dom_node(&child);
            comp.comp.borrow_mut().append_child(&child_dom);
            Self::dispatch_mount_event(&child_dom);
        }
        comp_node
    }
    */

    /*
    fn create_stateless_component(&self, comp: &StatelessModel<APP::MSG>) -> Node {
        #[cfg(feature = "with-debug")]
        let t1 = now();
        let comp_view = &comp.view;
        let vdom_template = comp_view.template();
        #[cfg(feature = "with-debug")]
        let t2 = now();
        let skip_diff = comp_view.skip_diff();
        match (vdom_template, skip_diff) {
            (Some(vdom_template), Some(skip_diff)) => {
                let template = register_template(comp.type_id, &vdom_template);
                let real_comp_view = comp_view.unwrap_template_ref();
                let patches =
                    self.create_patches_with_skip_diff(&vdom_template, &real_comp_view, &skip_diff);
                #[cfg(feature = "with-debug")]
                let t3 = now();
                let dom_patches = self
                    .convert_patches(&template, &patches)
                    .expect("convert patches");
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
                template
            }
            _ => {
                // create dom node without skip diff
                self.create_dom_node(&comp.view)
            }
        }
    }
    */

    /// Create and return a `CreatedNode` instance (containing a DOM `Node`
    /// together with potentially related closures) for this virtual node.
    pub fn create_dom_node(&self, vnode: &vdom::Node<APP::MSG>) -> DomNode {
        self.into_dom_node(None, vnode)
        /*
        match vnode {
            vdom::Node::Leaf(leaf_node) => self.create_leaf_node(leaf_node),
            vdom::Node::Element(element_node) => {
                let created_node = self.create_element_node(element_node);
                for child in element_node.children().iter() {
                    if let Some(child_text) = child.as_safe_html() {
                        // https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML
                        // TODO: html parse the SafeHtml -> maybe rename it to RawHtml
                        let created_element: &Element = created_node.unchecked_ref();
                        created_element
                            .insert_adjacent_html(intern("beforeend"), child_text)
                            .expect("must not error");
                    } else {
                        let created_child = self.create_dom_node(child);

                        Self::append_child_and_dispatch_mount_event(&created_node, &created_child);
                    }
                }
                created_node
            }
        }
        */
    }

    /*
    /// Build a DOM element by recursively creating DOM nodes for this element and it's
    /// children, it's children's children, etc.
    fn create_element_node(&self, velem: &vdom::Element<APP::MSG>) -> Node {
        let document = document();

        let element = if let Some(namespace) = velem.namespace() {
            document
                .create_element_ns(Some(intern(namespace)), intern(velem.tag()))
                .expect("Unable to create element")
        } else {
            DomNode::create_element(velem.tag())
        };

        let attrs = Attribute::merge_attributes_of_same_name(velem.attributes().iter());

        self.set_element_dom_attrs(
            &element,
            attrs
                .iter()
                .map(|a| self.convert_attr(a))
                .collect::<Vec<_>>(),
        );

        element.into()
    }
    */

    /// dispatch the mount event,
    /// call the listener since browser don't allow asynchronous execution of
    /// dispatching custom events (non-native browser events)
    ///
    pub(crate) fn dispatch_mount_event(target_node: &Node) {
        let event_target: &web_sys::EventTarget = target_node.unchecked_ref();
        assert_eq!(
            Ok(true),
            event_target.dispatch_event(&MountEvent::create_web_event())
        );
    }

    pub(crate) fn dispatch_mount_event_to_children(
        target_node: &Node,
        deep: usize,
        current_depth: usize,
    ) {
        if current_depth > deep {
            Self::dispatch_mount_event(&target_node);
        }
        let children = target_node.child_nodes();
        let len = children.length();
        for i in 0..len {
            let child = children.get(i).expect("child");
            Self::dispatch_mount_event_to_children(&child, deep, current_depth + 1);
        }
    }

    /// a helper method to append a node to its parent and trigger a mount event if there is any
    pub(crate) fn append_child_and_dispatch_mount_event(parent: &Node, child_node: &Node) {
        parent
            .append_child(child_node)
            .expect("must append child node");
        Self::dispatch_mount_event(child_node);
    }

    /// clear all children of the element
    pub(crate) fn clear_children(target_node: &Node) {
        while let Some(first_child) = target_node.first_child() {
            target_node
                .remove_child(&first_child)
                .expect("must remove child");
        }
    }

    /// set element with the dom attrs
    pub(crate) fn set_element_dom_attrs(
        &self,
        element: &Element,
        attrs: Vec<DomAttr>,
    ) -> Option<NamedEventClosures> {
        attrs
            .into_iter()
            .filter_map(|att| self.set_element_dom_attr(element, att))
            .reduce(|mut acc, e| {
                e.into_iter().for_each(|(k, v)| {
                    acc.insert(k, v);
                });
                acc
            })
    }

    /// set the element with dom attr
    pub(crate) fn set_element_dom_attr(
        &self,
        element: &Element,
        attr: DomAttr,
    ) -> Option<NamedEventClosures> {
        let attr_name = intern(attr.name);
        let attr_namespace = attr.namespace;

        let GroupedDomAttrValues {
            listeners,
            plain_values,
            styles,
            function_calls,
        } = attr.group_values();

        DomAttr::set_element_style(element, attr_name, styles);
        DomAttr::set_element_function_call_values(element, attr_name, function_calls);
        DomAttr::set_element_simple_values(element, attr_name, attr_namespace, plain_values);
        self.set_element_listeners(element, attr_name, listeners)
    }

    pub(crate) fn set_element_listeners(
        &self,
        element: &Element,
        attr_name: AttributeName,
        listeners: Vec<Closure<dyn FnMut(web_sys::Event)>>,
    ) -> Option<NamedEventClosures> {
        for listener in listeners.iter() {
            self.add_event_listener(element, attr_name, &listener)
                .expect("add listener");
        }

        if !listeners.is_empty() {
            let unique_id = create_unique_identifier();
            // set the data-sauron_vdom-id this will be read later on
            // when it's time to remove this element and its closures and event listeners
            element
                .set_attribute(intern(DATA_VDOM_ID), &unique_id.to_string())
                .expect("Could not set attribute on element");

            let listener_closures: IndexMap<&'static str, Closure<dyn FnMut(web_sys::Event)>> =
                IndexMap::from_iter(listeners.into_iter().map(|c| (attr_name, c)));

            /*
            self.node_closures
                .borrow_mut()
                .insert(unique_id, listener_closures);
            */
            Some(listener_closures)
        } else {
            None
        }
    }

    /// attach and event listener to an event target
    pub(crate) fn add_event_listeners(
        &self,
        target: &web_sys::EventTarget,
        event_listeners: Vec<Attribute<APP::MSG>>,
    ) -> Result<(), JsValue> {
        let dom_attrs = event_listeners
            .into_iter()
            .map(|a| self.convert_attr(&a))
            .collect();
        self.add_event_dom_listeners(target, dom_attrs)?;
        Ok(())
    }

    /// attach and event listener to an event target
    pub(crate) fn add_event_dom_listeners(
        &self,
        target: &web_sys::EventTarget,
        event_listeners: Vec<DomAttr>,
    ) -> Result<(), JsValue> {
        for attr in event_listeners.into_iter() {
            for event_cb in attr.value.into_iter() {
                let closure: Closure<dyn FnMut(web_sys::Event)> =
                    event_cb.as_event_closure().expect("expecting a callback");
                self.add_event_listener(target, attr.name, &closure)?;
                self.event_closures.borrow_mut().push(closure);
            }
        }
        Ok(())
    }

    /// add a event listener to a target element
    pub(crate) fn add_event_listener(
        &self,
        event_target: &web_sys::EventTarget,
        event_name: &str,
        listener: &Closure<dyn FnMut(web_sys::Event)>,
    ) -> Result<(), JsValue> {
        event_target.add_event_listener_with_callback(
            intern(event_name),
            listener.as_ref().unchecked_ref(),
        )?;
        Ok(())
    }

    /// remove all the event listeners for this node
    pub(crate) fn remove_event_listeners_recursive(
        &self,
        target_element: &Element,
    ) -> Result<(), JsValue> {
        let all_descendant_vdom_id = get_node_descendant_data_vdom_id(target_element);
        let mut node_closures = self.node_closures.borrow_mut();
        for vdom_id in all_descendant_vdom_id {
            if let Some(old_closure) = node_closures.get(&vdom_id) {
                for (event, oc) in old_closure.iter() {
                    let func: &Function = oc.as_ref().unchecked_ref();
                    target_element.remove_event_listener_with_callback(intern(event), func)?;
                }
                // remove closure active_closure in dom_updater to free up memory
                node_closures
                    .swap_remove(&vdom_id)
                    .expect("Unable to remove old closure");
            } else {
                log::warn!("There is no closure marked with that vdom_id: {}", vdom_id);
            }
        }
        Ok(())
    }

    /// remove the event listener which matches the given event name
    /// TODO: this is iterating over the decedant nodes to find the `vdom-id`
    /// maybe we can make the dropping of closure faster
    /// by making it automatically dropped by wrapping the Node with its closure
    pub(crate) fn remove_event_listener_with_name(
        &self,
        event_name: &'static str,
        target_element: &Element,
    ) -> Result<(), JsValue> {
        let mut node_closures = self.node_closures.borrow_mut();
        if let Some(vdom_id) = get_node_data_vdom_id(target_element) {
            if let Some(old_closure) = node_closures.get_mut(&vdom_id) {
                for (event, oc) in old_closure.iter() {
                    if *event == event_name {
                        let func: &Function = oc.as_ref().unchecked_ref();
                        target_element.remove_event_listener_with_callback(intern(event), func)?;
                    }
                }

                old_closure.retain(|event, _oc| *event != event_name);

                // remove closure active_closure in dom_updater to free up memory
                if old_closure.is_empty() {
                    node_closures
                        .swap_remove(&vdom_id)
                        .expect("Unable to remove old closure");
                }
            } else {
                log::warn!("There is no closure marked with that vdom_id: {}", vdom_id);
            }
        }
        Ok(())
    }
}

pub(crate) fn find_node(target_node: &DomNode, path: &mut TreePath) -> Option<DomNode> {
    if path.is_empty() {
        Some(target_node.clone())
    } else {
        let idx = path.remove_first();
        let children = target_node.children()?;
        if let Some(child) = children.get(idx) {
            find_node(&child, path)
        } else {
            None
        }
    }
}

pub(crate) fn find_all_nodes(
    target_node: &DomNode,
    nodes_to_find: &[(&TreePath, Option<&&'static str>)],
) -> IndexMap<TreePath, DomNode> {
    let mut nodes_to_patch: IndexMap<TreePath, DomNode> = IndexMap::with_capacity(nodes_to_find.len());
    for (path, tag) in nodes_to_find {
        let mut traverse_path: TreePath = (*path).clone();
        if let Some(found) = find_node(target_node, &mut traverse_path) {
            nodes_to_patch.insert((*path).clone(), found);
        } else {
            log::warn!(
                "can not find: {:?} {:?} target_node: {:?}",
                path,
                tag,
                target_node
            );
        }
    }
    nodes_to_patch
}

/// return the "data-vdom-id" value of this node
fn get_node_data_vdom_id(target_element: &Element) -> Option<usize> {
    if let Some(vdom_id_str) = target_element.get_attribute(intern(DATA_VDOM_ID)) {
        let vdom_id = vdom_id_str
            .parse::<usize>()
            .expect("unable to parse sauron_vdom-id");
        Some(vdom_id)
    } else {
        None
    }
}

/// Get the "data-vdom-id" of all the desendent of this node including itself
/// This is needed to free-up the closure that was attached ActiveClosure manually
fn get_node_descendant_data_vdom_id(target_element: &Element) -> Vec<usize> {
    let mut data_vdom_id = vec![];

    if let Some(vdom_id) = get_node_data_vdom_id(target_element) {
        data_vdom_id.push(vdom_id);
    }

    let children = target_element.child_nodes();
    let child_node_count = children.length();
    for i in 0..child_node_count {
        let child_node = children.item(i).expect("Expecting a child node");
        if child_node.node_type() == Node::ELEMENT_NODE {
            let child_element = child_node.unchecked_ref::<Element>();
            let child_data_vdom_id = get_node_descendant_data_vdom_id(child_element);
            data_vdom_id.extend(child_data_vdom_id);
        }
    }
    data_vdom_id
}
