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
    mt_dom::NodeIdx,
    Patch,
};
use std::collections::HashMap;
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
    /// a fast lookup for getting the Node from from NodeIdx
    pub node_idx_lookup: HashMap<NodeIdx, Node>,
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
            node_idx_lookup: HashMap::new(),
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

    /// each element and it's descendant in the vdom is created into
    /// an actual DOM node.
    fn mount<DSP>(&mut self, program: &DSP, replace: bool)
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let created_node: CreatedNode<Node> =
            CreatedNode::<Node>::create_dom_node(
                program,
                &mut self.node_idx_lookup,
                &self.current_vdom,
                &mut 0,
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
        #[cfg(feature = "with-measure")]
        let t1 = crate::now();
        let patches = diff(&self.current_vdom, &new_vdom);

        #[cfg(feature = "with-measure")]
        let _t2 = {
            let t2 = crate::now();
            log::trace!("vdom diffing took: {}ms", t2 - t1);
            t2
        };

        #[cfg(feature = "with-measure")]
        log::trace!("applying {} patches", patches.len());

        /*
        #[cfg(feature = "with-debug")]
        log::debug!("patches: {:#?}", patches);
        */

        let active_closures = patch(
            Some(program),
            self.root_node.clone(),
            &mut self.active_closures,
            &mut self.node_idx_lookup,
            patches,
        )
        .expect("Error in patching the dom");

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
            &mut self.node_idx_lookup,
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
            node_idx_lookup,
        } = self;
        DomUpdater {
            current_vdom: current_vdom.map_msg(func),
            root_node,
            active_closures,
            node_idx_lookup,
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
