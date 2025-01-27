//! provides diffing algorithm which returns patches
use super::{diff_lis, Attribute, Element, Node, Patch, TreePath};
use super::{Tag, KEY, REPLACE, SKIP, SKIP_CRITERIA};
use crate::dom::skip_diff::SkipAttrs;
use crate::dom::SkipPath;
use crate::vdom::AttributeValue;
use crate::vdom::Leaf;
use std::{cmp, mem};

#[cfg(feature = "use-skipdiff")]
static USE_SKIP_DIFF: bool = true;

#[cfg(not(feature = "use-skipdiff"))]
static USE_SKIP_DIFF: bool = false;

/// all the possible error when diffing Node(s)
#[derive(Debug, thiserror::Error, Clone, Copy)]
pub enum DiffError {
    /// Node list must have already unrolled when creating an element
    #[error("Node list must have already unrolled when creating an element")]
    UnrollError,
    /// Skip diff error
    #[error("Skip diff error")]
    SkipDiffError,
    /// Invalid root node count of: {0}
    #[error("Invalid root node count of: {0}")]
    InvalidRootNodeCount(usize),
}

/// Return the patches needed for `old_node` to have the same DOM as `new_node`
///
/// # Agruments
/// * old_node - the old virtual dom node
/// * new_node - the new virtual dom node
/// * key - the literal name of key attribute, ie: "key"
///
/// # Example
/// ```rust
/// use sauron::{diff::*, vdom::element, *};
///
///
/// let old: Node<()> = element(
///     "main",
///     vec![attr("class", "container")],
///     vec![
///         element("div", vec![attr("key", "1")], vec![]),
///         element("div", vec![attr("key", "2")], vec![]),
///     ],
/// );
///
/// let new: Node<()> = element(
///     "main",
///     vec![attr("class", "container")],
///     vec![element("div", vec![attr("key", "2")], vec![])],
/// );
///
/// let diff = diff(&old, &new).unwrap();
/// assert_eq!(
///     diff,
///     vec![Patch::remove_node(
///         Some(&"div"),
///         TreePath::new(vec![ 0]),
///     )
///     ]
/// );
/// ```
pub fn diff<'a, MSG>(
    old_node: &'a Node<MSG>,
    new_node: &'a Node<MSG>,
) -> Result<Vec<Patch<'a, MSG>>, DiffError> {
    diff_recursive(
        old_node,
        new_node,
        &SkipPath {
            path: TreePath::root(),
            skip_diff: None,
        },
    )
}

fn is_any_keyed<MSG>(nodes: &[Node<MSG>]) -> bool {
    nodes.iter().any(|child| is_keyed_node(child))
}

/// returns true any attributes of this node attribute has key in it
fn is_keyed_node<MSG>(node: &Node<MSG>) -> bool {
    if let Some(attributes) = node.attributes() {
        attributes.iter().any(|att| att.name == *KEY)
    } else {
        false
    }
}

fn should_replace<'a, MSG>(old_node: &'a Node<MSG>, new_node: &'a Node<MSG>) -> bool {
    // replace if they have different enum variants
    if mem::discriminant(old_node) != mem::discriminant(new_node) {
        return true;
    }
    let replace = |_old_node: &'a Node<MSG>, new_node: &'a Node<MSG>| {
        let explicit_replace_attr = new_node
            .first_value(REPLACE)
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        explicit_replace_attr
    };
    // handle explicit replace if the Rep fn evaluates to true
    if replace(old_node, new_node) {
        return true;
    }

    // replace if the old key does not match the new key
    if let (Some(old_key), Some(new_key)) =
        (old_node.attribute_value(KEY), new_node.attribute_value(KEY))
    {
        if old_key != new_key {
            return true;
        }
    }
    // replace if they have different element tag
    if let (Node::Element(old_element), Node::Element(new_element)) = (old_node, new_node) {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            return true;
        }
    }
    false
}

/// diff the nodes recursively
pub fn diff_recursive<'a, MSG>(
    old_node: &'a Node<MSG>,
    new_node: &'a Node<MSG>,
    path: &SkipPath,
) -> Result<Vec<Patch<'a, MSG>>, DiffError> {
    if let Some(skip_diff) = path.skip_diff.as_ref() {
        if USE_SKIP_DIFF && skip_diff.shall_skip_node() {
            return Err(DiffError::SkipDiffError);
        }
    }

    let skip = |old_node: &'a Node<MSG>, new_node: &'a Node<MSG>| {
        let new_skip_criteria = new_node.attribute_value(SKIP_CRITERIA);
        let old_skip_criteria = old_node.attribute_value(SKIP_CRITERIA);
        // if old and new skip_criteria didn't change skip diffing this nodes
        match (new_skip_criteria, old_skip_criteria) {
            (Some(new), Some(old)) => new == old,
            _ => new_node
                .first_value(SKIP)
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        }
    };
    // skip diffing if the function evaluates to true
    if skip(old_node, new_node) {
        return Ok(vec![]);
    }

    // multiple root nodes are not supported. this would be two root nodes "<div></div><div></div>"
    // the diff will work but the result is wrong, so instead we bail out here
    match new_node {
        Node::Leaf(leaf) => match leaf {
            Leaf::NodeList(list) => {
                if list.len() > 1 {
                    log::error!("invalid root node cound: input needs exactly one root node and childs, not several root nodes");
                    return Err(DiffError::InvalidRootNodeCount(list.len()));
                }
            }
            _ => {}
        },
        Node::Element(_) => {}
    }

    // replace node and return early
    if should_replace(old_node, new_node) {
        return Ok(vec![Patch::replace_node(
            old_node.tag(),
            path.path.clone(),
            vec![new_node],
        )]);
    }

    let mut patches = vec![];

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old_node, new_node) {
        (Node::Leaf(old_leaf), Node::Leaf(new_leaf)) => {
            match (old_leaf, new_leaf) {
                (Leaf::Text(_), Leaf::Text(_))
                | (Leaf::Symbol(_), Leaf::Symbol(_))
                | (Leaf::Comment(_), Leaf::Comment(_))
                | (Leaf::DocType(_), Leaf::DocType(_)) => {
                    if old_leaf != new_leaf {
                        let patch = Patch::replace_node(None, path.path.clone(), vec![new_node]);
                        patches.push(patch);
                    }
                }
                (Leaf::Fragment(old_nodes), Leaf::Fragment(new_nodes)) => {
                    // we back track since Fragment is not a real node, but it would still
                    // be traversed from the prior call
                    let patch = diff_nodes(None, old_nodes, new_nodes, &path.backtrack());
                    match patch {
                        Ok(patch) => patches.extend(patch),
                        Err(e) => return Err(e),
                    };
                }
                (Leaf::NodeList(_old_elements), Leaf::NodeList(_new_elements)) => {
                    return Err(DiffError::UnrollError)
                }
                (Leaf::StatelessComponent(old_comp), Leaf::StatelessComponent(new_comp)) => {
                    let new_path = SkipPath {
                        path: path.path.clone(),
                        skip_diff: old_comp.view.skip_diff(),
                    };

                    let old_real_view = old_comp.view.unwrap_template_ref();
                    let new_real_view = new_comp.view.unwrap_template_ref();

                    assert!(
                        !old_real_view.is_template(),
                        "old comp view should not be a template"
                    );
                    assert!(
                        !new_real_view.is_template(),
                        "new comp view should not be a template"
                    );
                    let patch = diff_recursive(old_real_view, new_real_view, &new_path);
                    match patch {
                        Ok(patch) => patches.extend(patch),
                        Err(e) => return Err(e),
                    }
                }
                (Leaf::StatefulComponent(old_comp), Leaf::StatefulComponent(new_comp)) => {
                    let attr_patches = create_attribute_patches(
                        &"component",
                        &old_comp.attrs,
                        &new_comp.attrs,
                        path,
                    );
                    match attr_patches {
                        Ok(attr_patches) => {
                            if !attr_patches.is_empty() {
                                log::info!("stateful component attr_patches: {attr_patches:#?}");
                            }
                            patches.extend(attr_patches);
                            let patch =
                                diff_nodes(None, &old_comp.children, &new_comp.children, path);
                            match patch {
                                Ok(patch) => {
                                    if !patch.is_empty() {
                                        log::info!("stateful component patch: {patch:#?}");
                                    }
                                    patches.extend(patch);
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                (Leaf::TemplatedView(_old_view), _) => {
                    unreachable!("templated view should not be diffed..")
                }
                (_, Leaf::TemplatedView(_new_view)) => {
                    unreachable!("templated view should not be diffed..")
                }
                _ => {
                    let patch = Patch::replace_node(None, path.path.clone(), vec![new_node]);
                    patches.push(patch);
                }
            }
        }
        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            let skip_attributes = if let Some(skip_diff) = path.skip_diff.as_ref() {
                USE_SKIP_DIFF && skip_diff.shall_skip_attributes()
            } else {
                false
            };

            if !skip_attributes {
                let attr_patches = create_attribute_patches(
                    old_element.tag(),
                    old_element.attributes(),
                    new_element.attributes(),
                    path,
                );
                match attr_patches {
                    Ok(attr_patches) => patches.extend(attr_patches),
                    Err(e) => return Err(e),
                };
            }

            let more_patches = diff_nodes(
                Some(old_element.tag()),
                old_element.children(),
                new_element.children(),
                path,
            );
            match more_patches {
                Ok(more_patches) => patches.extend(more_patches),
                Err(e) => return Err(e),
            };
        }
        _ => {
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    Ok(patches)
}

fn diff_nodes<'a, MSG>(
    old_tag: Option<&'a Tag>,
    old_children: &'a [Node<MSG>],
    new_children: &'a [Node<MSG>],
    path: &SkipPath,
) -> Result<Vec<Patch<'a, MSG>>, DiffError> {
    let diff_as_keyed = is_any_keyed(old_children) || is_any_keyed(new_children);

    if diff_as_keyed {
        let keyed_patches = diff_lis::diff_keyed_nodes(old_tag, old_children, new_children, path);
        Ok(keyed_patches)
    } else {
        let non_keyed_patches = diff_non_keyed_nodes(old_tag, old_children, new_children, path);
        non_keyed_patches
    }
}

/// In diffing non_keyed nodes,
///  we reuse existing DOM elements as much as possible
///
///  The algorithm used here is very simple.
///
///  If there are more children in the old_element than the new_element
///  the excess children is all removed.
///
///  If there are more children in the new_element than the old_element
///  it will be all appended in the old_element.
fn diff_non_keyed_nodes<'a, MSG>(
    old_element_tag: Option<&'a Tag>,
    old_children: &'a [Node<MSG>],
    new_children: &'a [Node<MSG>],
    path: &SkipPath,
) -> Result<Vec<Patch<'a, MSG>>, DiffError> {
    let mut patches: Vec<Patch<'a, MSG>> = vec![];
    let old_child_count = old_children.len();
    let new_child_count = new_children.len();

    // if there is no new children, then clear the children of this element
    if old_child_count > 0 && new_child_count == 0 {
        return Ok(vec![Patch::clear_children(
            old_element_tag,
            path.path.clone(),
        )]);
    }

    let min_count = cmp::min(old_child_count, new_child_count);
    for index in 0..min_count {
        // if we iterate trough the old elements, a new child_path is created for that iteration
        let child_path = path.traverse(index);

        let old_child = &old_children.get(index).expect("No old_node child node");
        let new_child = &new_children.get(index).expect("No new child node");

        let more_patches = diff_recursive(old_child, new_child, &child_path);
        match more_patches {
            Ok(more_patches) => patches.extend(more_patches),
            Err(e) => return Err(e),
        }
    }

    // If there are more new child than old_node child, we make a patch to append the excess element
    // starting from old_child_count to the last item of the new_elements
    if new_child_count > old_child_count {
        patches.push(Patch::append_children(
            old_element_tag,
            path.path.clone(),
            new_children.iter().skip(old_child_count).collect(),
        ));
    }

    if new_child_count < old_child_count {
        let remove_node_patches = old_children
            .iter()
            .skip(new_child_count)
            .enumerate()
            .map(|(i, old_child)| {
                Patch::remove_node(old_child.tag(), path.traverse(new_child_count + i).path)
            })
            .collect::<Vec<_>>();

        patches.extend(remove_node_patches);
    }

    Ok(patches)
}

///
/// Note: The performance bottlenecks
///     - allocating new vec
///     - merging attributes of the same name
#[allow(clippy::type_complexity)]
fn create_attribute_patches<'a, MSG>(
    old_tag: &'a Tag,
    old_attributes: &'a [Attribute<MSG>],
    new_attributes: &'a [Attribute<MSG>],
    path: &SkipPath,
) -> Result<Vec<Patch<'a, MSG>>, DiffError> {
    let skip_indices = if let Some(skip_diff) = &path.skip_diff {
        if let SkipAttrs::Indices(skip_indices) = &skip_diff.skip_attrs {
            skip_indices.clone()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let has_skip_indices = !skip_indices.is_empty();

    let mut patches = vec![];

    // return early if both attributes are empty
    if old_attributes.is_empty() && new_attributes.is_empty() {
        return Ok(vec![]);
    }

    let mut add_attributes: Vec<&Attribute<MSG>> = vec![];
    let mut remove_attributes: Vec<&Attribute<MSG>> = vec![];

    let new_attributes_grouped = Element::group_indexed_attributes_per_name(new_attributes);
    let old_attributes_grouped = Element::group_indexed_attributes_per_name(old_attributes);

    // for all new elements that doesn't exist in the old elements
    // or the values differ
    // add it to the AddAttribute patches
    for (new_attr_name, new_attrs) in new_attributes_grouped.iter() {
        let old_indexed_attr_values = old_attributes_grouped.get(new_attr_name).map(|attrs| {
            attrs
                .iter()
                .map(|(i, attr)| (*i, &attr.value))
                .collect::<Vec<_>>()
        });

        let new_indexed_attr_values = new_attributes_grouped.get(new_attr_name).map(|attrs| {
            attrs
                .iter()
                .map(|(i, attr)| (*i, &attr.value))
                .collect::<Vec<_>>()
        });

        if let Some(old_indexed_attr_values) = old_indexed_attr_values {
            let new_indexed_attr_values =
                new_indexed_attr_values.expect("must have new attr values");
            let (_new_indices, new_attr_values): (Vec<usize>, Vec<&Vec<AttributeValue<MSG>>>) =
                new_indexed_attr_values.into_iter().unzip();
            let (old_indices, old_attr_values): (Vec<usize>, Vec<&Vec<AttributeValue<MSG>>>) =
                old_indexed_attr_values.into_iter().unzip();
            if USE_SKIP_DIFF && has_skip_indices && is_subset_of(&old_indices, &skip_indices) {
                //
            } else if old_attr_values != new_attr_values {
                for (_i, new_att) in new_attrs {
                    add_attributes.push(new_att);
                }
            }
        } else {
            // these are new attributes
            for (_i, new_att) in new_attrs {
                add_attributes.push(new_att);
            }
        }
    }

    // if this attribute name does not exist anymore
    // to the new element, remove it
    for (old_attr_name, old_indexed_attrs) in old_attributes_grouped.into_iter() {
        let (old_indices, old_attrs): (Vec<usize>, Vec<&Attribute<MSG>>) =
            old_indexed_attrs.into_iter().unzip();
        if USE_SKIP_DIFF && has_skip_indices && is_subset_of(&old_indices, &skip_indices) {
            //
        } else if !new_attributes_grouped.contains_key(old_attr_name) {
            remove_attributes.extend(old_attrs.clone());
        }
    }

    if !add_attributes.is_empty() {
        patches.push(Patch::add_attributes(
            old_tag,
            path.path.clone(),
            add_attributes,
        ));
    }
    if !remove_attributes.is_empty() {
        patches.push(Patch::remove_attributes(
            old_tag,
            path.path.clone(),
            remove_attributes,
        ));
    }
    Ok(patches)
}

/// returns true if all the elements in subset is in big_set
/// This also returns the indices of big_set that are not found in the subset
fn is_subset_of<T: PartialEq>(subset: &[T], big_set: &[T]) -> bool {
    subset.iter().all(|set| big_set.contains(set))
}
