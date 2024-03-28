use crate::dom::component::StatelessModel;
use crate::dom::DomAttr;
use crate::dom::GroupedDomAttrValues;
use crate::dom::StatefulComponent;
use crate::dom::StatefulModel;
use crate::html::lookup;
use crate::vdom::TreePath;
use crate::{
    dom::document,
    dom::events::MountEvent,
    dom::{Application, Program},
    vdom,
    vdom::{Attribute, Leaf},
};
use indexmap::IndexMap;
use std::cell::Cell;
use std::cell::Ref;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{self, Element, Node};

pub(crate) type EventClosure = Closure<dyn FnMut(web_sys::Event)>;
pub type NamedEventClosures = IndexMap<&'static str, EventClosure>;

thread_local!(static NODE_ID_COUNTER: Cell<usize> = Cell::new(1));


/// A counter part of the vdom Node
/// This is needed, so that we can
/// 1. Keep track of event closure and drop them when nodes has been removed
/// 2. Custom removal of children nodes on a stateful component
///
#[derive(Clone, Debug)]
pub struct DomNode {
    pub(crate) inner: DomInner,
    //TODO: don't really need to have reference to the parent
    //as RemoveNode patch can just be called with Node::remove
    //though remove doesn't return the node to be removed
    //
    //MoveAfterNode and  with insertAdjacentElement
    pub(crate) parent: Rc<RefCell<Option<DomNode>>>,
}
#[derive(Clone)]
pub enum DomInner {
    /// a reference to an element node
    Element {
        /// the reference to the actual element
        element: web_sys::Element,
        /// the listeners of this element, which we will drop when this element is removed
        listeners: Rc<RefCell<Option<NamedEventClosures>>>,
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
    #[allow(unused)]
    StatefulComponent(Rc<RefCell<dyn StatefulComponent>>),
}

impl fmt::Debug for DomInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Element {
                element, children, ..
            } => {
                f.debug_struct("Element")
                    .field("tag", &element.tag_name().to_lowercase())
                    .field(
                        "children",
                        &children
                            .borrow()
                            .iter()
                            .map(|c| c.as_element().tag_name().to_lowercase())
                            .collect::<Vec<_>>(),
                    )
                    .finish()?;
                Ok(())
            }
            Self::Text(text_node) => f
                .debug_tuple("Text")
                .field(&text_node.borrow().whole_text().expect("whole text"))
                .finish(),
            Self::Comment(_) => write!(f, "Comment"),
            Self::Fragment { .. } => write!(f, "Fragment"),
            Self::StatefulComponent(_) => write!(f, "StatefulComponent"),
        }
    }
}


impl From<web_sys::Node> for DomNode {
    fn from(node: web_sys::Node) -> Self {
        let element: web_sys::Element = node.dyn_into().expect("must be an element");
        DomNode {
            inner: DomInner::Element {
                element,
                listeners: Rc::new(RefCell::new(None)),
                children: Rc::new(RefCell::new(vec![])),
            },
            parent: Rc::new(RefCell::new(None)),
        }
    }
}

impl DomNode {
    fn children(&self) -> Option<Ref<'_, Vec<DomNode>>> {
        match &self.inner {
            DomInner::Element { children, .. } => Some(children.borrow()),
            DomInner::Fragment { children, .. } => Some(children.borrow()),
            _ => None,
        }
    }

    pub(crate) fn is_fragment(&self) -> bool {
        matches!(&self.inner, DomInner::Fragment { .. })
    }

    pub(crate) fn tag(&self) -> Option<String> {
        match &self.inner {
            DomInner::Element { element, .. } => Some(element.tag_name().to_lowercase()),
            _ => None,
        }
    }

    /// exposed the underlying wrapped node as `web_sys::Node`
    pub fn as_node(&self) -> web_sys::Node {
        match &self.inner {
            DomInner::Element { element, .. } => element.clone().unchecked_into(),
            DomInner::Fragment { fragment, .. } => fragment.clone().unchecked_into(),
            DomInner::Text(text_node) => text_node.borrow().clone().unchecked_into(),
            DomInner::Comment(comment_node) => comment_node.clone().unchecked_into(),
            DomInner::StatefulComponent(_) => todo!("for stateful component.."),
        }
    }

    pub(crate) fn as_element(&self) -> web_sys::Element {
        match &self.inner {
            DomInner::Element { element, .. } => element.clone().unchecked_into(),
            DomInner::Fragment { fragment, .. } => {
                let fragment: web_sys::Element = fragment.clone().unchecked_into();
                assert!(fragment.is_object());
                fragment
            }
            DomInner::Text(text_node) => text_node.borrow().clone().unchecked_into(),
            DomInner::Comment(comment_node) => comment_node.clone().unchecked_into(),
            DomInner::StatefulComponent(_) => todo!("for stateful component.."),
        }
    }

    /// return the outer html string of an element
    pub fn outer_html(&self) -> String {
        let DomInner::Element {
            element,
            ..
        } = &self.inner
        else {
            unreachable!("should only be called to element");
        };
        element.outer_html()
    }

    fn set_parent(&self, parent_node: &DomNode) {
        *self.parent.borrow_mut() = Some(parent_node.clone());
    }

    /// append the DomNode `child` into this DomNode `self`
    pub fn append_children(&self, for_append: impl IntoIterator<Item = DomNode>) -> Result<(), JsValue> {
        match &self.inner {
            DomInner::Element {
                element, children, ..
            } => {
                let for_append = for_append.into_iter();
                for child in for_append{
                    element
                        .append_child(&child.as_node())
                        .expect("append child");
                    child.set_parent(&self);
                    children.borrow_mut().push(child);
                }
                Ok(())
            }
            _ => unreachable!(
                "appending should only be called to Element and Fragment, found: {:#?}",
                self
            ),
        }
    }

    /// Insert the DomNode `for_insert` before `self` DomNode
    pub(crate) fn insert_before(&self, for_insert: DomNode) -> Result<Option<DomNode>, JsValue> {
        let DomInner::Element {
            element: target_element,
            ..
        } = &self.inner
        else {
            unreachable!("target element should be an element");
        };
        let parent_target = self.parent.borrow();
        let parent_target = parent_target.as_ref().expect("must have a parent");
        let DomInner::Element {
            element: parent_element,
            children: parent_children,
            ..
        } = &parent_target.inner
        else {
            unreachable!("parent must be an element");
        };
        parent_element
            .insert_before(&for_insert.as_node(), Some(&target_element))
            .expect("must remove target node");

        let mut self_index = None;
        for (i,child) in parent_children.borrow().iter().enumerate(){
            if self.as_node() == child.as_node(){
                self_index = Some(i);
            }
        }
        for_insert.set_parent(parent_target);
        if let Some(self_index) = self_index{
            log::info!("insert before self_index: {}", self_index);
            parent_children.borrow_mut().insert(self_index, for_insert);
        }else{
            unreachable!("should have a self index");
        }
        Ok(None)
    }

    /// Insert the DomNode `for_insert` after `self` DomNode
    pub(crate) fn insert_after(&self, for_insert: DomNode) -> Result<Option<DomNode>, JsValue> {
        let target_element = match &self.inner {
            DomInner::Element { element, .. } => element,
            _ => unreachable!("target element should be an element"),
        };
        target_element.insert_adjacent_element(intern("afterend"), &for_insert.as_element())?;

        let parent_target = self.parent.borrow();
        let parent_target = parent_target.as_ref().expect("must have a parent");
        let DomInner::Element {
            children: parent_children,
            ..
        } = &parent_target.inner
        else {
            unreachable!("parent must be an element");
        };
        let mut self_index = None;
        for (i,child) in parent_children.borrow().iter().enumerate(){
            if self.as_node() == child.as_node(){
                self_index = Some(i);
            }
        }
        for_insert.set_parent(parent_target);
        if let Some(self_index) = self_index{
            parent_children.borrow_mut().insert(self_index + 1, for_insert);
        }else{
            unreachable!("should have a self index");
        }
        Ok(None)
    }

    /// Replace the child `child` DomNode with a replacement DomNode `replacement`
    pub(crate) fn replace_child(&self, child: &DomNode, replacement: DomNode) -> Option<DomNode> {
        log::debug!("atttempt to replace child..{}", child.render_to_string());
        match &self.inner {
            DomInner::Element { children, .. } => {
                let mut child_index = None;
                for (i, c) in children.borrow().iter().enumerate() {
                    if c.as_node() == child.as_node() {
                        child_index = Some(i);
                    }
                }
                if let Some(child_index) = child_index {
                    log::info!("--oo in replace_child, Replacing at: {}", child_index);
                    let child = children.borrow_mut().remove(child_index);
                    log::info!("--oo Removed {}", child.render_to_string());
                    child
                        .as_element()
                        .replace_with_with_node_1(&replacement.as_node())
                        .expect("must replace child");
                    replacement.set_parent(self);
                    log::info!("--oo in replace_child, Inserted at: {}, {}", child_index, replacement.render_to_string());
                    children.borrow_mut().insert(child_index, replacement);
                    Some(child)
                } else {
                    // if can not find the child, then must be the root node
                    unreachable!("must find the child...");
                }
            }
            _ => todo!(),
        }
    }

    /// Remove the DomNode `child` from the children of `self`
    pub(crate) fn remove_child(&self, child: &DomNode) -> Option<DomNode> {
        match &self.inner {
            DomInner::Element {
                element, children, ..
            } => {
                let mut child_index = None;
                {
                    for (i, c) in children.borrow().iter().enumerate() {
                        if c.as_node() == child.as_node() {
                            child_index = Some(i);
                        }
                    }
                }
                if let Some(child_index) = child_index {
                    log::info!("-->> In remove_child, Removing child at {}", child_index);
                    let child = children.borrow_mut().remove(child_index);
                    log::info!("-->> In remove_child, The remove child: {}", child.render_to_string());
                    element
                        .remove_child(&child.as_node())
                        .expect("remove child");
                    Some(child)
                } else {
                    unreachable!("no parent")
                }
            }
            _ => todo!(),
        }
    }

    /// remove all the children of this element
    pub(crate) fn clear_children(&self) {
        use crate::dom::now;
        match &self.inner {
            DomInner::Element {
                element, children, ..
            } => {
                let t1 = now();
                children.borrow_mut().clear();
                let t2 = now();
                while let Some(last_child) = element.last_child() {
                    element
                        .remove_child(&last_child)
                        .expect("must remove child");
                }
                let t3 = now();
                log::info!("for_each took: {}ms", t2 - t1);
                log::info!("while loop took: {}ms", t3 - t2);
            }
            _ => todo!(),
        }
    }

    pub(crate) fn remove_node(&self) {
        if let Some(parent) = self.parent.borrow().as_ref() {
            parent.remove_child(self);
        }
    }

    pub(crate) fn replace_node(&self, replacement: DomNode) -> Result<Option<DomNode>, JsValue> {
        if let Some(parent) = self.parent.borrow().as_ref() {
            parent.replace_child(self, replacement);
        } else {
            log::info!("There is no parent here..");
            unreachable!("unable to replace a node without a parent..");
        }
        Ok(None)
    }


    pub(crate) fn set_dom_attrs(
        &self,
        attrs: impl IntoIterator<Item = DomAttr>,
    ) -> Result<(), JsValue> {
        for attr in attrs.into_iter() {
            self.set_dom_attr(attr)?;
        }
        Ok(())
    }

    fn set_dom_attr(&self, attr: DomAttr) -> Result<(), JsValue> {
        match &self.inner {
            DomInner::Element {
                element, listeners, ..
            } => {
                let attr_name = intern(attr.name);
                let attr_namespace = attr.namespace;

                let GroupedDomAttrValues {
                    listeners: event_callbacks,
                    plain_values,
                    styles,
                } = attr.group_values();

                Self::add_event_dom_listeners(&element, attr_name, &event_callbacks)
                    .expect("event listeners");
                let is_none = listeners.borrow().is_none();
                if is_none {
                    let listener_closures: IndexMap<
                        &'static str,
                        Closure<dyn FnMut(web_sys::Event)>,
                    > = IndexMap::from_iter(event_callbacks.into_iter().map(|c| (attr_name, c)));
                    *listeners.borrow_mut() = Some(listener_closures);
                } else if let Some(listeners) = listeners.borrow_mut().as_mut() {
                    for event_cb in event_callbacks.into_iter() {
                        listeners.insert(attr_name, event_cb);
                    }
                }

                DomAttr::set_element_style(&element, attr_name, styles);
                DomAttr::set_element_simple_values(
                    &element,
                    attr_name,
                    attr_namespace,
                    plain_values,
                );
            }
            _ => unreachable!("should only be called for element"),
        }
        Ok(())
    }

    pub(crate) fn remove_dom_attr(&self, attr: &DomAttr) -> Result<(), JsValue> {
        let DomInner::Element { element, .. } = &self.inner else {
            unreachable!("expecting an element");
        };
        DomAttr::remove_element_dom_attr(&element, attr)
    }

    /// attach and event listener to an event target
    pub(crate) fn add_event_dom_listeners(
        target: &web_sys::EventTarget,
        attr_name: &'static str,
        event_listeners: &[EventClosure],
    ) -> Result<(), JsValue> {
        for event_cb in event_listeners.into_iter() {
            Self::add_event_listener(target, attr_name, &event_cb)?;
        }
        Ok(())
    }

    /// add a event listener to a target element
    pub(crate) fn add_event_listener(
        event_target: &web_sys::EventTarget,
        event_name: &str,
        listener: &EventClosure,
    ) -> Result<(), JsValue> {
        event_target.add_event_listener_with_callback(
            intern(event_name),
            listener.as_ref().unchecked_ref(),
        )?;
        Ok(())
    }

    /// render this DomNode into an html string represenation
    pub fn render_to_string(&self) -> String {
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
            DomInner::Comment(comment) => {
                write!(buffer, "<!--{}-->", comment.data())
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
            DomInner::Fragment { children, .. } => {
                for child in children.borrow().iter() {
                    child.render(buffer)?;
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

impl<APP> Program<APP>
where
    APP: Application + 'static,
{
    /// Create a dom node
    pub fn create_dom_node(
        &self,
        parent_node: Option<DomNode>,
        node: &vdom::Node<APP::MSG>,
    ) -> DomNode {
        match node {
            vdom::Node::Element(elm) => self.create_element_node(parent_node, elm),
            vdom::Node::Leaf(leaf) => self.create_leaf_node(parent_node, leaf),
        }
    }

    fn create_element_node(
        &self,
        parent_node: Option<DomNode>,
        elm: &vdom::Element<APP::MSG>,
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

        let listeners = self.set_element_dom_attrs(
            &element,
            attrs
                .iter()
                .map(|a| self.convert_attr(a))
                .collect::<Vec<_>>(),
        );
        let dom_node = DomNode {
            inner: DomInner::Element {
                element,
                listeners: Rc::new(RefCell::new(listeners)),
                children: Rc::new(RefCell::new(vec![])),
            },
            parent: Rc::new(RefCell::new(parent_node)),
        };
        let children: Vec<DomNode> = elm
            .children()
            .iter()
            .map(|child| self.create_dom_node(Some(dom_node.clone()), child))
            .collect();
        dom_node.append_children(children).unwrap();
        dom_node
    }

    fn create_leaf_node(
        &self,
        parent_node: Option<DomNode>,
        leaf: &vdom::Leaf<APP::MSG>,
    ) -> DomNode {
        match leaf {
            Leaf::Text(txt) => DomNode {
                inner: DomInner::Text(RefCell::new(document().create_text_node(txt))),
                parent: Rc::new(RefCell::new(parent_node)),
            },
            Leaf::Comment(comment) => DomNode {
                inner: DomInner::Comment(document().create_comment(comment)),
                parent: Rc::new(RefCell::new(parent_node)),
            },
            Leaf::Fragment(nodes) => self.create_fragment_node(parent_node, nodes),
            // NodeList that goes here is only possible when it is the root_node,
            // since node_list as children will be unrolled into as child_elements of the parent
            // We need to wrap this node_list into doc_fragment since root_node is only 1 element
            Leaf::NodeList(nodes) => self.create_fragment_node(parent_node, nodes),
            Leaf::StatefulComponent(comp) => self.create_stateful_component(parent_node, comp),
            Leaf::StatelessComponent(comp) => self.create_stateless_component(parent_node, comp),
            Leaf::TemplatedView(view) => {
                unreachable!("template view should not be created: {:#?}", view)
            }
            Leaf::DocType(_) => unreachable!("doc type is never converted"),
        }
    }

    fn create_fragment_node<'a>(
        &self,
        parent_node: Option<DomNode>,
        nodes: impl IntoIterator<Item = &'a vdom::Node<APP::MSG>>,
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
            .map(|node| self.create_dom_node(Some(dom_node.clone()), &node));
        dom_node.append_children(children).unwrap();
        dom_node
    }
}

/// A node along with all of the closures that were created for that
/// node's events and all of it's child node's events.
impl<APP> Program<APP>
where
    APP: Application,
{
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
    fn create_stateful_component(
        &self,
        parent_node: Option<DomNode>,
        comp: &StatefulModel<APP::MSG>,
    ) -> DomNode {
        let comp_node = self.create_dom_node(
            parent_node.clone(),
            &crate::html::div(
                [crate::html::attributes::class("component")]
                    .into_iter()
                    .chain(comp.attrs.clone().into_iter()),
                [],
            ),
        );
        // the component children is manually appended to the StatefulComponent
        // here to allow the conversion of dom nodes with its event
        // listener and removing the generics msg
        let created_children = comp.children.iter().map(|child| {
            self.create_dom_node(parent_node.clone(), &child)
        }).collect();
        comp.comp.borrow_mut().append_children(created_children);
        comp_node
    }

    fn create_stateless_component(
        &self,
        parent_node: Option<DomNode>,
        comp: &StatelessModel<APP::MSG>,
    ) -> DomNode {
        let comp_view = &comp.view;
        let real_comp_view = comp_view.unwrap_template_ref();
        self.create_dom_node(parent_node, &real_comp_view)
    }

    /// dispatch the mount event,
    /// call the listener since browser don't allow asynchronous execution of
    /// dispatching custom events (non-native browser events)
    #[allow(unused)]
    pub(crate) fn dispatch_mount_event(target_node: &Node) {
        let event_target: &web_sys::EventTarget = target_node.unchecked_ref();
        assert_eq!(
            Ok(true),
            event_target.dispatch_event(&MountEvent::create_web_event())
        );
    }

    #[allow(unused)]
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

    fn set_element_dom_attr(&self, element: &Element, attr: DomAttr) -> Option<NamedEventClosures> {
        let attr_name = intern(attr.name);
        let attr_namespace = attr.namespace;

        let GroupedDomAttrValues {
            listeners,
            plain_values,
            styles,
        } = attr.group_values();

        DomAttr::set_element_style(element, attr_name, styles);
        DomAttr::set_element_simple_values(element, attr_name, attr_namespace, plain_values);
        self.add_event_listeners(element, attr_name, &listeners)
            .unwrap();
        if !listeners.is_empty() {
            let event_closures =
                IndexMap::from_iter(listeners.into_iter().map(|cb| (attr_name, cb)));
            Some(event_closures)
        } else {
            None
        }
    }

    pub(crate) fn add_event_listeners(
        &self,
        event_target: &web_sys::EventTarget,
        event_name: &str,
        listeners: &[EventClosure],
    ) -> Result<(), JsValue> {
        for listener in listeners.iter() {
            self.add_event_listener(event_target, event_name, listener)
                .unwrap();
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
    let mut nodes_to_patch: IndexMap<TreePath, DomNode> =
        IndexMap::with_capacity(nodes_to_find.len());
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
