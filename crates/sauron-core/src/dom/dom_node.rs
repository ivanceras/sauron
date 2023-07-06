use crate::{
    dom::{Application, Event, Program},
    events::MountEvent,
    html,
    html::attributes::{AttributeValue, SegregatedAttributes},
    vdom,
    vdom::{Attribute, Leaf, Listener, NodeTrait},
};
use js_sys::Function;
use mt_dom::TreePath;
use std::collections::HashMap;
use std::{cell::Cell, collections::BTreeMap};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    self, Element, EventTarget, HtmlButtonElement, HtmlDataElement, HtmlDetailsElement,
    HtmlFieldSetElement, HtmlInputElement, HtmlLiElement, HtmlLinkElement, HtmlMenuItemElement,
    HtmlMeterElement, HtmlOptGroupElement, HtmlOptionElement, HtmlOutputElement, HtmlParamElement,
    HtmlProgressElement, HtmlSelectElement, HtmlStyleElement, HtmlTextAreaElement, Node, Text,
};

/// data attribute name used in assigning the node id of an element with events
pub(crate) const DATA_VDOM_ID: &str = "data-vdom-id";

thread_local!(static NODE_ID_COUNTER: Cell<usize> = Cell::new(1));

// a cache of commonly used elements, so we can clone them.
// cloning is much faster then creating the element
thread_local! {
    static CACHE_ELEMENTS: HashMap<&'static str, web_sys::Element> =
        HashMap::from_iter(["div", "span", "ol", "ul", "li"].map(create_element_with_tag));
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

fn create_element_with_tag(tag: &'static str) -> (&'static str, web_sys::Element) {
    let elm = crate::document().create_element(intern(tag)).unwrap();
    (tag, elm)
}

/// find the element from the most created element and clone it, else create it
/// TODO: feature gate this with `use-cached-elements`
fn create_element(tag: &'static str) -> web_sys::Element {
    CACHE_ELEMENTS.with(|map| {
        if let Some(elm) = map.get(tag) {
            elm.clone_node_with_deep(false)
                .expect("must clone node")
                .unchecked_into()
        } else {
            let elm = crate::document().create_element(intern(tag)).unwrap();
            elm
        }
    })
}

/// This is the value of the data-sauron-vdom-id.
/// Used to uniquely identify elements that contain closures so that the DomUpdater can
/// look them up by their unique id.
/// When the DomUpdater sees that the element no longer exists it will drop all of it's
/// Rc'd Closures for those events.
fn create_unique_identifier() -> usize {
    NODE_ID_COUNTER.with(|x| {
        let tmp = x.get();
        x.set(tmp + 1);
        tmp
    })
}

/// A node along with all of the closures that were created for that
/// node's events and all of it's child node's events.
impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    /// create a text node
    pub fn create_text_node(txt: &str) -> Text {
        crate::document().create_text_node(txt)
    }

    fn create_leaf_node(&self, leaf: &Leaf<MSG>) -> Node {
        match leaf {
            Leaf::Text(txt) => Self::create_text_node(txt).into(),
            Leaf::Comment(comment) => crate::document().create_comment(comment).into(),
            Leaf::SafeHtml(_safe_html) => {
                panic!("safe html must have already been dealt in create_element node");
            }
            Leaf::DocType(_doctype) => {
                panic!(
                    "It looks like you are using doctype in the middle of an app,
                    doctype is only used in rendering"
                );
            }
            Leaf::Fragment(nodes) => {
                let document = crate::document();
                let doc_fragment = document.create_document_fragment();
                for vnode in nodes {
                    let created_node = self.create_dom_node(vnode);
                    Self::append_child_and_dispatch_mount_event(&doc_fragment, &created_node)
                }
                doc_fragment.into()
            }
        }
    }

    /// Create and return a `CreatedNode` instance (containing a DOM `Node`
    /// together with potentially related closures) for this virtual node.
    pub fn create_dom_node(&self, vnode: &vdom::Node<MSG>) -> Node {
        match vnode {
            vdom::Node::Leaf(leaf_node) => self.create_leaf_node(leaf_node),
            vdom::Node::Element(element_node) => {
                let created_node = self.create_element_node(element_node);
                for child in element_node.get_children().iter() {
                    if child.is_safe_html() {
                        let child_text = child.unwrap_safe_html();
                        // https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML
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
            vdom::Node::NodeList(_node_list) => {
                panic!("Node list must have already been unrolled");
            }
        }
    }

    fn is_custom_element(tag: &str) -> bool {
        let custom_element = crate::window().custom_elements();
        let existing = custom_element.get(intern(tag));
        // define the custom element only when it is not yet defined
        !existing.is_undefined()
    }

    /// Build a DOM element by recursively creating DOM nodes for this element and it's
    /// children, it's children's children, etc.
    fn create_element_node(&self, velem: &vdom::Element<MSG>) -> Node {
        let document = crate::document();

        if Self::is_custom_element(velem.tag()) {
            //log::info!("This is a custom element: {}", velem.tag());
        }

        let element = if let Some(namespace) = velem.namespace() {
            document
                .create_element_ns(Some(intern(namespace)), intern(velem.tag()))
                .expect("Unable to create element")
        } else {
            create_element(velem.tag())
        };

        Self::set_element_attributes(
            self,
            &element,
            &velem.get_attributes().iter().collect::<Vec<_>>(),
        );

        element.into()
    }

    /// dispatch the mount event,
    /// call the listener since browser don't allow asynchronous execution of
    /// dispatching custom events (non-native browser events)
    ///
    pub fn dispatch_mount_event(node: &Node) {
        let event_target: &web_sys::EventTarget = node.unchecked_ref();
        assert_eq!(
            Ok(true),
            event_target.dispatch_event(&MountEvent::create_web_event())
        );
    }

    /// a helper method to append a node to its parent and trigger a mount event if there is any
    pub fn append_child_and_dispatch_mount_event(parent: &Node, child_node: &Node) {
        parent
            .append_child(child_node)
            .expect("must append child node");
        Self::dispatch_mount_event(child_node);
    }

    /// set the element attribute
    pub fn set_element_attributes(&self, element: &Element, attrs: &[&Attribute<MSG>]) {
        let attrs = mt_dom::merge_attributes_of_same_name(attrs);
        for att in attrs {
            self.set_element_attribute(element, &att);
        }
    }

    /// set the element attribute
    ///
    /// Note: this is called in a loop, so setting the attributes, and style will not be on
    /// the same call, but on a subsequent call to each other. Using the if-else-if here for
    /// attributes, style, function_call.
    pub fn set_element_attribute(&self, element: &Element, attr: &Attribute<MSG>) {
        let SegregatedAttributes {
            listeners,
            plain_values,
            styles,
            function_calls,
        } = html::attributes::partition_callbacks_from_plain_styles_and_func_calls(attr);

        // set simple values
        if let Some(merged_plain_values) =
            html::attributes::merge_plain_attributes_values(&plain_values)
        {
            if let Some(namespace) = attr.namespace() {
                // Warning NOTE: set_attribute_ns should only be called
                // when you meant to use a namespace
                // using this with None will error in the browser with:
                // NamespaceError: An attempt was made to create or change an object in a way which is incorrect with regard to namespaces
                element
                    .set_attribute_ns(
                        Some(intern(namespace)),
                        intern(attr.name()),
                        &merged_plain_values,
                    )
                    .unwrap_or_else(|_| panic!("Error setting an attribute_ns for {element:?}"));
            } else {
                let attr_name = attr.name();
                match *attr_name {
                    "value" => {
                        log::info!("setting value for {}", element.outer_html());
                        element
                            .set_attribute(intern(attr_name), &merged_plain_values)
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                        Self::set_value_str(element, &merged_plain_values);
                        Self::set_numeric_values(element, &plain_values);
                    }
                    "open" => {
                        let is_open: bool = plain_values
                            .first()
                            .and_then(|v| v.get_simple().and_then(|v| v.as_bool()))
                            .unwrap_or(false);

                        element
                            .set_attribute(intern(attr.name()), &is_open.to_string())
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                        Self::set_open(element, is_open);
                    }
                    "checked" => {
                        let is_checked: bool = plain_values
                            .first()
                            .and_then(|av| av.get_simple().and_then(|v| v.as_bool()))
                            .unwrap_or(false);

                        element
                            .set_attribute(intern(attr_name), &is_checked.to_string())
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                        Self::set_checked(element, is_checked)
                    }
                    "disabled" => {
                        let is_disabled: bool = plain_values
                            .first()
                            .and_then(|av| av.get_simple().and_then(|v| v.as_bool()))
                            .unwrap_or(false);

                        element
                            .set_attribute(intern(attr_name), &is_disabled.to_string())
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                        Self::set_disabled(element, is_disabled);
                    }
                    _ => {
                        element
                            .set_attribute(intern(attr_name), &merged_plain_values)
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                    }
                }
            }
        } else if let Some(merged_styles) =
            html::attributes::merge_styles_attributes_values(&styles)
        {
            // set the styles
            element
                .set_attribute(intern(attr.name()), &merged_styles)
                .unwrap_or_else(|_| panic!("Error setting an attribute_ns for {element:?}"));
        } else {
            //if the merged attribute is blank of empty when string is trimmed
            //remove the attribute
            element
                .remove_attribute(intern(attr.name()))
                .expect("must remove attribute");
        }

        // do function calls such as set_inner_html
        if let Some(merged_func_values) =
            html::attributes::merge_plain_attributes_values(&function_calls)
        {
            if *attr.name() == "inner_html" {
                element.set_inner_html(&merged_func_values);
            }
        }

        // add listeners using add_event_listener
        for listener in listeners {
            let unique_id = create_unique_identifier();

            // set the data-sauron_vdom-id this will be read later on
            // when it's time to remove this element and its closures and event listeners
            element
                .set_attribute(intern(DATA_VDOM_ID), &unique_id.to_string())
                .expect("Could not set attribute on element");

            let mut node_closures = self.node_closures.borrow_mut();
            node_closures.insert(unique_id, vec![]);

            let event_str = attr.name();
            let current_elm: &EventTarget =
                element.dyn_ref().expect("unable to cast to event targe");

            // This is where all of the UI events is wired in this part of the code.
            // All event listener is added to this element.
            // The callback to this listener emits an Msg which is then \
            // dispatched to the `program` which then triggers update view cycle.
            let callback_wrapped: Closure<dyn FnMut(web_sys::Event)> =
                self.create_closure_wrap(listener);
            current_elm
                .add_event_listener_with_callback(
                    intern(event_str),
                    callback_wrapped.as_ref().unchecked_ref(),
                )
                .expect("Unable to attached event listener");

            node_closures
                .get_mut(&unique_id)
                .expect("Unable to get closure")
                .push((event_str, callback_wrapped));
        }
    }

    /// explicitly call `set_checked` function on the html element
    /// since setting the attribute to false will not unchecked it.
    ///
    /// There are only 2 elements where set_checked is applicable:
    /// - input
    /// - menuitem
    fn set_checked(element: &Element, is_checked: bool) {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_checked(is_checked);
        } else if let Some(menu_item) = element.dyn_ref::<HtmlMenuItemElement>() {
            menu_item.set_checked(is_checked);
        }
    }

    /// explicitly call set_open for details
    /// since setting the attribute `open` to false will not close it.
    ///
    /// TODO: HtmlDialogElement ( but it is not supported on firefox and in safari, only works on chrome)
    ///
    /// Applies to:
    ///  - dialog
    ///  - details
    fn set_open(element: &Element, is_open: bool) {
        if let Some(details) = element.dyn_ref::<HtmlDetailsElement>() {
            details.set_open(is_open);
        }
    }

    /// explicitly call on `set_disabled`
    /// since setting the attribute `disabled` false will not enable it.
    ///
    /// These are 10 elements that we can call `set_disabled` function to.
    /// - input
    /// - button
    /// - textarea
    /// - style
    /// - link
    /// - select
    /// - option
    /// - optgroup
    /// - fieldset
    /// - menuitem
    ///
    /// TODO: use macro to simplify this code
    fn set_disabled(element: &Element, is_disabled: bool) {
        if let Some(elm) = element.dyn_ref::<HtmlInputElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlButtonElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlTextAreaElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlStyleElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlLinkElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlSelectElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptionElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptGroupElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlFieldSetElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlMenuItemElement>() {
            elm.set_disabled(is_disabled);
        }
    }

    /// we explicitly call the `set_value` function in the html element
    ///
    /// TODO: use macro to simplify this code
    fn set_value_str(element: &Element, value: &str) {
        if let Some(elm) = element.dyn_ref::<HtmlInputElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlTextAreaElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlSelectElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptionElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlButtonElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlDataElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlOutputElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlParamElement>() {
            elm.set_value(value);
        }
    }

    fn set_value_i32(element: &Element, value: i32) {
        if let Some(elm) = element.dyn_ref::<HtmlLiElement>() {
            elm.set_value(value);
        }
    }

    fn set_value_f64(element: &Element, value: f64) {
        if let Some(elm) = element.dyn_ref::<HtmlMeterElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlProgressElement>() {
            elm.set_value(value);
        }
    }

    /// set the element attribute value with the first numerical value found in values
    fn set_numeric_values(element: &Element, values: &[&AttributeValue<MSG>]) {
        let value_i32 = values
            .first()
            .and_then(|v| v.get_simple().and_then(|v| v.as_i32()));

        let value_f64 = values
            .first()
            .and_then(|v| v.get_simple().and_then(|v| v.as_f64()));

        if let Some(value_i32) = value_i32 {
            Self::set_value_i32(element, value_i32);
        }
        if let Some(value_f64) = value_f64 {
            Self::set_value_f64(element, value_f64);
        }
    }

    /// remove element attribute,
    /// takes care of special case such as checked
    pub fn remove_element_attribute(
        element: &Element,
        attr: &Attribute<MSG>,
    ) -> Result<(), JsValue> {
        log::trace!("removing attribute: {}", attr.name());

        match *attr.name() {
            "value" => {
                Self::set_value_str(element, "");
            }
            "open" => {
                Self::set_open(element, false);
            }
            "checked" => {
                Self::set_checked(element, false);
            }
            "disabled" => {
                Self::set_disabled(element, false);
            }
            _ => (),
        }
        //actually remove the element
        element.remove_attribute(intern(attr.name()))?;

        Ok(())
    }

    /// remove all the event listeners for this node
    pub(crate) fn remove_event_listeners(&self, node: &Element) -> Result<(), JsValue> {
        let all_descendant_vdom_id = get_node_descendant_data_vdom_id(node);
        let mut node_closures = self.node_closures.borrow_mut();
        for vdom_id in all_descendant_vdom_id {
            if let Some(old_closure) = node_closures.get(&vdom_id) {
                for (event, oc) in old_closure.iter() {
                    let func: &Function = oc.as_ref().unchecked_ref();
                    node.remove_event_listener_with_callback(intern(event), func)?;
                }
                // remove closure active_closure in dom_updater to free up memory
                node_closures
                    .remove(&vdom_id)
                    .expect("Unable to remove old closure");
            } else {
                log::warn!("There is no closure marked with that vdom_id: {}", vdom_id);
            }
        }
        Ok(())
    }

    /// remove the event listener which matches the given event name
    pub(crate) fn remove_event_listener_with_name(
        &self,
        event_name: &'static str,
        node: &Element,
    ) -> Result<(), JsValue> {
        let all_descendant_vdom_id = get_node_descendant_data_vdom_id(node);
        let mut node_closures = self.node_closures.borrow_mut();
        for vdom_id in all_descendant_vdom_id {
            if let Some(old_closure) = node_closures.get_mut(&vdom_id) {
                for (event, oc) in old_closure.iter() {
                    if *event == event_name {
                        let func: &Function = oc.as_ref().unchecked_ref();
                        node.remove_event_listener_with_callback(intern(event), func)?;
                    }
                }

                old_closure.retain(|(event, _oc)| *event != event_name);

                // remove closure active_closure in dom_updater to free up memory
                if old_closure.is_empty() {
                    node_closures
                        .remove(&vdom_id)
                        .expect("Unable to remove old closure");
                }
            } else {
                log::warn!("There is no closure marked with that vdom_id: {}", vdom_id);
            }
        }
        Ok(())
    }

    /// This wrap into a closure the function that is dispatched when the event is triggered.
    pub(crate) fn create_closure_wrap(
        &self,
        listener: &Listener<MSG>,
    ) -> Closure<dyn FnMut(web_sys::Event)> {
        let listener_clone = listener.clone();
        let program = self.clone();

        Closure::wrap(Box::new(move |event: web_sys::Event| {
            let msg = listener_clone.emit(Event::from(event));
            program.dispatch(msg);
        }))
    }
}

fn find_node(node: &Node, path: &mut TreePath) -> Option<Node> {
    if path.is_empty() {
        Some(node.clone())
    } else {
        let idx = path.remove_first();
        let children = node.child_nodes();
        if let Some(child) = children.item(idx as u32) {
            find_node(&child, path)
        } else {
            None
        }
    }
}

pub(crate) fn find_all_nodes(
    node: &Node,
    nodes_to_find: &[(&TreePath, Option<&&'static str>)],
) -> BTreeMap<TreePath, Node> {
    let mut nodes_to_patch: BTreeMap<TreePath, Node> = BTreeMap::new();

    for (path, tag) in nodes_to_find {
        let mut traverse_path: TreePath = (*path).clone();
        if let Some(found) = find_node(node, &mut traverse_path) {
            nodes_to_patch.insert((*path).clone(), found);
        } else {
            log::warn!("can not find: {:?} {:?} root_node: {:?}", path, tag, node);
        }
    }
    nodes_to_patch
}

/// Get the "data-vdom-id" of all the desendent of this node including itself
/// This is needed to free-up the closure that was attached ActiveClosure manually
fn get_node_descendant_data_vdom_id(root_element: &Element) -> Vec<usize> {
    let mut data_vdom_id = vec![];

    if let Some(vdom_id_str) = root_element.get_attribute(intern(DATA_VDOM_ID)) {
        let vdom_id = vdom_id_str
            .parse::<usize>()
            .expect("unable to parse sauron_vdom-id");
        data_vdom_id.push(vdom_id);
    }

    let children = root_element.child_nodes();
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
