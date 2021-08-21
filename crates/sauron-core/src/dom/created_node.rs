use crate::events::MountEvent;
use crate::Listener;
use crate::{
    dom::Dispatch,
    html,
    html::attributes::{AttributeValue, SegregatedAttributes, Special},
    Attribute, Event,
};
use once_cell::sync::OnceCell;
use std::{collections::HashMap, sync::Mutex};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    self, Element, EventTarget, HtmlElement, HtmlInputElement,
    HtmlTextAreaElement, Node, Text,
};

/// This is the value of the data-sauron-vdom-id.
/// Used to uniquely identify elements that contain closures so that the DomUpdater can
/// look them up by their unique id.
/// When the DomUpdater sees that the element no longer exists it will drop all of it's
/// Rc'd Closures for those events.
static DATA_SAURON_VDOM_ID_VALUE: OnceCell<Mutex<u32>> = OnceCell::new();

fn create_unique_identifier() -> u32 {
    let mut elem_unique_id = DATA_SAURON_VDOM_ID_VALUE
        .get_or_init(|| Mutex::new(0))
        .lock()
        .expect("Unable to obtain lock");
    *elem_unique_id += 1;
    *elem_unique_id
}

pub(crate) const DATA_VDOM_ID: &str = "data-vdom-id";

/// Closures that we are holding on to to make sure that they don't get invalidated after a
/// VirtualNode is dropped.
///
/// The u32 is a unique identifier that is associated with the DOM element that this closure is
/// attached to.
///
pub type ActiveClosure =
    HashMap<u32, Vec<(&'static str, Closure<dyn FnMut(web_sys::Event)>)>>;

/// A node along with all of the closures that were created for that
/// node's events and all of it's child node's events.
#[derive(Debug)]
pub struct CreatedNode {
    /// A `Node` or `Element` that was created from a `Node`
    pub node: Node,
    pub(crate) closures: ActiveClosure,
}

impl CreatedNode {
    /// create a simple node with no closure attache
    pub fn without_closures(node: Node) -> Self {
        CreatedNode {
            node,
            closures: HashMap::with_capacity(0),
        }
    }

    /// create a text node
    pub fn create_text_node(txt: &str) -> Text {
        crate::document().create_text_node(txt)
    }

    /// Create and return a `CreatedNode` instance (containing a DOM `Node`
    /// together with potentially related closures) for this virtual node.
    pub fn create_dom_node<DSP, MSG>(
        program: &DSP,
        vnode: &crate::Node<MSG>,
        focused_node: &mut Option<Node>,
    ) -> CreatedNode
    where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        match vnode {
            crate::Node::Text(txt) => {
                let text_node = Self::create_text_node(&txt.text);
                CreatedNode::without_closures(text_node.unchecked_into())
            }
            crate::Node::Element(element_node) => {
                Self::create_element_node(program, element_node, focused_node)
            }
        }
    }

    /// dispatch the mount event,
    /// call the listener since browser don't allow asynchronous execution of
    /// dispatching custom events (non-native browser events)
    fn dispatch_mount_event<DSP, MSG>(
        program: &DSP,
        velem: &crate::Element<MSG>,
        element: &Element,
    ) where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        for att in velem.attrs.iter() {
            if *att.name() == "mount" {
                for val in att.value().iter() {
                    if let AttributeValue::EventListener(cb) = val {
                        let msg = cb.emit(Event::from(MountEvent {
                            target_node: element.clone().unchecked_into(),
                        }));
                        program.dispatch(msg);
                    }
                }
            }
        }
    }

    /// Build a DOM element by recursively creating DOM nodes for this element and it's
    /// children, it's children's children, etc.
    fn create_element_node<DSP, MSG>(
        program: &DSP,
        velem: &crate::Element<MSG>,
        focused_node: &mut Option<Node>,
    ) -> CreatedNode
    where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        let document = crate::document();

        let element = if let Some(namespace) = velem.namespace() {
            document
                .create_element_ns(Some(namespace), velem.tag())
                .expect("Unable to create element")
        } else {
            document
                .create_element(velem.tag())
                .expect("Unable to create element")
        };

        Self::dispatch_mount_event(program, velem, &element);

        if velem.is_focused() {
            *focused_node = Some(element.clone().unchecked_into());
            log::trace!("element is focused..{:?}", focused_node);
        }

        let mut closures = ActiveClosure::new();

        Self::set_element_attributes(
            program,
            &mut closures,
            &element,
            &velem.get_attributes().iter().collect::<Vec<_>>(),
        );

        let mut previous_node_was_text = false;

        for child in velem.get_children().iter() {
            match child {
                crate::Node::Text(txt) => {
                    let current_node: &web_sys::Node = element.as_ref();

                    // We ensure that the text siblings are patched by preventing the browser from merging
                    // neighboring text nodes. Originally inspired by some of React's work from 2016.
                    //  -> https://reactjs.org/blog/2016/04/07/react-v15.html#major-changes
                    //  -> https://github.com/facebook/react/pull/5753
                    //
                    // `mordor` one does not simply walk into mordor
                    if previous_node_was_text {
                        let separator = document.create_comment("mordor");
                        current_node
                            .append_child(separator.as_ref())
                            .expect("Unable to append child");
                    }

                    if txt.safe_html {
                        // https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML
                        element
                            .insert_adjacent_html("beforeend", &txt.text)
                            .expect("must not error");
                    } else {
                        let text_node = Self::create_text_node(&txt.text);
                        current_node
                            .append_child(&text_node)
                            .expect("Unable to append text node");
                    }

                    previous_node_was_text = true;
                }
                crate::Node::Element(_element_node) => {
                    previous_node_was_text = false;

                    let created_child =
                        Self::create_dom_node(program, child, focused_node);
                    closures.extend(created_child.closures);

                    element
                        .append_child(&created_child.node)
                        .expect("Unable to append element node");
                }
            }
        }

        let node: Node = element.unchecked_into();
        CreatedNode { node, closures }
    }

    /// set the element attribute
    pub fn set_element_attributes<DSP, MSG>(
        program: &DSP,
        closures: &mut ActiveClosure,
        element: &Element,
        attrs: &[&Attribute<MSG>],
    ) where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        let attrs = mt_dom::merge_attributes_of_same_name(attrs);
        for att in attrs {
            Self::set_element_attribute(program, closures, element, &att);
        }
    }

    /// set the element attribute
    pub fn set_element_attribute<DSP, MSG>(
        program: &DSP,
        closures: &mut ActiveClosure,
        element: &Element,
        attr: &Attribute<MSG>,
    ) where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        let SegregatedAttributes {
            listeners,
            plain_values,
            styles,
            function_calls,
        } =
            html::attributes::partition_callbacks_from_plain_styles_and_func_calls(
                attr,
            );

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
                        Some(namespace),
                        attr.name(),
                        &merged_plain_values,
                    )
                    .unwrap_or_else(|_| {
                        panic!(
                            "Error setting an attribute_ns for {:?}",
                            element
                        )
                    });
            } else {
                match *attr.name() {
                    // we explicitly call the `set_value` function in the html element
                    "value" => {
                        if let Some(input) =
                            element.dyn_ref::<HtmlInputElement>()
                        {
                            input.set_value(&merged_plain_values);
                        } else if let Some(textarea) =
                            element.dyn_ref::<HtmlTextAreaElement>()
                        {
                            textarea.set_value(&merged_plain_values);
                        }
                    }
                    // we explicitly call `set_checked` function on the html element
                    "checked" => {
                        if let Some(input) =
                            element.dyn_ref::<HtmlInputElement>()
                        {
                            let checked: bool = plain_values
                                .first()
                                .map(|av| {
                                    av.get_simple()
                                        .map(|v| v.as_bool())
                                        .flatten()
                                })
                                .flatten()
                                .unwrap_or(false);

                            input.set_checked(checked);
                        }
                    }
                    _ => {
                        element
                            .set_attribute(attr.name(), &merged_plain_values)
                            .unwrap_or_else(|_| {
                                panic!(
                                    "Error setting an attribute for {:?}",
                                    element
                                )
                            });
                    }
                }
            }
        } else if let Some(merged_styles) =
            html::attributes::merge_styles_attributes_values(&styles)
        {
            // set the styles
            element
                .set_attribute(attr.name(), &merged_styles)
                .unwrap_or_else(|_| {
                    panic!("Error setting an attribute_ns for {:?}", element)
                });
        } else {
            //if the merged attribute is blank of empty when string is trimmed
            //remove the attribute
            element
                .remove_attribute(attr.name())
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
                .set_attribute(DATA_VDOM_ID, &unique_id.to_string())
                .expect("Could not set attribute on element");

            closures.insert(unique_id, vec![]);

            let event_str = attr.name();
            let current_elm: &EventTarget =
                element.dyn_ref().expect("unable to cast to event targe");

            // a custom enter event which triggers the listener
            // when the enter key is pressed
            if *event_str == "enter" {
                let program_clone = program.clone();
                let listener_clone = listener.clone();
                let key_press_func: Closure<dyn FnMut(web_sys::Event)> =
                    Closure::wrap(Box::new(move |event: web_sys::Event| {
                        let ke: &web_sys::KeyboardEvent = event
                            .dyn_ref()
                            .expect("should be a keyboard event");
                        if ke.key() == "Enter" {
                            let msg = listener_clone.emit(Event::from(event));
                            program_clone.dispatch(msg);
                        }
                    }));

                current_elm
                    .add_event_listener_with_callback(
                        "keypress",
                        key_press_func.as_ref().unchecked_ref(),
                    )
                    .expect("unable to attach enter event listener");

                key_press_func.forget();
            } else {
                let callback_wrapped: Closure<dyn FnMut(web_sys::Event)> =
                    create_closure_wrap(program, listener);
                current_elm
                    .add_event_listener_with_callback(
                        event_str,
                        callback_wrapped.as_ref().unchecked_ref(),
                    )
                    .expect("Unable to attached event listener");
                closures
                    .get_mut(&unique_id)
                    .expect("Unable to get closure")
                    .push((event_str, callback_wrapped));
            }
        }
    }

    /// set focus to this element
    pub(crate) fn set_element_focus(element: &Element) {
        let html_element: &HtmlElement = element.unchecked_ref();
        html_element.focus().expect("must focus")
    }

    /// remove element attribute,
    /// takes care of special case such as checked
    pub fn remove_element_attribute<MSG>(
        element: &Element,
        attr: &Attribute<MSG>,
    ) -> Result<(), JsValue> {
        log::trace!("removing attribute: {}", attr.name());

        element.remove_attribute(attr.name())?;

        if *attr.name() == "checked" {
            if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
                input.set_checked(false);
            }
        }
        Ok(())
    }
}

/// This wrap into a closure the function that is dispatched when the event is triggered.
pub(crate) fn create_closure_wrap<DSP, MSG>(
    program: &DSP,
    listener: &Listener<MSG>,
) -> Closure<dyn FnMut(web_sys::Event)>
where
    MSG: 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    let listener_clone = listener.clone();
    let program_clone = program.clone();

    Closure::wrap(Box::new(move |event: web_sys::Event| {
        let msg = listener_clone.emit(Event::from(event));
        program_clone.dispatch(msg);
    }))
}
