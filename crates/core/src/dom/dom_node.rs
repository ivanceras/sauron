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
use std::borrow::Cow;
use std::cell::Ref;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{self, Node};

pub(crate) type EventClosure = Closure<dyn FnMut(web_sys::Event)>;
pub type NamedEventClosures = IndexMap<&'static str, EventClosure>;

/// A counter part of the vdom Node
/// This is needed, so that we can
/// 1. Keep track of event closure and drop them when nodes has been removed
/// 2. Custom removal of children nodes on a stateful component
///
#[derive(Clone, Debug)]
pub struct DomNode {
    pub(crate) inner: DomInner,
}
#[derive(Clone)]
pub enum DomInner {
    /// a reference to an element node
    Element {
        /// the reference to the actual element
        element: web_sys::Element,
        // TODO: put all DomAttr here
        /// the listeners of this element, which we will drop when this element is removed
        listeners: Rc<RefCell<Option<NamedEventClosures>>>,
        /// keeps track of the children nodes
        /// this needs to be synced with the actual element children
        children: Rc<RefCell<Vec<DomNode>>>,
        /// determine if this element needs to dispatch a mount event
        has_mount_callback: bool,
    },
    /// text node
    Text(web_sys::Text),
    Symbol(Cow<'static, str>),
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
    StatefulComponent {
        comp: Rc<RefCell<dyn StatefulComponent>>,
        dom_node: Rc<DomNode>,
    },
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
                            .map(|c| c.tag())
                            .collect::<Vec<_>>(),
                    )
                    .finish()?;
                Ok(())
            }
            Self::Text(text_node) => f
                .debug_tuple("Text")
                .field(&text_node.whole_text().expect("whole text"))
                .finish(),
            Self::Symbol(symbol) => f.debug_tuple("Symbol").field(&symbol).finish(),
            Self::Comment(_) => write!(f, "Comment"),
            Self::Fragment { .. } => write!(f, "Fragment"),
            Self::StatefulComponent { .. } => write!(f, "StatefulComponent"),
        }
    }
}

impl From<web_sys::Node> for DomNode {
    fn from(node: web_sys::Node) -> Self {
        match node.node_type() {
            Node::ELEMENT_NODE => {
                let element: web_sys::Element = node.unchecked_into();
                let child_nodes = element.child_nodes();
                let children_count = child_nodes.length();
                let children = (0..children_count)
                    .map(|i| {
                        let child = child_nodes.get(i).expect("child");
                        DomNode::from(child)
                    })
                    .collect();
                DomNode {
                    inner: DomInner::Element {
                        element,
                        listeners: Rc::new(RefCell::new(None)),
                        children: Rc::new(RefCell::new(children)),
                        has_mount_callback: false,
                    },
                }
            }
            Node::TEXT_NODE => {
                let text_node: web_sys::Text = node.unchecked_into();
                DomNode {
                    inner: DomInner::Text(text_node),
                }
            }
            Node::COMMENT_NODE => {
                let comment: web_sys::Comment = node.unchecked_into();
                DomNode {
                    inner: DomInner::Comment(comment),
                }
            }
            Node::DOCUMENT_FRAGMENT_NODE => {
                let fragment: web_sys::DocumentFragment = node.unchecked_into();
                DomNode {
                    inner: DomInner::Fragment {
                        fragment,
                        children: Rc::new(RefCell::new(vec![])),
                    },
                }
            }
            _node_type => todo!("for: {_node_type:?}"),
        }
    }
}

impl PartialEq for DomNode {
    fn eq(&self, other: &Self) -> bool {
        match (&self.inner, &other.inner) {
            (DomInner::Element { element: v, .. }, DomInner::Element { element: o, .. }) => v == o,
            (DomInner::Fragment { fragment: v, .. }, DomInner::Fragment { fragment: o, .. }) => {
                v == o
            }
            (DomInner::Text(v), DomInner::Text(o)) => v == o,
            (DomInner::Symbol(v), DomInner::Symbol(o)) => v == o,
            (DomInner::Comment(v), DomInner::Comment(o)) => v == o,
            (DomInner::StatefulComponent { .. }, DomInner::StatefulComponent { .. }) => todo!(),
            _ => false,
        }
    }
}

impl DomNode {
    pub(crate) fn children(&self) -> Option<Ref<'_, Vec<DomNode>>> {
        match &self.inner {
            DomInner::Element { children, .. } => Some(children.borrow()),
            DomInner::Fragment { children, .. } => Some(children.borrow()),
            _ => None,
        }
    }

    /// returns true if this an element node
    pub fn is_element(&self) -> bool {
        matches!(&self.inner, DomInner::Element { .. })
    }

    /// returns true if this a fragment node
    pub fn is_fragment(&self) -> bool {
        matches!(&self.inner, DomInner::Fragment { .. })
    }

    /// returns true if this a text node
    pub fn is_text_node(&self) -> bool {
        matches!(&self.inner, DomInner::Text(_))
    }

    /// returns true if this Comment node
    pub fn is_comment(&self) -> bool {
        matches!(&self.inner, DomInner::Comment(_))
    }

    /// returns true if this DomNode is a html entity symbol
    pub fn is_symbol(&self) -> bool {
        matches!(&self.inner, DomInner::Symbol(_))
    }

    /// returns true if this is a stateful component
    pub fn is_stateful_component(&self) -> bool {
        matches!(&self.inner, DomInner::StatefulComponent { .. })
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
            DomInner::Text(text_node) => text_node.clone().unchecked_into(),
            DomInner::Symbol(_) => unreachable!("symbol should be handled separately"),
            DomInner::Comment(comment_node) => comment_node.clone().unchecked_into(),
            DomInner::StatefulComponent { dom_node, .. } => dom_node.as_node(),
        }
    }

    /// exposed the underlying wrapped node as `web_sys::Element`
    #[track_caller]
    pub fn as_element(&self) -> web_sys::Element {
        match &self.inner {
            DomInner::Element { element, .. } => element.clone(),
            DomInner::Fragment { fragment, .. } => {
                let fragment: web_sys::Element = fragment.clone().unchecked_into();
                fragment
            }
            DomInner::Text(text_node) => text_node.clone().unchecked_into(),
            DomInner::Symbol(_) => unreachable!("symbol should be handled separately"),
            DomInner::Comment(comment_node) => comment_node.clone().unchecked_into(),
            DomInner::StatefulComponent { dom_node, .. } => dom_node.as_element(),
        }
    }

    /// return the string content of this symbol
    pub fn as_symbol(&self) -> Option<&str> {
        match &self.inner {
            DomInner::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }

    /// return the outer html string of an element
    pub fn outer_html(&self) -> String {
        let DomInner::Element { element, .. } = &self.inner else {
            unreachable!("should only be called to element");
        };
        element.outer_html()
    }

    /// append the DomNode `child` into this DomNode `self`
    pub fn append_children(&self, for_append: Vec<DomNode>) {
        match &self.inner {
            DomInner::Element {
                element, children, ..
            } => {
                for child in for_append.into_iter() {
                    if let Some(symbol) = child.as_symbol() {
                        element
                            .insert_adjacent_html(intern("beforeend"), symbol)
                            .expect("must not error");
                    } else {
                        element
                            .append_child(&child.as_node())
                            .expect("append child");
                        child.dispatch_mount_event();
                    }
                    children.borrow_mut().push(child);
                }
            }
            DomInner::Fragment {
                fragment, children, ..
            } => {
                for child in for_append.into_iter() {
                    fragment
                        .append_child(&child.as_node())
                        .expect("append child");
                    child.dispatch_mount_event();
                    children.borrow_mut().push(child);
                }
            }
            _ => unreachable!(
                "appending should only be called to Element and Fragment, found: {:#?}",
                self
            ),
        }
    }

    /// Insert the DomNode `for_insert` before `self` DomNode
    pub(crate) fn insert_before(&self, target_element: &DomNode, for_insert: Vec<DomNode>) {
        let DomInner::Element { children, .. } = &self.inner else {
            unreachable!("parent must be an element");
        };

        let mut target_index = None;
        for (i, child) in children.borrow().iter().enumerate() {
            if target_element == child {
                target_index = Some(i);
                break;
            }
        }
        // NOTE: This is not reverse since inserting the last insert_node will always be next
        // before the target element
        for insert_node in for_insert.iter() {
            target_element
                .as_element()
                .insert_adjacent_element(intern("beforebegin"), &insert_node.as_element())
                .expect("must insert before this element");
            insert_node.dispatch_mount_event();
        }

        // NOTE: It is important that we reverse the insertion to the wrapper DomNode since it is
        // just a Vec where inserting from the last will preserve the index to insert into
        for insert_node in for_insert.into_iter().rev() {
            if let Some(target_index) = target_index {
                children.borrow_mut().insert(target_index, insert_node);
            } else {
                unreachable!("should have a self index");
            }
        }
    }

    /// Insert the DomNode `for_insert` after `self` DomNode
    pub(crate) fn insert_after(&self, target_element: &DomNode, for_insert: Vec<DomNode>) {
        let DomInner::Element { children, .. } = &self.inner else {
            unreachable!("parent must be an element");
        };
        let mut target_index = None;
        for (i, child) in children.borrow().iter().enumerate() {
            if target_element == child {
                target_index = Some(i);
                break;
            }
        }
        for insert_node in for_insert.into_iter().rev() {
            target_element
                .as_element()
                .insert_adjacent_element(intern("afterend"), &insert_node.as_element())
                .expect("must insert after this element");
            insert_node.dispatch_mount_event();

            if let Some(target_index) = target_index {
                children.borrow_mut().insert(target_index + 1, insert_node);
            } else {
                unreachable!("should have a self index");
            }
        }
    }

    /// Replace the child `child` DomNode with a replacement DomNode `replacement`
    pub(crate) fn replace_child(&self, target_child: &DomNode, replacement: DomNode) {
        match &self.inner {
            DomInner::Element { children, .. } => {
                let mut child_index = None;
                for (i, ch) in children.borrow().iter().enumerate() {
                    if ch == target_child {
                        child_index = Some(i);
                        break;
                    }
                }
                if let Some(child_index) = child_index {
                    children.borrow_mut().remove(child_index);
                    target_child
                        .as_element()
                        .replace_with_with_node_1(&replacement.as_node())
                        .expect("must replace child");
                    replacement.dispatch_mount_event();
                    children.borrow_mut().insert(child_index, replacement);
                } else {
                    log::warn!(
                        "unable to find target_child: {target_child:#?} with a replacement: {:?}",
                        replacement
                    );
                    // if can not find the child, then must be the root node
                    unreachable!("must find the child...");
                }
            }
            _ => todo!(),
        }
    }

    /// Remove the DomNode `child` from the children of `self`
    pub(crate) fn remove_children(&self, for_remove: &[&DomNode]) {
        match &self.inner {
            DomInner::Element {
                element, children, ..
            } => {
                let mut child_indexes = vec![];
                for (i, ch) in children.borrow().iter().enumerate() {
                    for remove_node in for_remove.iter() {
                        if ch == *remove_node {
                            child_indexes.push(i);
                            break;
                        }
                    }
                }
                assert_eq!(child_indexes.len(), for_remove.len(), "must find all");

                // NOTE: It is important to remove from the last, since
                // vec shifts to the left, while removing from the last
                // with the rev child index, we remove the correct child_index
                for child_index in child_indexes.into_iter().rev() {
                    let child = children.borrow_mut().remove(child_index);
                    element
                        .remove_child(&child.as_node())
                        .expect("remove child");
                }
            }
            _ => todo!(),
        }
    }

    /// remove all the children of this element
    pub(crate) fn clear_children(&self) {
        match &self.inner {
            DomInner::Element {
                element, children, ..
            } => {
                children.borrow_mut().clear();
                // NOTE: It is faster to remove from the last
                // This is removing the children of the actual node
                // regardless if it is mapped with the DomNode wrapper
                while let Some(last_child) = element.last_child() {
                    element
                        .remove_child(&last_child)
                        .expect("must remove child");
                }
            }
            _ => todo!(),
        }
    }

    pub(crate) fn replace_node(&self, replacement: DomNode) {
        //NOTE: This must be replacing a mount node
        self.as_element()
            .replace_with_with_node_1(&replacement.as_node())
            .expect("must replace child");
    }

    /// set the attributes of the dom element
    pub fn set_dom_attrs(&self, attrs: impl IntoIterator<Item = DomAttr>) -> Result<(), JsValue> {
        for attr in attrs.into_iter() {
            self.set_dom_attr(attr)?;
        }
        Ok(())
    }

    /// set the attribute of the dom element
    pub fn set_dom_attr(&self, attr: DomAttr) -> Result<(), JsValue> {
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

                Self::add_event_dom_listeners(element, attr_name, &event_callbacks)
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

                DomAttr::set_element_style(element, attr_name, styles);
                DomAttr::set_element_simple_values(
                    element,
                    attr_name,
                    attr_namespace,
                    plain_values,
                );
            }
            DomInner::StatefulComponent { comp, .. } => {
                log::info!("applying attribute change for stateful component...{attr:?}");
                comp.borrow_mut().attribute_changed(attr);
            }
            _ => {
                log::info!("set the dom attr for {self:?}, with dom_attr: {attr:?}");
                unreachable!("should only be called for element");
            }
        }
        Ok(())
    }

    pub(crate) fn remove_dom_attr(&self, attr: &DomAttr) -> Result<(), JsValue> {
        let DomInner::Element { element, .. } = &self.inner else {
            unreachable!("expecting an element");
        };
        DomAttr::remove_element_dom_attr(element, attr)
    }

    /// attach and event listener to an event target
    pub(crate) fn add_event_dom_listeners(
        target: &web_sys::EventTarget,
        attr_name: &'static str,
        event_listeners: &[EventClosure],
    ) -> Result<(), JsValue> {
        for event_cb in event_listeners.iter() {
            Self::add_event_listener(target, attr_name, event_cb)?;
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

    /// always dispatch the mount event on stateful component
    /// dispatch mount event to element that has on_mount callback set.
    fn should_dispatch_mount_event(&self) -> bool {
        match self.inner {
            DomInner::Element {
                has_mount_callback, ..
            } => has_mount_callback,
            DomInner::StatefulComponent { .. } => true,
            _ => false,
        }
    }

    fn dispatch_mount_event(&self) {
        if self.should_dispatch_mount_event() {
            let event_target: web_sys::EventTarget = self.as_element().unchecked_into();
            event_target
                .dispatch_event(&MountEvent::create_web_event())
                .expect("must be ok");
        }
    }

    #[allow(unused)]
    pub(crate) fn find_child(&self, target_child: &DomNode, path: TreePath) -> Option<TreePath> {
        if self == target_child {
            Some(path)
        } else {
            let children = self.children()?;
            for (i, child) in children.iter().enumerate() {
                let child_path = path.traverse(i);
                let found = child.find_child(target_child, child_path);
                if found.is_some() {
                    return found;
                }
            }
            None
        }
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
                let text = text_node.whole_text().expect("whole text");
                write!(buffer, "{text}")?;
                Ok(())
            }
            DomInner::Comment(comment) => {
                write!(buffer, "<!--{}-->", comment.data())
            }
            DomInner::Symbol(symbol) => {
                write!(buffer, "{symbol}")
            }
            DomInner::Element {
                element, children, ..
            } => {
                let tag = element.tag_name().to_lowercase();
                let is_self_closing = lookup::is_self_closing(&tag);

                write!(buffer, "<{tag}")?;
                let attrs = element.attributes();
                let attrs_len = attrs.length();
                for i in 0..attrs_len {
                    let attr = attrs.item(i).expect("attr");
                    write!(buffer, " {}=\"{}\"", attr.local_name(), attr.value())?;
                }
                if is_self_closing {
                    write!(buffer, "/>")?;
                } else {
                    write!(buffer, ">")?;
                }

                for child in children.borrow().iter() {
                    child.render(buffer)?;
                }
                if !is_self_closing {
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
            DomInner::StatefulComponent { comp: _, dom_node } => {
                dom_node.render(buffer)?;
                Ok(())
            }
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
    pub fn create_dom_node(&self, node: &vdom::Node<APP::MSG>) -> DomNode {
        match node {
            vdom::Node::Element(elm) => self.create_element_node(elm),
            vdom::Node::Leaf(leaf) => self.create_leaf_node(leaf),
        }
    }

    fn create_element_node(&self, elm: &vdom::Element<APP::MSG>) -> DomNode {
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

        let dom_node = DomNode {
            inner: DomInner::Element {
                element,
                listeners: Rc::new(RefCell::new(None)),
                children: Rc::new(RefCell::new(vec![])),
                has_mount_callback: elm.has_mount_callback(),
            },
        };
        let dom_attrs = attrs.iter().map(|a| self.convert_attr(a));
        dom_node.set_dom_attrs(dom_attrs).expect("set dom attrs");
        let children: Vec<DomNode> = elm
            .children()
            .iter()
            .map(|child| self.create_dom_node(child))
            .collect();
        dom_node.append_children(children);
        dom_node
    }

    fn create_leaf_node(&self, leaf: &vdom::Leaf<APP::MSG>) -> DomNode {
        match leaf {
            Leaf::Text(txt) => DomNode {
                inner: DomInner::Text(document().create_text_node(txt)),
            },
            Leaf::Symbol(symbol) => DomNode {
                inner: DomInner::Symbol(symbol.clone()),
            },
            Leaf::Comment(comment) => DomNode {
                inner: DomInner::Comment(document().create_comment(comment)),
            },
            Leaf::Fragment(nodes) => self.create_fragment_node(nodes),
            // NodeList that goes here is only possible when it is the root_node,
            // since node_list as children will be unrolled into as child_elements of the parent
            // We need to wrap this node_list into doc_fragment since root_node is only 1 element
            Leaf::NodeList(nodes) => self.create_fragment_node(nodes),
            Leaf::StatefulComponent(comp) => {
                //TODO: also put the children and attributes here
                DomNode {
                    inner: DomInner::StatefulComponent {
                        comp: Rc::clone(&comp.comp),
                        dom_node: Rc::new(self.create_stateful_component(comp)),
                    },
                }
            }
            Leaf::StatelessComponent(comp) => self.create_stateless_component(comp),
            Leaf::TemplatedView(view) => {
                unreachable!("template view should not be created: {:#?}", view)
            }
            Leaf::DocType(_) => unreachable!("doc type is never converted"),
        }
    }

    fn create_fragment_node<'a>(
        &self,
        nodes: impl IntoIterator<Item = &'a vdom::Node<APP::MSG>>,
    ) -> DomNode {
        let fragment = document().create_document_fragment();
        let dom_node = DomNode {
            inner: DomInner::Fragment {
                fragment,
                children: Rc::new(RefCell::new(vec![])),
            },
        };
        let children = nodes
            .into_iter()
            .map(|node| self.create_dom_node(node))
            .collect();
        dom_node.append_children(children);
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
    fn create_stateful_component(&self, comp: &StatefulModel<APP::MSG>) -> DomNode {
        let comp_node = self.create_dom_node(&crate::html::div(
            [crate::html::attributes::class("component")]
                .into_iter()
                .chain(comp.attrs.clone()),
            [],
        ));

        let dom_attrs: Vec<DomAttr> = comp.attrs.iter().map(|a| self.convert_attr(a)).collect();
        for dom_attr in dom_attrs.into_iter() {
            log::info!("calling attribute changed..");
            comp.comp.borrow_mut().attribute_changed(dom_attr);
        }

        // the component children is manually appended to the StatefulComponent
        // here to allow the conversion of dom nodes with its event
        // listener and removing the generics msg
        let created_children = comp
            .children
            .iter()
            .map(|child| self.create_dom_node(child))
            .collect();
        comp.comp.borrow_mut().append_children(created_children);
        comp_node
    }

    #[allow(unused)]
    pub(crate) fn create_stateless_component(&self, comp: &StatelessModel<APP::MSG>) -> DomNode {
        let comp_view = &comp.view;
        let real_comp_view = comp_view.unwrap_template_ref();
        self.create_dom_node(real_comp_view)
    }
}

/// render the underlying real dom node into string
pub fn render_real_dom_to_string(node: &web_sys::Node) -> String {
    let mut f = String::new();
    render_real_dom(node, &mut f).expect("must not error");
    f
}

/// render the underlying real dom structure
pub fn render_real_dom(node: &web_sys::Node, buffer: &mut dyn fmt::Write) -> fmt::Result {
    match node.node_type() {
        Node::TEXT_NODE => {
            let text_node: &web_sys::Text = node.unchecked_ref();
            let text = text_node.whole_text().expect("whole text");
            write!(buffer, "{text}")?;
            Ok(())
        }
        Node::COMMENT_NODE => {
            let comment: &web_sys::Comment = node.unchecked_ref();
            write!(buffer, "<!--{}-->", comment.data())
        }
        Node::ELEMENT_NODE => {
            let element: &web_sys::Element = node.unchecked_ref();
            let tag = element.tag_name().to_lowercase();
            let is_self_closing = lookup::is_self_closing(&tag);

            write!(buffer, "<{tag}")?;
            let attrs = element.attributes();
            let attrs_len = attrs.length();
            for i in 0..attrs_len {
                let attr = attrs.item(i).expect("attr");
                write!(buffer, " {}=\"{}\"", attr.local_name(), attr.value())?;
            }
            if is_self_closing {
                write!(buffer, "/>")?;
            } else {
                write!(buffer, ">")?;
            }

            let child_nodes = element.child_nodes();
            let child_count = child_nodes.length();
            for i in 0..child_count {
                let child = child_nodes.get(i).unwrap();
                render_real_dom(&child, buffer)?;
            }
            if !is_self_closing {
                write!(buffer, "</{tag}>")?;
            }
            Ok(())
        }
        _ => todo!("for other else"),
    }
}
