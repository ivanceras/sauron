#![deny(warnings)]
use sauron::vdom::{patch::*, *};

#[test]
fn test_replace_node() {
    let old: Node<()> = element("div", vec![], vec![]);
    let new = element("span", vec![], vec![]);

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![]),
            vec![&new]
        )],
    );
}

#[test]
fn test_replace_text_node() {
    let old: Node<()> = leaf("hello");
    let new = element("span", vec![], vec![]);

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::replace_node(None, TreePath::new(vec![]), vec![&new])],
    );
}

#[test]
fn test_replace_node_in_child() {
    let old: Node<()> =
        element("main", vec![], vec![element("div", vec![], vec![])]);
    let new = element("main", vec![], vec![element("span", vec![], vec![])]);

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![0]),
            vec![&element("span", vec![], vec![])]
        )],
        "Should replace the first node"
    );
}

#[test]
fn test_205() {
    let old: Node<()> = element(
        "div",
        vec![],
        vec![
            element(
                "b",
                vec![],
                vec![
                    element("i", vec![], vec![]),
                    element("i", vec![], vec![]),
                ],
            ),
            element("b", vec![], vec![]),
        ],
    ); //{ <div> <b> <i></i> <i></i> </b> <b></b> </div> },

    assert_eq!(5, old.node_count());
    let new = element(
        "div",
        vec![],
        vec![
            element("b", vec![], vec![element("i", vec![], vec![])]),
            element("i", vec![], vec![]),
        ],
    ); //{ <div> <b> <i></i> </b> <i></i> </div>},
    assert_eq!(
        dbg!(diff(&old, &new)),
        vec![
            Patch::remove_node(Some(&"i"), TreePath::new(vec![0, 1]),),
            Patch::replace_node(
                Some(&"b"),
                TreePath::new(vec![1]),
                vec![&element("i", vec![], vec![])]
            ),
        ],
    )
}

#[test]
fn test_no_changed() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![])
}

#[test]
fn test_attribute_order_changed() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("class", "some-class"), attr("id", "some-id")],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![])
}

#[test]
fn test_class_changed() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class2")],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::add_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&attr("class", "some-class2")]
        )]
    )
}

#[test]
fn leaf_node_changed() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![leaf("text1")],
    );

    let new = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![leaf("text2")],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            None,
            TreePath::new(vec![0]),
            vec![&leaf("text2")]
        )]
    )
}

#[test]
fn test_class_will_not_be_merged_on_different_calls() {
    let old: Node<()> = element("div", vec![], vec![]);

    let new = element(
        "div",
        vec![attr("class", "class1"), attr("class", "class2")],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_ne!(
        diff,
        vec![Patch::add_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&Attribute::with_multiple_values(
                None,
                "class",
                vec!["class1".into(), "class2".into()]
            )]
        )]
    )
}

#[test]
fn test_class_removed() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new = element("div", vec![attr("id", "some-id")], vec![]);

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::remove_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&attr("class", "some-class")]
        )]
    )
}

#[test]
fn test_multiple_calls_to_style() {
    let old: Node<()> = element(
        "div",
        vec![
            attr("style", "display:flex"),
            attr("style", "width:100px;height:100px"),
        ],
        vec![],
    );

    let new = element(
        "div",
        vec![
            attr("style", "display:flex"),
            attr("style", "width:200px;height:200px"),
        ],
        vec![],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::add_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![
                &attr("style", "display:flex"),
                &attr("style", "width:200px;height:200px"),
            ]
        )]
    )
}

#[test]
fn inner_html_func_calls() {
    let old: Node<()> = element("div", vec![], vec![]);

    let new: Node<()> =
        element("div", vec![attr("inner_html", "<h1>Hello</h2>")], vec![]);

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::add_attributes(
            &"div",
            TreePath::new(vec![]),
            vec![&attr("inner_html", "<h1>Hello</h2>")]
        )]
    )
}

#[test]
fn test_append() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![element("div", vec![], vec![leaf("1")])],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![], vec![leaf("2")]),
        ],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::append_children(
            Some(&"div"),
            TreePath::new(vec![]),
            vec![&element("div", vec![], vec![leaf("2")])],
        )]
    )
}

#[test]
fn test_append_more() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![element("div", vec![], vec![leaf("1")])],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::append_children(
            Some(&"div"),
            TreePath::new(vec![]),
            vec![
                &element("div", vec![], vec![leaf("2")]),
                &element("div", vec![], vec![leaf("3")])
            ],
        )]
    )
}

#[test]
fn test_append_at_sub_level() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![element(
            "main",
            vec![],
            vec![element("div", vec![], vec![leaf("1")])],
        )],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![element(
            "main",
            vec![],
            vec![
                element("div", vec![], vec![leaf("1")]),
                element("div", vec![], vec![leaf("2")]),
                element("div", vec![], vec![leaf("3")]),
            ],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![Patch::append_children(
            Some(&"main"),
            TreePath::new(vec![0]),
            vec![
                &element("div", vec![], vec![leaf("2")]),
                &element("div", vec![], vec![leaf("3")])
            ],
        )]
    )
}
