use crate::{
    dom::{
        created_node::{self,ActiveClosure, CreatedNode},
        Dispatch,
        DomPatch,
    },
    vdom,
    vdom::diff,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{self, Element, Node};
//use crate::vdom::Patch;
use mt_dom::TreePath;
use std::collections::VecDeque;

/// Used for keeping a real DOM node up to date based on the current Node
/// and a new incoming Node that represents our latest DOM state.
pub struct DomUpdater<MSG> {
    /// the current vdom representation
    pub current_vdom: vdom::Node<MSG>,
    /// the first element of the app view, where the patch is generated is relative to
    pub root_node: Option<Node>,

    /// the actual DOM element where the APP is mounted to.
    pub mount_node: Node,

    /// The closures that are currently attached to elements in the page.
    ///
    /// We keep these around so that they don't get dropped (and thus stop working);
    ///
    pub active_closures: ActiveClosure,
    /// after mounting or update dispatch call, the element will be focused
    pub focused_node: Option<Node>,

    /// if the mount node is replaced by the root_node
    pub replace: bool,

    /// whether or not to use shadow root of the mount_node
    pub use_shadow: bool,

    /// for optimization purposes to avoid sluggishness of the app, when a patch
    /// can not be run in 1 execution due to limited remaining time deadline
    /// it will be put into the pending patches to be executed on the next run.
    pub pending_patches: VecDeque<DomPatch<MSG>>,
}


impl<MSG> DomUpdater<MSG> {
    /// Creates and instance of this DOM updater, but doesn't mount the current_vdom to the DOM just yet.
    pub(crate) fn new(
        current_vdom: vdom::Node<MSG>,
        mount: &Node,
        replace: bool,
        use_shadow: bool,
    ) -> DomUpdater<MSG> {
        DomUpdater {
            current_vdom,
            root_node: None,
            mount_node: mount.clone(),
            active_closures: ActiveClosure::new(),
            focused_node: None,
            replace,
            use_shadow,
            pending_patches: VecDeque::new(),
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
    /// each element and it's descendant in the vdom is created into
    /// an actual DOM node.
    pub fn mount<DSP>(&mut self, program: &DSP)
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let created_node = CreatedNode::create_dom_node(
            program,
            &self.current_vdom,
            &mut self.focused_node,
        );

        //TODO: maybe remove replace the mount
        if self.replace {
            let mount_element: &Element = self.mount_node.unchecked_ref();
            mount_element
                .replace_with_with_node_1(&created_node.node)
                .expect("Could not append child to mount");
        } else if self.use_shadow {
            let mount_element: &web_sys::Element =
                self.mount_node.unchecked_ref();
            mount_element
                .attach_shadow(&web_sys::ShadowRootInit::new(
                    web_sys::ShadowRootMode::Open,
                ))
                .expect("unable to attached shadow");
            let mount_shadow =
                mount_element.shadow_root().expect("must have a shadow");

            let mount_shadow_node: &web_sys::Node =
                mount_shadow.unchecked_ref();

            mount_shadow_node
                .append_child(&created_node.node)
                .expect("could not append child to mount shadow");
        } else {
            self.mount_node
                .append_child(&created_node.node)
                .expect("Could not append child to mount");
        }
        self.root_node = Some(created_node.node);
        self.active_closures = created_node.closures;
        self.set_focus_element();
    }

    /// inject style element to the mount node
    pub fn inject_style_to_mount<DSP>(&self, program: &DSP, style: &str)
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let style_node =
            crate::html::tags::style([], [crate::html::text(style)]);
        let created_node =
            CreatedNode::create_dom_node(program, &style_node, &mut None);
        if self.use_shadow {
            let mount_element: &web_sys::Element =
                self.mount_node.unchecked_ref();
            let mount_shadow =
                mount_element.shadow_root().expect("must have a shadow");

            let mount_shadow_node: &web_sys::Node =
                mount_shadow.unchecked_ref();

            mount_shadow_node
                .append_child(&created_node.node)
                .expect("could not append child to mount shadow");
        } else {
            panic!("injecting style to non shadow mount is not supported");
        }
    }

    fn set_focus_element(&self) {
        if let Some(focused_node) = &self.focused_node {
            let focused_element: &Element = focused_node.unchecked_ref();
            CreatedNode::set_element_focus(focused_element);
        }
    }

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and appended (as a child) to your passed
    /// in mount element.
    pub fn new_append_to_mount<DSP>(
        program: &DSP,
        current_vdom: vdom::Node<MSG>,
        mount: &Element,
    ) -> DomUpdater<MSG>
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {
        let mut dom_updater = Self::new(current_vdom, mount, false, false);
        dom_updater.mount(program);
        dom_updater
    }

    /// Diff the current virtual dom with the new virtual dom that is being passed in.
    ///
    /// Then use that diff to patch the real DOM in the user's browser so that they are
    /// seeing the latest state of the application.
    ///
    /// Return the total number of patches applied
    pub async fn update_dom<DSP>(
        &mut self,
        program: &DSP,
        new_vdom: vdom::Node<MSG>,
    ) -> Result<usize, JsValue>
    where
        DSP: Dispatch<MSG> + Clone + 'static,
    {

        let total_patches = self.patch(
            program,
            new_vdom
        )?;


        self.set_focus_element();
        Ok(total_patches)
    }




    /// Apply all of the patches to our old root node in order to create the new root node
    /// that we desire.
    /// This is usually used after diffing two virtual nodes.
    ///
    pub fn patch<DSP>(
        &mut self,
        program: &DSP,
        new_vdom: vdom::Node<MSG>,
    ) -> Result<usize, JsValue>
    where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        let patches = diff(&self.current_vdom, &new_vdom);
        // move the diff into patch function.
        let total_patches = patches.len();

        #[cfg(feature = "with-debug")]
        log::debug!("patches: {:#?}", patches);

        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        let t1 = crate::now();
        let nodes_to_find: Vec<(&TreePath, Option<&&'static str>)> = patches
            .iter()
            .map(|patch| (patch.path(), patch.tag()))
            .collect();

        let mut paths = vec![];
        for patch in patches.iter() {
            paths.push(patch.path());
        }

        //Note: it is important that root_node points to the original mutable reference here
        // since it can be replaced with a new root Node(the top-level node of the view) when patching
        let root_node = self.root_node.as_mut().expect("must have a root_node");

        let nodes_to_patch = created_node::find_all_nodes(root_node, &nodes_to_find);
        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        let t2 = crate::now();

        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        log::info!("Took {}ms to find all the nodes", t2 - t1);

        //TODO: spawn all the apply patch here to to it asynchronously
        // can be done with Promise.all (https://docs.rs/js-sys/0.3.61/js_sys/struct.Promise.html#method.all)
        for patch in patches.iter() {
            let patch_path = patch.path();
            let patch_tag = patch.tag();
            if let Some(target_node) = nodes_to_patch.get(patch_path) {
                // check the tag here if it matches
                let target_element: &Element = target_node.unchecked_ref();
                if let Some(tag) = patch_tag {
                    let target_tag = target_element.tag_name().to_lowercase();
                    if target_tag != **tag {
                        panic!(
                            "expecting a tag: {:?}, but found: {:?}",
                            tag, target_tag
                        );
                    }
                }

                #[cfg(all(feature = "with-measure", feature = "with-debug"))]
                let t3 = crate::now();

                //TODO: push this into a vecqueue for executing the patches
                //taking into account deadline remaining time
                let dom_patch =
                    DomPatch::from_patch(program, target_node, &mut self.focused_node, patch);

                #[cfg(all(feature = "with-measure", feature = "with-debug"))]
                let t4 = crate::now();

                #[cfg(all(feature = "with-measure", feature = "with-debug"))]
                log::info!("Creating dom_patch took {}ms", t4 - t3);

                self.pending_patches.push_back(dom_patch);
            } else {
                unreachable!("Getting here means we didn't find the element of next node that we are supposed to patch, patch_path: {:?}, with tag: {:?}", patch_path, patch_tag);
            }
        }

        let deadline = 100.0;

        self.apply_pending_patches(program, deadline)
            .expect("must not error");

        self.current_vdom = new_vdom;
        Ok(total_patches)
    }

    /// apply the pending patches into the DOM
    fn apply_pending_patches<DSP>(
        &mut self,
        program: &DSP,
        deadline: f64,
    ) -> Result<(), JsValue>
    where
        MSG: 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        let t1 = crate::now();
        #[cfg(feature = "with-debug")]
        let mut cnt = 0;
        while let Some(dom_patch) = self.pending_patches.pop_front() {
            #[cfg(feature = "with-debug")]
            log::debug!("Executing pending patch item {}", cnt);
            let t2 = crate::now();
            dom_patch
                .apply(program, &mut self.active_closures)
                .expect("must apply the dom patch");
            let elapsed = t2 - t1;
            if elapsed > deadline {
                log::info!("breaking here...");
                break;
            }
            #[cfg(feature = "with-debug")]
            {
                cnt += 1;
            }
        }
        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        let t3 = crate::now();
        #[cfg(all(feature = "with-measure", feature = "with-debug"))]
        log::info!("Pending patches took {}ms", t3 - t1);
        Ok(())
    }

    /// Return the root node of your application, the highest ancestor of all other nodes in
    /// your real DOM tree.
    pub fn mount_node(&self) -> Node {
        // Note that we're cloning the `web_sys::Node`, not the DOM element.
        // So we're effectively cloning a pointer here, which is fast.
        self.mount_node.clone()
    }
}

