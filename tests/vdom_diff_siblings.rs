use sauron::diff::diff_sibling_nodes;
use sauron::*;

#[test]
fn test_siblings_insert_after() {
    let old: Node<()> = node! {
    <ul>
        <li>item1</li>
        <li>item2</li>
        <li>item3</li>
        <li>item4</li>
    </ul>
    };
    let old_children = old.children();

    let new: Node<()> = node! {
    <ul>
        <li>item1</li>
        <li>item2</li>
        <li>item3</li>
        <li>item4</li>
        <li>item5</li>
        <li>item6</li>
    </ul>
    };
    let new_children = new.children();

    let patch = diff_sibling_nodes(
        Some(&"li"),
        old_children,
        new_children,
        &TreePath::new([0]),
        Some(0),
    );
    println!("patch: {patch:?}");
    assert_eq!(
        patch,
        vec![Patch::insert_after_node(
            Some(&"li"),
            TreePath::new([0]),
            vec![&li([], [text("item5")]), &li([], [text("item6")])]
        )]
    );
}

#[test]
fn test_siblings_removed() {
    let old: Node<()> = node! {
    <ul>
        <li>item1</li>
        <li>item2</li>
        <li>item3</li>
        <li>item4</li>
        <li>item5</li>
        <li>item6</li>
    </ul>
    };
    let old_children = old.children();

    let new: Node<()> = node! {
    <ul>
        <li>item1</li>
        <li>item2</li>
        <li>item3</li>
        <li>item4</li>
    </ul>
    };
    let new_children = new.children();

    let patch = diff_sibling_nodes(
        Some(&"li"),
        old_children,
        new_children,
        &TreePath::new([0]),
        Some(0),
    );
    println!("patch: {patch:?}");
    assert_eq!(
        patch,
        vec![
            Patch::remove_node(Some(&"li"), TreePath::new([4]),),
            Patch::remove_node(Some(&"li"), TreePath::new([5]),)
        ]
    );
}
