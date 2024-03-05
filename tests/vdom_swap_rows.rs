use sauron::vdom::{diff::*, patch::*, *};

#[test]
fn swap_999() {
    let old: Node<()> = element(
        "ul",
        vec![attr("class", "container")],
        (0..1000).map(|i| {
            element(
                "li",
                vec![attr("key", i.to_string())],
                vec![leaf(format!("line{i}"))],
            )
        }),
    );

    let mut range: Vec<usize> = (0..1000).collect();
    range.swap(1, 998);

    let new: Node<()> = element(
        "ul",
        vec![attr("class", "container")],
        range.iter().map(|i| {
            element(
                "li",
                vec![attr("key", i.to_string())],
                vec![leaf(format!("line{i}"))],
            )
        }),
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::move_before_node(
                Some(&"li"),
                TreePath::new([1]),
                [TreePath::new([998])]
            ),
            Patch::move_after_node(
                Some(&"li"),
                TreePath::new([997]),
                [TreePath::new([1])]
            ),
        ]
    );
}

#[test]
fn swap_rows_non_keyed() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("class", "1")], vec![leaf("line1")]),
            element("div", vec![attr("class", "2")], vec![leaf("line2")]),
            element("div", vec![attr("class", "3")], vec![leaf("line3")]),
            element("div", vec![attr("class", "4")], vec![leaf("line4")]),
            element("div", vec![attr("class", "5")], vec![leaf("line5")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("class", "1")], vec![leaf("line1")]),
            element("div", vec![attr("class", "4")], vec![leaf("line4")]),
            element("div", vec![attr("class", "3")], vec![leaf("line3")]),
            element("div", vec![attr("class", "2")], vec![leaf("line2")]),
            element("div", vec![attr("class", "5")], vec![leaf("line5")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::add_attributes(
                &"div",
                TreePath::new([1]),
                vec![&attr("class", "4")],
            ),
            Patch::replace_node(
                None,
                TreePath::new([1, 0]),
                vec![&leaf("line4")]
            ),
            Patch::add_attributes(
                &"div",
                TreePath::new([3],),
                [&attr("class", "2")],
            ),
            Patch::replace_node(None, TreePath::new([3, 0],), [&leaf("line2")],)
        ]
    );
}

#[test]
fn move_key_2_to_after_node_index_6() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::move_after_node(
            Some(&"div",),
            TreePath::new([5]),
            [TreePath::new([1])]
        ),]
    );
}

#[test]
fn move_key_7_to_before_node_index_1() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::move_before_node(
            Some(&"div",),
            TreePath::new([1]),
            [TreePath::new([6])]
        ),]
    );
}

#[test]
fn swap_rows_keyed() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::move_before_node(
                Some(&"div"),
                TreePath::new([1]),
                [TreePath::new([6])]
            ),
            Patch::move_after_node(
                Some(&"div",),
                TreePath::new([5]),
                [TreePath::new([1])]
            ),
        ]
    );
}

#[test]
fn swap_rows_keyed_6_items() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::move_before_node(
                Some(&"div",),
                TreePath::new([1]),
                [TreePath::new([4])]
            ),
            Patch::move_after_node(
                Some(&"div"),
                TreePath::new([3]),
                [TreePath::new([1])]
            ),
        ]
    );
}

#[test]
fn swap_rows_keyed_5_items() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "k1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "k2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "k3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "k4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "k5")], vec![leaf("line5")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "k1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "k4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "k3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "k2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "k5")], vec![leaf("line5")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    // k2 is the known lis, so we need to move k3 and k4
    assert_eq!(
        diff,
        vec![Patch::move_before_node(
            Some(&"div",),
            TreePath::new([1]),
            [TreePath::new([3]), TreePath::new([2])]
        ),]
    );
}
