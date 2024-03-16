use crate::dom::component::lookup_template;
use crate::dom::component::StatelessModel;
#[cfg(feature = "with-debug")]
use crate::dom::now;
use crate::dom::DomAttr;
use crate::dom::GroupedDomAttrValues;
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
#[cfg(feature = "with-debug")]
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{self, Element, Node, Text};

/// data attribute name used in assigning the node id of an element with events
pub(crate) const DATA_VDOM_ID: &str = "data-vdom-id";

thread_local!(static NODE_ID_COUNTER: Cell<usize> = Cell::new(1));

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

#[cfg(feature = "with-debug")]
pub fn total_time_spent() -> Section {
    TIME_SPENT.with_borrow(|values| total(values))
}

// a cache of commonly used elements, so we can clone them.
// cloning is much faster then creating the element
thread_local! {
    static CACHE_ELEMENTS: HashMap<&'static str, web_sys::Element> =
        HashMap::from_iter(["div", "span", "ol", "ul", "li"].map(web_sys::Node::create_element_with_tag));
}

/// Provides helper traits for web_sys::Node
pub trait DomNode {
    /// return the inner html if it is an element
    fn inner_html(&self) -> Option<String>;

    /// create a text node
    fn create_text_node(txt: &str) -> Text {
        document().create_text_node(txt)
    }

    /// create a web_sys::Element with the specified tag
    fn create_element_with_tag(tag: &'static str) -> (&'static str, web_sys::Element) {
        let elm = document().create_element(intern(tag)).unwrap();
        (tag, elm)
    }

    /// create dom element from tag name
    #[cfg(feature = "use-cached-elements")]
    fn create_element(tag: &'static str) -> web_sys::Element {
        // find the element from the most created element and clone it, else create it
        CACHE_ELEMENTS.with(|map| {
            if let Some(elm) = map.get(tag) {
                elm.clone_node_with_deep(false)
                    .expect("must clone node")
                    .unchecked_into()
            } else {
                document()
                    .create_element(intern(tag))
                    .expect("create element")
            }
        })
    }

    /// create dom element from tag name
    #[cfg(not(feature = "use-cached-elements"))]
    fn create_element(tag: &'static str) -> web_sys::Element {
        document()
            .create_element(intern(tag))
            .expect("create element")
    }

    ///
    fn render_to_string(&self) -> String;

    ///
    fn render(&self, buffer: &mut dyn fmt::Write) -> fmt::Result;
}

impl DomNode for web_sys::Node {
    fn inner_html(&self) -> Option<String> {
        let element: &Element = self.dyn_ref()?;
        Some(element.inner_html())
    }

    fn render_to_string(&self) -> String {
        let mut buffer = String::new();
        self.render(&mut buffer).expect("must render");
        buffer
    }

    fn render(&self, buffer: &mut dyn fmt::Write) -> fmt::Result {
        match self.node_type() {
            Node::TEXT_NODE => {
                let text_node = self.unchecked_ref::<Text>();
                let text = text_node.whole_text().expect("whole text");
                write!(buffer, "{text}")?;
                Ok(())
            }
            Node::ELEMENT_NODE => {
                let elm = self.unchecked_ref::<Element>();
                let tag = elm.tag_name().to_lowercase();

                write!(buffer, "<{tag}")?;
                let attrs = elm.attributes();
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

                let children = elm.children();
                let children_len = children.length();
                for i in 0..children_len {
                    let child = children.item(i).expect("element child");
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
    fn create_document_fragment(&self, nodes: &[vdom::Node<APP::MSG>]) -> Node {
        let doc_fragment = document().create_document_fragment();
        for vnode in nodes {
            let created_node = self.create_dom_node(vnode);
            Self::append_child_and_dispatch_mount_event(&doc_fragment, &created_node)
        }
        doc_fragment.into()
    }

    fn create_leaf_node(&self, leaf: &Leaf<APP::MSG>) -> Node {
        match leaf {
            Leaf::Text(txt) => web_sys::Node::create_text_node(txt).into(),
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
            Leaf::StatefulComponent(comp) => self.create_stateful_component(comp),
            Leaf::StatelessComponent(comp) => self.create_stateless_component(comp),
        }
    }

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
        let comp_node = self.create_dom_node(&crate::html::div(comp.attrs.clone(), []));
        // the component children is manually appended to the StatefulComponent
        // here to allow the conversion of dom nodes with its event
        // listener and removing the generics msg
        for child in comp.children.iter() {
            let child_dom = self.create_dom_node(&child);
            Self::dispatch_mount_event(&child_dom);
            comp.comp.borrow_mut().append_child(&child_dom);
        }
        comp_node
    }

    fn create_stateless_component(&self, comp: &StatelessModel<APP::MSG>) -> Node {
        #[cfg(feature = "with-debug")]
        let t1 = now();
        let template = lookup_template(comp.type_id);
        #[cfg(feature = "with-debug")]
        let t2 = now();
        let skip_diff = self.app_context.skip_diff.as_ref();
        match (template, skip_diff) {
            (Some(template), Some(skip_diff)) => {
                // use dept_limit 0 if it involves template
                let patches =
                    self.create_patches_with_skip_diff(&comp.vdom_template, &comp.view, skip_diff, Some(0));
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

                //log::info!("looking up template took: {}ms", t2 - t1);
                //log::info!("diffing took: {}ms", t3 - t2);
                //log::info!("converting patches took: {}ms", t4 - t3);
                //log::info!("applying patches took: {}ms", t5 - t4);
                //log::info!("creating stateless component took: {}ms", t5 - t1);
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

    /// Create and return a `CreatedNode` instance (containing a DOM `Node`
    /// together with potentially related closures) for this virtual node.
    pub fn create_dom_node(&self, vnode: &vdom::Node<APP::MSG>) -> Node {
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
            vdom::Node::Fragment(nodes) => self.create_document_fragment(nodes),
            // NodeList that goes here is only possible when it is the root_node,
            // since node_list as children will be unrolled into as child_elements of the parent
            // We need to wrap this node_list into doc_fragment since root_node is only 1 element
            vdom::Node::NodeList(node_list) => self.create_document_fragment(node_list),
        }
    }

    /// Build a DOM element by recursively creating DOM nodes for this element and it's
    /// children, it's children's children, etc.
    fn create_element_node(&self, velem: &vdom::Element<APP::MSG>) -> Node {
        let document = document();

        let element = if let Some(namespace) = velem.namespace() {
            document
                .create_element_ns(Some(intern(namespace)), intern(velem.tag()))
                .expect("Unable to create element")
        } else {
            Node::create_element(velem.tag())
        };

        let attrs = Attribute::merge_attributes_of_same_name(velem.attributes().iter());

        Self::set_element_dom_attrs(
            self,
            &element,
            attrs
                .iter()
                .map(|a| self.convert_attr(a))
                .collect::<Vec<_>>(),
        );

        element.into()
    }

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
    pub(crate) fn set_element_dom_attrs(&self, element: &Element, attrs: Vec<DomAttr>) {
        for att in attrs.into_iter() {
            self.set_element_dom_attr(element, att);
        }
    }

    /// set the element with dom attr
    pub(crate) fn set_element_dom_attr(&self, element: &Element, attr: DomAttr) {
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
        self.set_element_listeners(element, attr_name, listeners);
    }

    pub(crate) fn set_element_listeners(
        &self,
        element: &Element,
        attr_name: AttributeName,
        listeners: Vec<Closure<dyn FnMut(web_sys::Event)>>,
    ) {
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

            let listener_closures: micromap::Map<
                &'static str,
                Closure<dyn FnMut(web_sys::Event)>,
                5,
            > = micromap::Map::from_iter(listeners.into_iter().map(|c| (attr_name, c)));

            self.node_closures
                .borrow_mut()
                .insert(unique_id, listener_closures);
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

pub(crate) fn find_node(target_node: &Node, path: &mut TreePath) -> Option<Node> {
    if path.is_empty() {
        Some(target_node.clone())
    } else {
        let idx = path.remove_first();
        let children = target_node.child_nodes();
        if let Some(child) = children.item(idx as u32) {
            find_node(&child, path)
        } else {
            None
        }
    }
}

pub(crate) fn find_all_nodes(
    target_node: &Node,
    nodes_to_find: &[(&TreePath, Option<&&'static str>)],
) -> IndexMap<TreePath, Node> {
    let mut nodes_to_patch: IndexMap<TreePath, Node> = IndexMap::with_capacity(nodes_to_find.len());
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
