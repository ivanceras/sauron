use crate::{
    diff,
    dom::{
        apply_patches::patch,
        created_node::{
            ActiveClosure,
            CreatedNode,
        },
        Dispatch,
    },
    Patch,
};
use wasm_bindgen::JsCast;
use web_sys::{
    self,
    Element,
    Node,
};

/// Used for keeping a real DOM node up to date based on the current Node
/// and a new incoming Node that represents our latest DOM state.
pub struct DomUpdater<MSG> {
    /// the current vdom representation
    pub current_vdom: crate::Node<MSG>,
    root_node: Node,

    /// The closures that are currently attached to elements in the page.
    ///
    /// We keep these around so that they don't get dropped (and thus stop working);
    ///
    pub active_closures: ActiveClosure,
}

impl<MSG> DomUpdater<MSG> {
    /// Creates and instance of this DOM updater, but doesn't mount the current_vdom to the DOM just yet.
    pub fn new(
        current_vdom: crate::Node<MSG>,
        mount: &Node,
    ) -> DomUpdater<MSG> {
        DomUpdater {
            current_vdom,
            root_node: mount.clone(),
            active_closures: ActiveClosure::new(),
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
}

impl<MSG> DomUpdater<MSG>
where
    MSG: 'static,
{
    /// Mount the current_vdom appending to the actual browser DOM specified in the root_node
    /// This also gets the closures that was created when mounting the vdom to their
    /// actual DOM counterparts.
    pub fn append_to_mount<DSP>(&mut self, program: &DSP)
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        self.mount(program, false);
    }

    fn mount<DSP>(&mut self, program: &DSP, replace: bool)
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let created_node: CreatedNode<Node> =
            CreatedNode::<Node>::create_dom_node(
                program,
                &self.current_vdom,
                &mut Some(0),
            );
        if replace {
            let root_element: &Element = self.root_node.unchecked_ref();
            root_element
                .replace_with_with_node_1(&created_node.node)
                .expect("Could not append child to mount");
        } else {
            self.root_node
                .append_child(&created_node.node)
                .expect("Could not append child to mount");
        }
        self.root_node = created_node.node;
        self.active_closures = created_node.closures;
    }

    /// Mount the current_vdom replacing the actual browser DOM specified in the root_node
    /// This also gets the closures that was created when mounting the vdom to their
    /// actual DOM counterparts.
    pub fn replace_mount<DSP>(&mut self, program: &DSP)
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        self.mount(program, true);
    }

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and appended (as a child) to your passed
    /// in mount element.
    pub fn new_append_to_mount<DSP>(
        program: &DSP,
        current_vdom: crate::Node<MSG>,
        mount: &Element,
    ) -> DomUpdater<MSG>
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let mut dom_updater = Self::new(current_vdom, mount);
        dom_updater.append_to_mount(program);
        dom_updater
    }

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and it will replace your passed in mount
    /// element.
    pub fn new_replace_mount<DSP>(
        program: &DSP,
        current_vdom: crate::Node<MSG>,
        mount: Element,
    ) -> DomUpdater<MSG>
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let mut dom_updater = Self::new(current_vdom, &mount);
        dom_updater.replace_mount(program);
        dom_updater
    }

    /// Diff the current virtual dom with the new virtual dom that is being passed in.
    ///
    /// Then use that diff to patch the real DOM in the user's browser so that they are
    /// seeing the latest state of the application.
    pub fn update_dom<DSP>(&mut self, program: &DSP, new_vdom: crate::Node<MSG>)
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        /*
        #[cfg(feature = "with-measure")]
        use crate::render::Render;
        */

        /*
        #[cfg(feature = "with-measure")]
        let _current_dom = {
            let mut current_dom = String::new();
            self.current_vdom
                .render_compressed(&mut current_dom)
                .expect("must render");
            log::trace!("current dom: {}", current_dom);
            current_dom
        };
        */

        /*
        #[cfg(feature = "with-measure")]
        let _target_dom = {
            let mut target_dom = String::new();
            new_vdom
                .render_compressed(&mut target_dom)
                .expect("must render");
            log::trace!("target dom: {}", target_dom);
            target_dom
        };
        */

        let patches = diff(&self.current_vdom, &new_vdom);
        #[cfg(feature = "with-measure")]
        log::trace!("applying {} patches", patches.len());
        #[cfg(feature = "with-measure")]
        log::trace!("patches: {:#?}", patches);

        let active_closures = patch(
            Some(program),
            self.root_node.clone(),
            &mut self.active_closures,
            patches,
        )
        .expect("Error in patching the dom");

        /*
        #[cfg(feature = "with-measure")]
        {
            let _root_element: &web_sys::Element =
                self.root_node.unchecked_ref();
            //assert_eq!(target_dom, root_element.outer_html());
        }
        */

        self.active_closures.extend(active_closures);
        self.current_vdom = new_vdom;
    }

    /// Apply patches to the dom updater
    /// Warning: only used this for debuggin purposes
    pub fn patch_dom<DSP>(&mut self, program: &DSP, patches: Vec<Patch<MSG>>)
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let active_closures = patch(
            Some(program),
            self.root_node.clone(),
            &mut self.active_closures,
            patches,
        )
        .expect("Error in patching the dom");
        self.active_closures.extend(active_closures);
    }

    /// map this DomUpdater such that the Node<MSG> will become Node<MSG2>
    pub fn map_msg<F, MSG2>(self, func: F) -> DomUpdater<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let DomUpdater {
            current_vdom,
            root_node,
            active_closures,
        } = self;
        DomUpdater {
            current_vdom: current_vdom.map_msg(func),
            root_node,
            active_closures,
        }
    }

    /// Return the root node of your application, the highest ancestor of all other nodes in
    /// your real DOM tree.
    pub fn root_node(&self) -> Node {
        // Note that we're cloning the `web_sys::Node`, not the DOM element.
        // So we're effectively cloning a pointer here, which is fast.
        self.root_node.clone()
    }
}
