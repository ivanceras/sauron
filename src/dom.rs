use crate::Dispatch;
use apply_patches::patch;
use sauron_vdom::{
    self,
    diff,
    Callback,
};
use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
    sync::Mutex,
};
use wasm_bindgen::{
    closure::Closure,
    JsCast,
};
use web_sys::{
    self,
    Element,
    EventTarget,
    Node,
    Text,
};

mod apply_patches;

// Used to uniquely identify elements that contain closures so that the DomUpdater can
// look them up by their unique id.
// When the DomUpdater sees that the element no longer exists it will drop all of it's
// Rc'd Closures for those events.
use lazy_static::lazy_static;
lazy_static! {
    static ref ELEM_UNIQUE_ID: Mutex<u32> = Mutex::new(0);
}

pub(self) const DATA_SAURON_VDOM_ID: &str = "data-sauron-vdom-id";

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
pub struct CreatedNode<T> {
    /// A `Node` or `Element` that was created from a `Node`
    pub node: T,
    closures: ActiveClosure,
}

/// Used for keeping a real DOM node up to date based on the current Node
/// and a new incoming Node that represents our latest DOM state.
pub struct DomUpdater<DSP, MSG>
where
    MSG: 'static,
{
    current_vdom: crate::Node<MSG>,
    root_node: Node,

    /// The closures that are currently attached to elements in the page.
    ///
    /// We keep these around so that they don't get dropped (and thus stop working);
    ///
    pub active_closures: ActiveClosure,
    _phantom_dsp: PhantomData<DSP>,
}

impl<T> CreatedNode<T> {
    pub fn without_closures<N: Into<T>>(node: N) -> Self {
        CreatedNode {
            node: node.into(),
            closures: HashMap::with_capacity(0),
        }
    }

    pub fn create_text_node(text: &sauron_vdom::Text) -> Text {
        crate::document().create_text_node(&text.text)
    }

    /// Create and return a `CreatedNode` instance (containing a DOM `Node`
    /// together with potentially related closures) for this virtual node.
    pub fn create_dom_node<DSP, MSG>(
        program: &Rc<DSP>,
        vnode: &crate::Node<MSG>,
    ) -> CreatedNode<Node>
    where
        MSG: 'static,
        DSP: Dispatch<MSG> + 'static,
    {
        match vnode {
            crate::Node::Text(text_node) => {
                CreatedNode::without_closures(Self::create_text_node(text_node))
            }
            crate::Node::Element(element_node) => {
                let created_element: CreatedNode<Node> =
                    Self::create_element_node(program, element_node).into();
                created_element
            }
        }
    }

    /// Build a DOM element by recursively creating DOM nodes for this element and it's
    /// children, it's children's children, etc.
    pub fn create_element_node<DSP, MSG>(
        program: &Rc<DSP>,
        velem: &crate::Element<MSG>,
    ) -> CreatedNode<Element>
    where
        MSG: 'static,
        DSP: Dispatch<MSG> + 'static,
    {
        let document = crate::document();

        let element = if let Some(ref namespace) = velem.namespace {
            document
                .create_element_ns(Some(namespace), &velem.tag)
                .expect("Unable to create element")
        } else {
            document
                .create_element(&velem.tag)
                .expect("Unable to create element")
        };

        let mut closures = ActiveClosure::new();

        velem.attributes().iter().for_each(|attr| {
            element
                .set_attribute(attr.name, &attr.value.to_string())
                .expect("Set element attribute in create element");
        });

        if !velem.events().is_empty() {
            let unique_id = create_unique_identifier();

            // set the data-sauron_vdom-id this will be read later on
            // when it's time to remove this element and its closures and event listeners
            element
                .set_attribute(DATA_SAURON_VDOM_ID, &unique_id.to_string())
                .expect("Could not set attribute on element");

            closures.insert(unique_id, vec![]);

            for event_attr in velem.events().iter() {
                let event_str = event_attr.name;
                let callback = event_attr
                    .value
                    .get_callback()
                    .expect("expecting a callback");
                let current_elm: &EventTarget =
                    element.dyn_ref().expect("unable to cast to event targe");
                let closure_wrap: Closure<dyn FnMut(web_sys::Event)> =
                    create_closure_wrap(program, &callback);
                current_elm
                    .add_event_listener_with_callback(
                        event_str,
                        closure_wrap.as_ref().unchecked_ref(),
                    )
                    .expect("Unable to attached event listener");
                closures
                    .get_mut(&unique_id)
                    .expect("Unable to get closure")
                    .push((event_str, closure_wrap));
            }
        }

        let mut previous_node_was_text = false;
        for child in velem.children.iter() {
            match child {
                crate::Node::Text(text_node) => {
                    let current_node = element.as_ref() as &web_sys::Node;

                    // We ensure that the text siblings are patched by preventing the browser from merging
                    // neighboring text nodes. Originally inspired by some of React's work from 2016.
                    //  -> https://reactjs.org/blog/2016/04/07/react-v15.html#major-changes
                    //  -> https://github.com/facebook/react/pull/5753
                    //
                    // `ptns` = Percy text node separator
                    if previous_node_was_text {
                        let separator = document.create_comment("ptns");
                        current_node
                            .append_child(separator.as_ref() as &web_sys::Node)
                            .expect("Unable to append child");
                    }

                    current_node
                        .append_child(&Self::create_text_node(&text_node))
                        .expect("Unable to append text node");

                    previous_node_was_text = true;
                }
                crate::Node::Element(element_node) => {
                    previous_node_was_text = false;

                    let child =
                        Self::create_element_node(program, element_node);
                    let child_elem: Element = child.node;
                    closures.extend(child.closures);

                    element
                        .append_child(&child_elem)
                        .expect("Unable to append element node");
                }
            }
        }

        CreatedNode {
            node: element,
            closures,
        }
    }
}

/// This wrap into a closure the function that is dispatched when the event is triggered.
fn create_closure_wrap<DSP, MSG>(
    program: &Rc<DSP>,
    callback: &Callback<crate::Event, MSG>,
) -> Closure<dyn FnMut(web_sys::Event)>
where
    MSG: 'static,
    DSP: Dispatch<MSG> + 'static + 'static,
{
    let callback_clone = callback.clone();
    let program_clone = Rc::clone(&program);

    Closure::wrap(Box::new(move |event: web_sys::Event| {
        // stop propagation to the containers of this element to have
        // a more fine grain control and expected results
        event.stop_propagation();
        // prevent the reloading the page in href links
        event.prevent_default();
        let cb_event = crate::Event(event);
        let msg = callback_clone.emit(cb_event);
        program_clone.dispatch(msg);
    }))
}

impl<DSP, MSG> DomUpdater<DSP, MSG>
where
    MSG: 'static,
    DSP: Dispatch<MSG> + 'static,
{
    /// Creates and instance of this DOM updater, but doesn't mount the current_vdom to the DOM just yet.
    pub fn new(
        current_vdom: crate::Node<MSG>,
        root_node: &Node,
    ) -> DomUpdater<DSP, MSG> {
        DomUpdater {
            current_vdom,
            root_node: root_node.clone(),
            active_closures: ActiveClosure::new(),
            _phantom_dsp: PhantomData,
        }
    }

    /// count the total active closures
    /// regardless of which element it attached to.
    pub fn active_closure_len(&self) -> usize {
        self.active_closures
            .iter()
            .map(|(_elm_id, closures)| closures.len())
            .sum()
    }

    /// Mount the current_vdom appending to the actual browser DOM specified in the root_node
    /// This also gets the closures that was created when mounting the vdom to their
    /// actual DOM counterparts.
    pub fn append_to_mount(&mut self, program: &Rc<DSP>) {
        let created_node: CreatedNode<Node> =
            CreatedNode::<Node>::create_dom_node(program, &self.current_vdom);
        self.root_node
            .append_child(&created_node.node)
            .expect("Could not append child to mount");
        self.root_node = created_node.node;
        self.active_closures = created_node.closures;
    }

    /// Mount the current_vdom replacing the actual browser DOM specified in the root_node
    /// This also gets the closures that was created when mounting the vdom to their
    /// actual DOM counterparts.
    pub fn replace_mount(&mut self, program: &Rc<DSP>) {
        let created_node: CreatedNode<Node> =
            CreatedNode::<Node>::create_dom_node(program, &self.current_vdom);
        let root_element: &Element = self.root_node.unchecked_ref();
        root_element
            .replace_with_with_node_1(&created_node.node)
            .expect("Could not append child to mount");
        self.root_node = created_node.node;
        self.active_closures = created_node.closures;
    }

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and appended (as a child) to your passed
    /// in mount element.
    pub fn new_append_to_mount(
        program: &Rc<DSP>,
        current_vdom: crate::Node<MSG>,
        mount: &Element,
    ) -> DomUpdater<DSP, MSG> {
        let mut dom_updater = Self::new(current_vdom, mount);
        dom_updater.append_to_mount(program);
        dom_updater
    }

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and it will replace your passed in mount
    /// element.
    pub fn new_replace_mount(
        program: &Rc<DSP>,
        current_vdom: crate::Node<MSG>,
        mount: Element,
    ) -> DomUpdater<DSP, MSG> {
        let mut dom_updater = Self::new(current_vdom, &mount);
        dom_updater.replace_mount(program);
        dom_updater
    }

    /// Diff the current virtual dom with the new virtual dom that is being passed in.
    ///
    /// Then use that diff to patch the real DOM in the user's browser so that they are
    /// seeing the latest state of the application.
    pub fn update_dom(
        &mut self,
        program: &Rc<DSP>,
        new_vdom: crate::Node<MSG>,
    ) {
        let patches = diff(&self.current_vdom, &new_vdom);
        let active_closures = patch(
            program,
            self.root_node.clone(),
            &mut self.active_closures,
            &patches,
        )
        .expect("Error in patching the dom");
        self.active_closures.extend(active_closures);
        self.current_vdom = new_vdom;
    }

    /// Return the root node of your application, the highest ancestor of all other nodes in
    /// your real DOM tree.
    pub fn root_node(&self) -> Node {
        // Note that we're cloning the `web_sys::Node`, not the DOM element.
        // So we're effectively cloning a pointer here, which is fast.
        self.root_node.clone()
    }
}

fn create_unique_identifier() -> u32 {
    let mut elem_unique_id =
        ELEM_UNIQUE_ID.lock().expect("Unable to obtain lock");
    *elem_unique_id += 1;
    *elem_unique_id
}

impl From<CreatedNode<Element>> for CreatedNode<Node> {
    fn from(other: CreatedNode<Element>) -> CreatedNode<Node> {
        CreatedNode {
            node: other.node.into(),
            closures: other.closures,
        }
    }
}

impl<T> Deref for CreatedNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
