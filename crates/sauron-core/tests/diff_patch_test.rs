#![deny(warnings)]
use sauron_core::{
    diff,
    html::{
        attributes::*,
        events::*,
        *,
    },
    mt_dom::patch::*,
    Attribute,
    Node,
    *,
};

#[test]
fn event_remove() {
    let elem_id = "input-remove-event-test";

    let old: Node<()> = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
            on_input(move |_event: InputEvent| {
                println!("input event is triggered");
            }),
        ],
        vec![],
    );

    let new = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
        ],
        vec![],
    );
    let patch = diff(&old, &new);
    println!("patch: {:#?}", patch);

    assert_eq!(
        patch,
        vec![RemoveAttributes::new(
            &"input",
            0,
            0,
            vec![&on("input", |_| { () })],
        )
        .into()]
    );
}

#[test]
fn change_class_attribute() {
    let old: Node<()> = div(vec![classes(["class1", "class2"])], vec![]);

    let new = div(vec![classes(["class1", "difference_class"])], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![AddAttributes::new(
            &"div",
            0,
            0,
            vec![&Attribute::with_multiple_values(
                None,
                "class",
                vec![
                    AttributeValue::from_value("class1".to_string().into()),
                    AttributeValue::from_value(
                        "difference_class".to_string().into()
                    )
                ]
            )]
        )
        .into()],
        "Should add the new attributes"
    );
}

#[test]
fn truncate_children() {
    let old: Node<()> = div(
        vec![],
        vec![
            div(vec![class("class1")], vec![]),
            div(vec![class("class2")], vec![]),
            div(vec![class("class3")], vec![]),
            div(vec![class("class4")], vec![]),
            div(vec![class("class5")], vec![]),
            div(vec![class("class6")], vec![]),
            div(vec![class("class7")], vec![]),
        ],
    );

    let new = div(
        vec![],
        vec![
            div(vec![class("class1")], vec![]),
            div(vec![class("class2")], vec![]),
            div(vec![class("class3")], vec![]),
        ],
    );
    assert_eq!(
        diff(&old, &new),
        vec![
            RemoveNode::new(Some(&"div"), 4).into(),
            RemoveNode::new(Some(&"div"), 5).into(),
            RemoveNode::new(Some(&"div"), 6).into(),
            RemoveNode::new(Some(&"div"), 7).into(),
        ],
        "Should truncate children"
    );
}

#[test]
fn truncate_children_different_attributes() {
    let old: Node<()> = div(
        vec![],
        vec![
            div(vec![class("class1")], vec![]),
            div(vec![class("class2")], vec![]),
            div(vec![class("class3")], vec![]),
            div(vec![class("class4")], vec![]),
            div(vec![class("class5")], vec![]),
            div(vec![class("class6")], vec![]),
            div(vec![class("class7")], vec![]),
        ],
    );

    let new = div(
        vec![],
        vec![
            div(vec![class("class5")], vec![]),
            div(vec![class("class6")], vec![]),
            div(vec![class("class7")], vec![]),
        ],
    );

    let patch = diff(&old, &new);
    dbg!(&patch);

    assert_eq!(
        patch,
        vec![
            AddAttributes::new(&"div", 1, 1, vec![&class("class5")]).into(),
            AddAttributes::new(&"div", 2, 2, vec![&class("class6")]).into(),
            AddAttributes::new(&"div", 3, 3, vec![&class("class7")]).into(),
            RemoveNode::new(Some(&"div"), 4).into(),
            RemoveNode::new(Some(&"div"), 5).into(),
            RemoveNode::new(Some(&"div"), 6).into(),
            RemoveNode::new(Some(&"div"), 7).into(),
        ],
        "Should truncate children"
    );
}

#[test]
fn replace_node() {
    let old: Node<()> = div(vec![], vec![]);
    let new = span(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![ReplaceNode::new(Some(&"div"), 0, 0, &span(vec![], vec![])).into()],
        "ReplaceNode the root if the tag changed"
    );

    let old: Node<()> = div(vec![], vec![b(vec![], vec![])]);
    let new = div(vec![], vec![strong(vec![], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![ReplaceNode::new(Some(&"b"), 1, 1, &strong(vec![], vec![])).into()],
        "ReplaceNode a child node"
    );
}

#[test]
fn replace_node2() {
    let old: Node<()> =
        div(vec![], vec![b(vec![], vec![text("1")]), b(vec![], vec![])]);
    let new = div(vec![], vec![i(vec![], vec![text("1")]), i(vec![], vec![])]);

    let patch = diff(&old, &new);
    dbg!(&patch);
    assert_eq!(
        patch,
        vec![
            ReplaceNode::new(Some(&"b"), 1, 1, &i(vec![], vec![text("1")]))
                .into(),
            ReplaceNode::new(Some(&"b"), 3, 3, &i(vec![], vec![])).into(),
        ],
        "ReplaceNode node with a child",
    )
}

#[test]
fn add_children() {
    let old: Node<()> = div(vec![], vec![b(vec![], vec![])]); //{ <div> <b></b> </div> },
    let new: Node<()> = div(
        vec![],
        vec![b(vec![], vec![]), html_element("new", vec![], vec![])],
    ); //{ <div> <b></b> <new></new> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![AppendChildren::new(
            &"div",
            0,
            vec![(2, &html_element("new", vec![], vec![]))]
        )
        .into()],
        "Added a new node to the root node",
    )
}

#[test]
fn remove_nodes() {
    let old: Node<()> =
        div(vec![], vec![b(vec![], vec![]), span(vec![], vec![])]); //{ <div> <b></b> <span></span> </div> },
    let new = div(vec![], vec![]); //{ <div> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![
            RemoveNode::new(Some(&"b"), 1).into(),
            RemoveNode::new(Some(&"span"), 2).into(),
        ],
        "Remove all child nodes at and after child sibling index 1",
    );

    let old: Node<()> = div(
        vec![],
        vec![
            span(
                vec![],
                vec![
                    b(vec![], vec![]),
                    // This `i` tag will get removed
                    i(vec![], vec![]),
                ],
            ),
            // This `strong` tag will get removed
            strong(vec![], vec![]),
        ],
    );

    let new: Node<()> =
        div(vec![], vec![span(vec![], vec![b(vec![], vec![])])]);

    assert_eq!(
        diff(&old, &new),
        vec![
            RemoveNode::new(Some(&"i"), 3).into(),
            RemoveNode::new(Some(&"strong"), 4).into(),
        ],
        "Remove a child and a grandchild node",
    );

    let old: Node<()> = div(
        vec![],
        vec![
            b(vec![], vec![i(vec![], vec![]), i(vec![], vec![])]),
            b(vec![], vec![]),
        ],
    ); //{ <div> <b> <i></i> <i></i> </b> <b></b> </div> },
    let new = div(
        vec![],
        vec![b(vec![], vec![i(vec![], vec![])]), i(vec![], vec![])],
    ); //{ <div> <b> <i></i> </b> <i></i> </div>},
    assert_eq!(
        diff(&old, &new),
        vec![
            RemoveNode::new(Some(&"i"), 3).into(),
            ReplaceNode::new(Some(&"b"), 4, 3, &i(vec![], vec![])).into(),
        ],
        "Removing child and change next node after parent",
    )
}

#[test]
fn add_attributes() {
    let old: Node<()> = div(vec![], vec![]); //{ <div> </div> },
    let new = div(vec![id("hello")], vec![]); //{ <div id="hello"> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![AddAttributes::new(&"div", 0, 0, vec![&id("hello")]).into()],
        "Add attributes",
    );

    let old: Node<()> = div(vec![id("foobar")], vec![]); //{ <div id="foobar"> </div> },
    let new = div(vec![id("hello")], vec![]); //{ <div id="hello"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![AddAttributes::new(&"div", 0, 0, vec![&id("hello")]).into()],
        "Change attribute",
    );
}

#[test]
fn add_style_attributes() {
    let old: Node<()> = div(vec![style("display", "block")], vec![]);
    let new = div(vec![style("display", "none")], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![
            AddAttributes::new(&"div", 0, 0, vec![&style("display", "none")])
                .into()
        ],
        "Add attributes",
    );
}

#[test]
fn add_style_attributes_1_change() {
    let old: Node<()> = div(
        vec![styles([("display", "block"), ("position", "absolute")])],
        vec![],
    );
    let new = div(
        vec![styles([("display", "none"), ("position", "absolute")])],
        vec![],
    );
    assert_eq!(
        diff(&old, &new),
        vec![AddAttributes::new(
            &"div",
            0,
            0,
            vec![&Attribute::with_multiple_values(
                None,
                "style",
                vec![AttributeValue::Style(vec![
                    Style::new("display", "none".into()),
                    Style::new("position", "absolute".into())
                ])]
            )]
        )
        .into()],
    );
}

#[test]
fn add_style_attributes_no_changes() {
    let old: Node<()> = div(
        vec![styles([("display", "block"), ("position", "absolute")])],
        vec![],
    );
    let new = div(
        vec![styles([("display", "block"), ("position", "absolute")])],
        vec![],
    );
    assert_eq!(diff(&old, &new), vec![],);
}

#[test]
fn remove_style_attributes() {
    let old: Node<()> = div(vec![style("display", "block")], vec![]);
    let new = div(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![RemoveAttributes::new(
            &"div",
            0,
            0,
            vec![&style("display", "block")]
        )
        .into()],
        "Add attributes",
    );
}

#[test]
fn remove_attributes() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]);
    let new = div(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![RemoveAttributes::new(&"div", 0, 0, vec![&id("hey-there")]).into()],
        "Remove attributes",
    );
}

#[test]
fn remove_events() {
    let old: Node<()> = div(vec![on_click(|_| println!("hi"))], vec![]);
    let new = div(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![RemoveAttributes::new(
            &"div",
            0,
            0,
            vec![&on_click(|_| println!("hi"))]
        )
        .into()],
        "Remove events",
    );
}

#[test]
fn change_attribute() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]); //{ <div id="hey-there"></div> },
    let new = div(vec![id("changed")], vec![]); //{ <div id="changed"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![AddAttributes::new(&"div", 0, 0, vec![&id("changed")]).into()],
        "Add attributes",
    );
}

#[test]
fn replace_text_node() {
    let old: Node<()> = text("Old"); //{ Old },
    let new: Node<()> = text("New"); //{ New },

    assert_eq!(
        diff(&old, &new),
        vec![ChangeText::new(0, &Text::new("Old"), 0, &Text::new("New")).into()],
    );
}

#[test]
fn text_changed_in_keyed_elements() {
    let old: Node<()> = main(
        vec![class("test4")],
        vec![section(
            vec![class("todo")],
            vec![
                article(vec![key(1)], vec![text("item1")]),
                article(vec![key(2)], vec![text("item2")]),
                article(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    // we remove the key1
    let update1: Node<()> = main(
        vec![class("test4")],
        vec![section(
            vec![class("todo")],
            vec![
                article(vec![key(2)], vec![text("item2")]),
                article(vec![key(3)], vec![text("item3 with changes")]),
            ],
        )],
    );

    let patch = diff(&old, &update1);
    assert_eq!(
        patch,
        vec![
            ChangeText::new(
                7,
                &Text::new("item3"),
                5,
                &Text::new("item3 with changes")
            )
            .into(),
            RemoveNode::new(Some(&"article"), 2).into(),
        ]
    );
}

#[test]
fn multiple_style_calls() {
    let old: Node<&'static str> = div(
        vec![
            styles_flag([
                ("font-family", "monospace", true),
                ("user-select", "none", false),
            ]),
            styles([("display", "flex")]),
        ],
        vec![],
    );
    let new: Node<&'static str> = div(
        vec![style("font-family", "monospace1"), style("display", "flex")],
        vec![],
    );
    let patches = diff(&old, &new);
    println!("patches: {:#?}", patches);
    assert_eq!(
        patches,
        vec![AddAttributes::new(
            &"div",
            0,
            0,
            vec![
                &style("font-family", "monospace1"),
                &style("display", "flex")
            ]
        )
        .into()]
    );
}
