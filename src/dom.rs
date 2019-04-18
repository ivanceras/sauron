use crate::Component;
use crate::Program;
use apply_patches::patch;
use sauron_vdom::Callback;
use sauron_vdom::{self, diff};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{self, Element, EventTarget, Node, Text};
use web_sys::{Event, KeyboardEvent, MouseEvent};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

mod apply_patches;

// Used to uniquely identify elements that contain closures so that the DomUpdater can
// look them up by their unique id.
// When the DomUpdater sees that the element no longer exists it will drop all of it's
// Rc'd Closures for those events.
use lazy_static::lazy_static;
lazy_static! {
    static ref ELEM_UNIQUE_ID: Mutex<u32> = Mutex::new(0);
}

pub type ActiveClosure = HashMap<u32, Vec<Closure<Fn(Event)>>>;

/// A node along with all of the closures that were created for that
/// node's events and all of it's child node's events.
pub struct CreatedNode<T> {
    /// A `Node` or `Element` that was created from a `Node`
    pub node: T,
    closures: ActiveClosure,
}

/// Used for keeping a real DOM node up to date based on the current Node
/// and a new incoming Node that represents our latest DOM state.
pub struct DomUpdater<APP, MSG> {
    current_vdom: crate::Node<MSG>,
    root_node: Node,

    /// The closures that are currently attached to elements in the page.
    ///
    /// We keep these around so that they don't get dropped (and thus stop working);
    ///
    /// FIXME: Drop them when the element is no longer in the page. Need to figure out
    /// a good strategy for when to do this.
    pub active_closures: ActiveClosure,
    _phantom_data: PhantomData<APP>,
}

impl<T> CreatedNode<T> {
    pub fn without_closures<N: Into<T>>(node: N) -> Self {
        CreatedNode {
            node: node.into(),
            closures: HashMap::with_capacity(0),
        }
    }

    pub fn create_text_node(text: &sauron_vdom::Text) -> Text {
        let document = web_sys::window().unwrap().document().unwrap();
        document.create_text_node(&text.text)
    }

    /// Create and return a `CreatedNode` instance (containing a DOM `Node`
    /// together with potentially related closures) for this virtual node.
    pub fn create_dom_node<APP, MSG>(
        program: &Rc<Program<APP, MSG>>,
        vnode: &crate::Node<MSG>,
    ) -> CreatedNode<Node>
    where
        MSG: Clone + Debug + 'static,
        APP: Component<MSG> + 'static,
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
    pub fn create_element_node<APP, MSG>(
        program: &Rc<Program<APP, MSG>>,
        velem: &crate::Element<MSG>,
    ) -> CreatedNode<Element>
    where
        MSG: Clone + Debug + 'static,
        APP: Component<MSG> + 'static,
    {
        let document = web_sys::window().unwrap().document().unwrap();

        let element = if let Some(ref namespace) = velem.namespace {
            document
                .create_element_ns(Some(namespace), &velem.tag)
                .unwrap()
        } else {
            document.create_element(&velem.tag).unwrap()
        };

        let mut closures = HashMap::new();

        velem.attrs.iter().for_each(|(name, value)| {
            element
                .set_attribute(name, &value.to_string())
                .expect("Set element attribute in create element");
        });

        if !velem.events.is_empty() {
            let unique_id = create_unique_identifier();

            element
                .set_attribute("data-sauron_vdom-id", &unique_id.to_string())
                .expect("Could not set attribute on element");

            closures.insert(unique_id, vec![]);

            velem.events.iter().for_each(
                |(event_str, callback): (&String, &Callback<sauron_vdom::Event, MSG>)| {
                    let current_elem: &EventTarget = element.dyn_ref().unwrap();

                    let closure_wrap: Closure<Fn(Event)> = create_closure_wrap(program, &callback);

                    current_elem
                        .add_event_listener_with_callback(
                            event_str,
                            closure_wrap.as_ref().unchecked_ref(),
                        )
                        .unwrap();

                    closures.get_mut(&unique_id).unwrap().push(closure_wrap);
                },
            );
        }

        let mut previous_node_was_text = false;
        velem.children.iter().for_each(|child| {
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
                            .unwrap();
                    }

                    current_node
                        .append_child(&Self::create_text_node(&text_node))
                        .unwrap();

                    previous_node_was_text = true;
                }
                crate::Node::Element(element_node) => {
                    previous_node_was_text = false;

                    let child = Self::create_element_node(program, element_node);
                    let child_elem: Element = child.node;
                    closures.extend(child.closures);

                    element.append_child(&child_elem).unwrap();
                }
            }
        });

        CreatedNode {
            node: element,
            closures,
        }
    }
}

fn create_closure_wrap<APP, MSG>(
    program: &Rc<Program<APP, MSG>>,
    callback: &Callback<sauron_vdom::Event, MSG>,
) -> Closure<Fn(Event)>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    let callback_clone = callback.clone();
    let program_clone = Rc::clone(&program);
    let app_clone: Rc<RefCell<APP>> = Rc::clone(&program.app);
    let dom_updater_clone: Rc<RefCell<DomUpdater<APP, MSG>>> = Rc::clone(&program.dom_updater);

    Closure::wrap(Box::new(move |event: Event| {
        let mouse_event: Option<&MouseEvent> = event.dyn_ref();
        let key_event: Option<&KeyboardEvent> = event.dyn_ref();
        let target: Option<EventTarget> = event.target();

        let cb_event = if let Some(mouse_event) = mouse_event {
            if event.type_() == "click" {
                sauron_vdom::Event::MouseEvent(sauron_vdom::MouseEvent::Press(
                    sauron_vdom::MouseButton::Left,
                    mouse_event.x() as u16,
                    mouse_event.y() as u16,
                ))
            } else {
                sauron_vdom::Event::Generic(event.type_())
            }
        } else if let Some(key_event) = key_event {
            sauron_vdom::Event::KeyEvent(sauron_vdom::KeyEvent {
                key: key_event.key(),
                ctrl: key_event.ctrl_key(),
                alt: key_event.alt_key(),
                shift: key_event.shift_key(),
                meta: key_event.meta_key(),
            })
        } else if let Some(target) = target {
            let input: Option<&HtmlInputElement> = target.dyn_ref();
            let textarea: Option<&HtmlTextAreaElement> = target.dyn_ref();
            if let Some(input) = input {
                sauron_vdom::Event::InputEvent(sauron_vdom::InputEvent {
                    value: input.value(),
                })
            } else if let Some(textarea) = textarea {
                sauron_vdom::Event::InputEvent(sauron_vdom::InputEvent {
                    value: textarea.value(),
                })
            } else {
                sauron_vdom::Event::Generic(event.type_())
            }
        } else {
            sauron_vdom::Event::Generic(event.type_())
        };
        let msg = callback_clone.emit(cb_event);
        // FIXME: dispatch msg here
        crate::log!("dispatch msg here.. {:#?}", msg);
        app_clone.borrow_mut().update(&msg);
        let view = app_clone.borrow().view();
        //That's why Program needs to be wrapped with Rc, so we can pass it dom_updater.update
        //function
        //using a clone
        dom_updater_clone.borrow_mut().update(&program_clone, view);
    }))
}

impl<APP, MSG> DomUpdater<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    pub fn new(current_vdom: crate::Node<MSG>, root_node: &Node) -> DomUpdater<APP, MSG> {
        DomUpdater {
            current_vdom,
            root_node: root_node.clone(),
            active_closures: ActiveClosure::new(),
            _phantom_data: PhantomData,
        }
    }

    pub fn mount(&mut self, program: &Rc<Program<APP, MSG>>) {
        let created_node: CreatedNode<Node> =
            CreatedNode::<Node>::create_dom_node(program, &self.current_vdom);
        self.root_node
            .append_child(&created_node.node)
            .expect("Could not append child to mount");
        self.root_node = created_node.node;
        self.active_closures = created_node.closures;
    }
    /*
    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created but not added to your DOM.
    pub fn new(program: &Rc<Program<APP,MSG>>, current_vdom: crate::Node<MSG>) -> DomUpdater<APP, MSG> {
        let created_node = CreatedNode::<Node>::create_dom_node(program, &current_vdom);
        DomUpdater {
            current_vdom,
            root_node: created_node.node,
            active_closures: created_node.closures,
            _phantom_data: PhantomData,
        }
    }
    */

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and appended (as a child) to your passed
    /// in mount element.
    pub fn new_append_to_mount(
        program: &Rc<Program<APP, MSG>>,
        current_vdom: crate::Node<MSG>,
        mount: &Element,
    ) -> DomUpdater<APP, MSG> {
        let created_node: CreatedNode<Node> =
            CreatedNode::<Node>::create_dom_node(program, &current_vdom);
        mount
            .append_child(&created_node.node)
            .expect("Could not append child to mount");
        DomUpdater {
            current_vdom,
            root_node: created_node.node,
            active_closures: created_node.closures,
            _phantom_data: PhantomData,
        }
    }

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and it will replace your passed in mount
    /// element.
    pub fn new_replace_mount(
        program: &Rc<Program<APP, MSG>>,
        current_vdom: crate::Node<MSG>,
        mount: Element,
    ) -> DomUpdater<APP, MSG> {
        let created_node = CreatedNode::<Node>::create_dom_node(program, &current_vdom);
        mount
            .replace_with_with_node_1(&created_node.node)
            .expect("Could not replace mount element");
        DomUpdater {
            current_vdom,
            root_node: created_node.node,
            active_closures: created_node.closures,
            _phantom_data: PhantomData,
        }
    }

    /// Diff the current virtual dom with the new virtual dom that is being passed in.
    ///
    /// Then use that diff to patch the real DOM in the user's browser so that they are
    /// seeing the latest state of the application.
    pub fn update(&mut self, program: &Rc<Program<APP, MSG>>, new_vdom: crate::Node<MSG>) {
        let patches = diff(&self.current_vdom, &new_vdom);
        let active_closures = patch(
            program,
            self.root_node.clone(),
            &mut self.active_closures,
            &patches,
        )
        .unwrap();
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
    let mut elem_unique_id = ELEM_UNIQUE_ID.lock().unwrap();
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
