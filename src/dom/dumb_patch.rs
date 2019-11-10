use crate::{
    dom::{
        apply_patches,
        created_node::{
            ActiveClosure,
            CreatedNode,
        },
    },
    Dispatch,
    Patch,
};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::Node;

struct DumbProgram();

impl Dispatch<()> for DumbProgram {
    fn dispatch(self: &Rc<Self>, _msg: ()) {}
}

pub fn create_dumb_node(vnode: &crate::Node<()>) -> CreatedNode<Node> {
    CreatedNode::<Node>::create_dom_node_opt(None::<&Rc<DumbProgram>>, vnode)
}

/// Apply a patch to the DOM
/// The patch can be coming from somewhere else, not only
/// as the effect of dom_update, this can also come
/// from a server_side pre-processed diff
///
/// Warning: Use only when the dom update is only one-way
/// meaning, the source keep track of the previous vdom
pub fn apply_dumb_patch<N>(
    root_node: N,
    patches: &[Patch<()>],
) -> Result<(), JsValue>
where
    N: Into<Node>,
{
    apply_patches::patch(
        None::<&Rc<DumbProgram>>,
        root_node,
        &mut ActiveClosure::new(),
        &patches,
    )
    .expect("Error in patching the dom");
    Ok(())
}
