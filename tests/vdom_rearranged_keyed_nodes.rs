use sauron::vdom::{diff::*, patch::*, *};

#[test]
fn text_changed_keyed() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container"), attr("key", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container"), attr("key", "container")],
        vec![
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
        ],
    );

    let diff = diff(&old, &new).unwrap();
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::move_before_node(
            Some(&"div"),
            TreePath::new([0]),
            [TreePath::new([2]), TreePath::new([1])]
        )]
    );
}
