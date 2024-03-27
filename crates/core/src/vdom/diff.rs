//! provides diffing algorithm which returns patches
use super::{diff_lis, Attribute, Element, Node, Patch, TreePath};
use super::{Tag, KEY, REPLACE, SKIP, SKIP_CRITERIA};
use crate::vdom::Leaf;
use std::{cmp, mem};
use crate::dom::SkipPath;

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
/// let diff = diff(&old, &new);
/// assert_eq!(
///     diff,
///     vec![Patch::remove_node(
///         Some(&"div"),
///         TreePath::new(vec![ 0]),
///     )
///     ]
/// );
/// ```
pub fn diff<'a, MSG>(old_node: &'a Node<MSG>, new_node: &'a Node<MSG>) -> Vec<Patch<'a, MSG>> {
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
        log::warn!("different discriminant: old_node: {:#?}", old_node);
        log::warn!("different discriminant: new_node: {:#?}", new_node);
        return true;
    }
    let replace = |_old_node: &'a Node<MSG>, new_node: &'a Node<MSG>| {
        let explicit_replace_attr = new_node
            .first_value(&REPLACE)
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
) -> Vec<Patch<'a, MSG>> {
    if let Some(skip_diff) = path.skip_diff.as_ref() {
        if skip_diff.shall_skip_node() {
            return vec![];
        }
    }

    let skip = |old_node: &'a Node<MSG>, new_node: &'a Node<MSG>| {
        let new_skip_criteria = new_node.attribute_value(&SKIP_CRITERIA);
        let old_skip_criteria = old_node.attribute_value(&SKIP_CRITERIA);
        // if old and new skip_criteria didn't change skip diffing this nodes
        match (new_skip_criteria, old_skip_criteria) {
            (Some(new), Some(old)) => new == old,
            _ => new_node
                .first_value(&SKIP)
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        }
    };
    // skip diffing if the function evaluates to true
    if skip(old_node, new_node) {
        return vec![];
    }

    // replace node and return early
    if should_replace(old_node, new_node) {
        return vec![Patch::replace_node(
            old_node.tag(),
            path.path.clone(),
            vec![new_node],
        )];
    }

    let mut patches = vec![];

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old_node, new_node) {
        (Node::Leaf(old_leaf), Node::Leaf(new_leaf)) => {
            match (old_leaf, new_leaf) {
                (Leaf::Text(_), Leaf::Text(_))
                | (Leaf::SafeHtml(_), Leaf::SafeHtml(_))
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
                    patches.extend(patch);
                }
                (Leaf::NodeList(_old_elements), Leaf::NodeList(_new_elements)) => {
                    panic!("Node list must have already unrolled when creating an element");
                }
                (Leaf::StatelessComponent(old_comp), Leaf::StatelessComponent(new_comp)) => {
                    log::info!("diffing stateless component...");
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
                    patches.extend(patch);
                }
                (Leaf::StatefulComponent(old_comp), Leaf::StatefulComponent(new_comp)) => {
                    log::info!("diffing stateful component");
                    let patch = diff_nodes(None, &old_comp.children, &new_comp.children, &path);
                    patches.extend(patch);
                }
                (Leaf::TemplatedView(_old_view), _) => {
                    unreachable!("templated view should not be diffed..")
                }
                (_, Leaf::TemplatedView(_new_view)) => {
                    unreachable!("templated view should not be diffed..")
                }
                _ => {
                    log::info!("replace patch here...");
                    let patch = Patch::replace_node(None, path.path.clone(), vec![new_node]);
                    patches.push(patch);
                }
            }
        }
        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            let skip_attributes = if let Some(skip_diff) = path.skip_diff.as_ref() {
                skip_diff.shall
            } else {
                false
            };

            if !skip_attributes {
                let attr_patches = create_attribute_patches(old_element, new_element, path);
                patches.extend(attr_patches);
            }

            let more_patches = diff_nodes(
                Some(old_element.tag()),
                &old_element.children(),
                &new_element.children(),
                path,
            );

            patches.extend(more_patches);
        }
        _ => {
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    patches
}

fn diff_nodes<'a, MSG>(
    old_tag: Option<&'a Tag>,
    old_children: &'a [Node<MSG>],
    new_children: &'a [Node<MSG>],
    path: &SkipPath,
) -> Vec<Patch<'a, MSG>> {
    let diff_as_keyed = is_any_keyed(old_children) || is_any_keyed(new_children);

    if diff_as_keyed {
        let keyed_patches = diff_lis::diff_keyed_nodes(old_tag, old_children, new_children, path);
        keyed_patches
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
) -> Vec<Patch<'a, MSG>> {
    let mut patches = vec![];
    let old_child_count = old_children.len();
    let new_child_count = new_children.len();

    // if there is no new children, then clear the children of this element
    if old_child_count > 0 && new_child_count == 0 {
        return vec![Patch::clear_children(old_element_tag, path.path.clone())];
    }

    let min_count = cmp::min(old_child_count, new_child_count);
    for index in 0..min_count {
        // if we iterate trough the old elements, a new child_path is created for that iteration
        let child_path = path.traverse(index);

        let old_child = &old_children.get(index).expect("No old_node child node");
        let new_child = &new_children.get(index).expect("No new child node");

        let more_patches = diff_recursive(old_child, new_child, &child_path);
        patches.extend(more_patches);
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

    patches
}

///
/// Note: The performance bottlenecks
///     - allocating new vec
///     - merging attributes of the same name
fn create_attribute_patches<'a, MSG>(
    old_element: &'a Element<MSG>,
    new_element: &'a Element<MSG>,
    path: &SkipPath,
) -> Vec<Patch<'a, MSG>> {
    let new_attributes = new_element.attributes();
    let old_attributes = old_element.attributes();

    let mut patches = vec![];

    // return early if both attributes are empty
    if old_attributes.is_empty() && new_attributes.is_empty() {
        return vec![];
    }

    let mut add_attributes: Vec<&Attribute<MSG>> = vec![];
    let mut remove_attributes: Vec<&Attribute<MSG>> = vec![];

    let new_attributes_grouped = Attribute::group_attributes_per_name(new_attributes.iter());
    let old_attributes_grouped = Attribute::group_attributes_per_name(old_attributes.iter());

    // for all new elements that doesn't exist in the old elements
    // or the values differ
    // add it to the AddAttribute patches
    for (new_attr_name, new_attrs) in new_attributes_grouped.iter() {
        let old_attr_values = old_attributes_grouped
            .get(new_attr_name)
            .map(|attrs| attrs.iter().map(|attr| &attr.value).collect::<Vec<_>>());

        let new_attr_values = new_attributes_grouped
            .get(new_attr_name)
            .map(|attrs| attrs.iter().map(|attr| &attr.value).collect::<Vec<_>>());

        if let Some(old_attr_values) = old_attr_values {
            let new_attr_values = new_attr_values.expect("must have new attr values");
            if old_attr_values != new_attr_values {
                add_attributes.extend(new_attrs);
            }
        } else {
            add_attributes.extend(new_attrs);
        }
    }

    // if this attribute name does not exist anymore
    // to the new element, remove it
    for (old_attr_name, old_attrs) in old_attributes_grouped.iter() {
        if !new_attributes_grouped.contains_key(old_attr_name) {
            remove_attributes.extend(old_attrs);
        }
    }

    if !add_attributes.is_empty() {
        patches.push(Patch::add_attributes(
            &old_element.tag,
            path.path.clone(),
            add_attributes,
        ));
    }
    if !remove_attributes.is_empty() {
        patches.push(Patch::remove_attributes(
            &old_element.tag,
            path.path.clone(),
            remove_attributes,
        ));
    }
    patches
}
