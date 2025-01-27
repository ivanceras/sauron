use sauron::vdom::{diff::*, patch::*, *};

#[test]
fn key_lis_1_to_9() {
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
            element("div", vec![attr("key", "XXX1")], vec![leaf("lineXXX")]),
            element("div", vec![attr("key", "XXX2")], vec![leaf("lineXXX")]),
            element("div", vec![attr("key", "XXX3")], vec![leaf("lineXXX")]),
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
            element("div", vec![attr("key", "XXX4")], vec![leaf("lineXXX")]),
            element("div", vec![attr("key", "XXX5")], vec![leaf("lineXXX")]),
            element("div", vec![attr("key", "XXX6")], vec![leaf("lineXXX")]),
        ],
    );

    let diff = diff(&old, &new).unwrap();

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::insert_after_node(
                Some(&"div"),
                TreePath::new(vec![8]),
                vec![
                    &element("div", vec![attr("key", "XXX4")], vec![leaf("lineXXX")]),
                    &element("div", vec![attr("key", "XXX5")], vec![leaf("lineXXX")]),
                    &element("div", vec![attr("key", "XXX6")], vec![leaf("lineXXX")]),
                ]
            ),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![0]),
                vec![
                    &element("div", vec![attr("key", "XXX1")], vec![leaf("lineXXX")]),
                    &element("div", vec![attr("key", "XXX2")], vec![leaf("lineXXX")]),
                    &element("div", vec![attr("key", "XXX3")], vec![leaf("lineXXX")]),
                ]
            )
        ]
    );
}
