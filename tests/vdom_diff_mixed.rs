use sauron::vdom::{diff::*, patch::*, *};

// should have no changes
#[test]
fn mixed_key_and_no_key_with_no_change() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(diff, vec![]);
}

#[test]
fn mixed_key_and_no_key_with_2_matched() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("3")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            Patch::replace_node(None, TreePath::new(vec![1, 0]), vec![&leaf("1")]),
            Patch::replace_node(None, TreePath::new(vec![2, 0]), vec![&leaf("3")]),
        ]
    );
}

#[test]
fn mixed_key_and_no_key_with_misordered_2_matched() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::remove_node(Some(&"div"), TreePath::new(vec![1])),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![0]),
                vec![&element("div", vec![], vec![leaf("1")])],
            ),
        ]
    );
}
