use sauron::vdom::{diff::*, patch::*, *};

#[test]
fn test_multiple_replace() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "multi_replace")],
        vec![element(
            "ul",
            vec![attr("class", "todo")],
            vec![
                element("li", vec![attr("key", "1")], vec![leaf("item1")]),
                element("li", vec![attr("key", "2")], vec![leaf("item2")]),
                element("li", vec![attr("key", "3")], vec![leaf("item3")]),
            ],
        )],
    );

    // we remove the key1
    let update1: Node<()> = element(
        "main",
        vec![attr("class", "multi_replace")],
        vec![element(
            "ul",
            vec![attr("class", "todo")],
            vec![
                element("li", vec![attr("key", "10")], vec![leaf("item10")]),
                element("li", vec![attr("key", "20")], vec![leaf("item20")]),
                element("li", vec![attr("key", "30")], vec![leaf("item30")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);

    dbg!(&patches);

    assert_eq!(
        patches,
        vec![
            Patch::remove_node(Some(&"li"), TreePath::new(vec![0, 1]),),
            Patch::remove_node(Some(&"li"), TreePath::new(vec![0, 2]),),
            Patch::replace_node(
                Some(&"li"),
                TreePath::new(vec![0, 0]),
                vec![
                    &element("li", vec![attr("key", "10")], vec![leaf("item10")]),
                    &element("li", vec![attr("key", "20")], vec![leaf("item20")]),
                    &element("li", vec![attr("key", "30")], vec![leaf("item30")]),
                ],
            ),
        ]
    );
}
