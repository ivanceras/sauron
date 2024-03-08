/// specify how the App is mounted to the DOM
#[derive(Clone, Copy)]
pub enum MountAction {
    /// append the APP's root node to the target mount node
    Append,
    /// clear any children of the target mount node then append the APP's root node
    ClearAppend,
    /// replace the target mount node with the APP's root node
    Replace,
}

/// specify whether to attach the Node in shadow_root
#[derive(Clone, Copy)]
pub enum MountTarget {
    /// attached in the mount node
    MountNode,
    /// attached to the shadow root
    ShadowRoot,
}

/// specify how the root node will be mounted to the mount node
#[derive(Clone, Copy)]
pub struct MountProcedure {
    pub action: MountAction,
    pub target: MountTarget,
}

impl MountProcedure {
    /// mount procedure with specified action and target
    pub fn new(action: MountAction, target: MountTarget) -> Self {
        Self { action, target }
    }

    /// append to the mount node
    pub fn append() -> Self {
        Self::new(MountAction::Append, MountTarget::MountNode)
    }

    /// clear the mount node before appending
    pub fn clear_append() -> Self {
        Self::new(MountAction::ClearAppend, MountTarget::MountNode)
    }

    /// replace the mount node
    pub fn replace() -> Self {
        Self::new(MountAction::Replace, MountTarget::MountNode)
    }

    /// append to the mount node but on it's shadow
    pub fn append_to_shadow() -> Self {
        Self::new(MountAction::Append, MountTarget::ShadowRoot)
    }
}

impl Default for MountProcedure {
    fn default() -> Self {
        Self {
            action: MountAction::Append,
            target: MountTarget::MountNode,
        }
    }
}
