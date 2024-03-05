//! patch module

use super::Tag;
use super::{Attribute, Node};


use derive_where::derive_where;

pub use tree_path::TreePath;

mod tree_path;

/// A Patch encodes an operation that modifies a real DOM element or native UI element
///
/// To update the real DOM that a user sees you'll want to first diff your
/// old virtual dom and new virtual dom.
///
/// This diff operation will generate `Vec<Patch>` with zero or more patches that, when
/// applied to your real DOM, will make your real DOM look like your new virtual dom.
///
/// Each of the Patch contains `TreePath` which contains an array of indexes for each node
/// that we need to traverse to get the target element.
///
/// Consider the following html:
///
/// ```html
/// <body>
///     <main>
///         <input type="text"/>
///         <img src="pic.jpg"/>
///     </main>
///     <footer>
///         <a>Link</a>
///         <nav/>
///     </footer>
/// </body>
/// ```
/// The corresponding DOM tree would be
/// ```bob
///              .─.
///             ( 0 )  <body>
///              `-'
///             /   \
///            /     \
///           /       \
///          ▼         ▼
///  <main> .─.         .─. <footer>
///        ( 0 )       ( 1 )
///         `-'         `-'
///        /  \          | \ '.
///       /    \         |  \  '.
///      ▼      ▼        |   \   '.
///    .─.      .─.      ▼    ▼     ▼
///   ( 0 )    ( 1 )    .─.   .─.   .─.
///    `─'      `─'    ( 0 ) ( 1 ) ( 2 )
///  <input> <img>      `─'   `─'   `─'
///                    <a>  <Text>   <nav>
/// ```
/// To traverse to the `<nav>` element we follow the TreePath([0,1,2]).
/// 0 - is the root element which is always zero.
/// 1 - is the `footer` element since it is the 2nd element of the body.
/// 2 - is the `nav` element since it is the 3rd node in the `footer` element.
#[derive_where(Clone, Debug, PartialEq, Eq)]
pub struct Patch<'a, MSG> {
    /// the tag of the node at patch_path
    pub tag: Option<&'a Tag>,
    /// the path to traverse to get to the target element
    pub patch_path: TreePath,
    /// the type of patch we are going to apply
    pub patch_type: PatchType<'a, MSG>,
}

/*
impl<'a, MSG> Clone for Patch<'a, MSG>{
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<'a, MSG> fmt::Debug for Patch<'a, MSG>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl<'a, MSG> PartialEq for Patch<'a, MSG>{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a, MSG> Eq for Patch<'a, MSG>{ }
*/

/// the patch variant
#[derive_where(Clone, Debug, PartialEq, Eq)]
pub enum PatchType<'a, MSG> {
    /// insert the nodes before the node at patch_path
    InsertBeforeNode {
        /// the nodes to be inserted before patch_path
        nodes: Vec<&'a Node<MSG>>,
    },

    /// insert the nodes after the node at patch_path
    InsertAfterNode {
        /// the nodes to be inserted after the patch_path
        nodes: Vec<&'a Node<MSG>>,
    },

    /// Append a vector of child nodes to a parent node id at patch_path
    AppendChildren {
        /// children nodes to be appended and their corresponding new_node_idx
        children: Vec<&'a Node<MSG>>,
    },
    /// remove the target node
    RemoveNode,
    /// remove the nodes pointed at these `nodes_path`
    /// and move them before `target_element` pointed at `patch_path`
    MoveBeforeNode {
        /// before this target location
        nodes_path: Vec<TreePath>,
    },
    /// remove the the nodes pointed at these nodes_path
    /// and move them after the `target_element` pointed at `patch_path`
    MoveAfterNode {
        /// after this target location
        nodes_path: Vec<TreePath>,
    },

    /// ReplaceNode a node with another node. This typically happens when a node's tag changes.
    /// ex: <div> becomes <span>
    ReplaceNode {
        /// the node that will replace the target node
        replacement: Vec<&'a Node<MSG>>,
    },
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    AddAttributes {
        /// the attributes to be patched into the target node
        attrs: Vec<&'a Attribute<MSG>>,
    },
    /// Remove attributes that the old node had that the new node doesn't
    RemoveAttributes {
        /// attributes that are to be removed from this target node
        attrs: Vec<&'a Attribute<MSG>>,
    },
}


impl<'a, MSG> Patch<'a, MSG> {
    /// return the path to traverse for this patch to get to the target Node
    pub fn path(&self) -> &TreePath {
        &self.patch_path
    }

    /// return the node paths involve such as those in moving nodes
    pub fn node_paths(&self) -> &[TreePath] {
        match &self.patch_type {
            PatchType::MoveBeforeNode { nodes_path } => nodes_path,
            PatchType::MoveAfterNode { nodes_path } => nodes_path,
            _ => &[],
        }
    }

    /// return the tag of this patch
    pub fn tag(&self) -> Option<&Tag> {
        self.tag
    }

    /// create an InsertBeforeNode patch
    pub fn insert_before_node(
        tag: Option<&'a Tag>,
        patch_path: TreePath,
        nodes: impl IntoIterator<Item = &'a Node<MSG>>,
    ) -> Patch<'a, MSG> {
        Patch {
            tag,
            patch_path,
            patch_type: PatchType::InsertBeforeNode {
                nodes: nodes.into_iter().collect(),
            },
        }
    }

    /// create an InsertAfterNode patch
    pub fn insert_after_node(
        tag: Option<&'a Tag>,
        patch_path: TreePath,
        nodes: Vec<&'a Node<MSG>>,
    ) -> Patch<'a, MSG> {
        Patch {
            tag,
            patch_path,
            patch_type: PatchType::InsertAfterNode { nodes },
        }
    }

    /// create a patch where we add children to the target node
    pub fn append_children(
        tag: Option<&'a Tag>,
        patch_path: TreePath,
        children: Vec<&'a Node<MSG>>,
    ) -> Patch<'a, MSG> {
        Patch {
            tag,
            patch_path,
            patch_type: PatchType::AppendChildren { children },
        }
    }

    /// create a patch where the target element that can be traverse
    /// using the patch path will be remove
    pub fn remove_node(
        tag: Option<&'a Tag>,
        patch_path: TreePath,
    ) -> Patch<'a, MSG> {
        Patch {
            tag,
            patch_path,
            patch_type: PatchType::RemoveNode,
        }
    }

    /// remove the nodes pointed at the `nodes_path` and insert them before the target element
    /// pointed at patch_path
    pub fn move_before_node(
        tag: Option<&'a Tag>,
        patch_path: TreePath,
        nodes_path: impl IntoIterator<Item = TreePath>,
    ) -> Patch<'a, MSG> {
        Patch {
            tag,
            patch_path,
            patch_type: PatchType::MoveBeforeNode {
                nodes_path: nodes_path.into_iter().collect(),
            },
        }
    }

    /// remove the nodes pointed at the `nodes_path` and insert them after the target element
    /// pointed at patch_path
    pub fn move_after_node(
        tag: Option<&'a Tag>,
        patch_path: TreePath,
        nodes_path: impl IntoIterator<Item = TreePath>,
    ) -> Patch<'a, MSG> {
        Patch {
            tag,
            patch_path,
            patch_type: PatchType::MoveAfterNode {
                nodes_path: nodes_path.into_iter().collect(),
            },
        }
    }

    /// create a patch where a node is replaced by the `replacement` node.
    /// The target node to be replace is traverse using the `patch_path`
    pub fn replace_node(
        tag: Option<&'a Tag>,
        patch_path: TreePath,
        replacement: impl IntoIterator<Item = &'a Node<MSG>>,
    ) -> Patch<'a, MSG> {
        Patch {
            tag,
            patch_path,
            patch_type: PatchType::ReplaceNode {
                replacement: replacement.into_iter().collect(),
            },
        }
    }

    /// create a patch where a new attribute is added to the target element
    pub fn add_attributes(
        tag: &'a Tag,
        patch_path: TreePath,
        attrs: impl IntoIterator<Item = &'a Attribute<MSG>>,
    ) -> Patch<'a, MSG> {
        Patch {
            tag: Some(tag),
            patch_path,
            patch_type: PatchType::AddAttributes {
                attrs: attrs.into_iter().collect(),
            },
        }
    }

    /// create patch where it remove attributes of the target element that can be traversed by the
    /// patch_path.
    pub fn remove_attributes(
        tag: &'a Tag,
        patch_path: TreePath,
        attrs: Vec<&'a Attribute<MSG>>,
    ) -> Patch<'a, MSG> {
        Patch {
            tag: Some(tag),
            patch_path,
            patch_type: PatchType::RemoveAttributes { attrs },
        }
    }
}
