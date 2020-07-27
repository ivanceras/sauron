#![deny(warnings)]
use sauron::{
    diff,
    html::{attributes::*, events::*, *},
    Attribute, Node, Patch,
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
        vec![Patch::RemoveAttributes(
            &"input",
            0,
            vec![&on("input", |_| { () })],
        )]
    );
}

#[test]
fn change_class_attribute() {
    let old: Node<()> = div(vec![classes(["class1", "class2"])], vec![]);

    let new = div(vec![classes(["class1", "difference_class"])], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(
            &"div",
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
        )],
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
        vec![Patch::RemoveChildren(&"div", 0, vec![3, 4, 5, 6])],
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
    assert_eq!(
        diff(&old, &new),
        vec![
            Patch::AddAttributes(&"div", 1, vec![&class("class5")]),
            Patch::AddAttributes(&"div", 2, vec![&class("class6")]),
            Patch::AddAttributes(&"div", 3, vec![&class("class7")]),
            Patch::RemoveChildren(&"div", 0, vec![3, 4, 5, 6]),
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
        vec![Patch::Replace(&"div", 0, &span(vec![], vec![]))],
        "Replace the root if the tag changed"
    );

    let old: Node<()> = div(vec![], vec![b(vec![], vec![])]);
    let new = div(vec![], vec![strong(vec![], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(&"b", 1, &strong(vec![], vec![]))],
        "Replace a child node"
    );
}

#[test]
fn replace_node2() {
    let old: Node<()> =
        div(vec![], vec![b(vec![], vec![text("1")]), b(vec![], vec![])]);
    let new = div(vec![], vec![i(vec![], vec![text("1")]), i(vec![], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![
            Patch::Replace(&"b", 1, &i(vec![], vec![text("1")])),
            Patch::Replace(&"b", 3, &i(vec![], vec![])),
        ],
        "Replace node with a child",
    )
}

#[test]
fn add_children() {
    let old: Node<()> = div(vec![], vec![b(vec![], vec![])]); //{ <div> <b></b> </div> },
    let new = div(
        vec![],
        vec![b(vec![], vec![]), html_element("new", vec![], vec![])],
    ); //{ <div> <b></b> <new></new> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AppendChildren(
            &"div",
            0,
            vec![&html_element("new", vec![], vec![])]
        )],
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
        vec![Patch::RemoveChildren(&"div", 0, vec![0, 1])],
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
            Patch::RemoveChildren(&"span", 1, vec![1]),
            Patch::RemoveChildren(&"div", 0, vec![1]),
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
            Patch::RemoveChildren(&"b", 1, vec![1]),
            Patch::Replace(&"b", 4, &i(vec![], vec![])),
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
        vec![Patch::AddAttributes(&"div", 0, vec![&id("hello")])],
        "Add attributes",
    );

    let old: Node<()> = div(vec![id("foobar")], vec![]); //{ <div id="foobar"> </div> },
    let new = div(vec![id("hello")], vec![]); //{ <div id="hello"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(&"div", 0, vec![&id("hello")])],
        "Change attribute",
    );
}

#[test]
fn add_style_attributes() {
    let old: Node<()> = div(vec![style("display", "block")], vec![]);
    let new = div(vec![style("display", "none")], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(
            &"div",
            0,
            vec![&style("display", "none")]
        )],
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
        vec![Patch::AddAttributes(
            &"div",
            0,
            vec![&Attribute::with_multiple_values(
                None,
                "style",
                vec![AttributeValue::Style(vec![
                    Style::new("display", "none".into()),
                    Style::new("position", "absolute".into())
                ])]
            )]
        )],
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
        vec![Patch::RemoveAttributes(
            &"div",
            0,
            vec![&style("display", "block")]
        )],
        "Add attributes",
    );
}

#[test]
fn remove_attributes() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]);
    let new = div(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::RemoveAttributes(&"div", 0, vec![&id("hey-there")])],
        "Remove attributes",
    );
}

#[test]
fn remove_events() {
    let old: Node<()> = div(vec![on_click(|_| println!("hi"))], vec![]);
    let new = div(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::RemoveAttributes(
            &"div",
            0,
            vec![&on_click(|_| println!("hi"))]
        )],
        "Remove events",
    );
}

#[test]
fn change_attribute() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]); //{ <div id="hey-there"></div> },
    let new = div(vec![id("changed")], vec![]); //{ <div id="changed"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(&"div", 0, vec![&id("changed")])],
        "Add attributes",
    );
}

#[test]
fn replace_text_node() {
    let old: Node<()> = text("Old"); //{ Old },
    let new: Node<()> = text("New"); //{ New },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::ChangeText(0, "New")],
        "Replace text node",
    );
}

// the element with keys could have event listeners (which are not comparable)
// To be safe, just remove non-matching old keyed element and re-create an element for the non-matching keyed
// new element
#[test]
fn different_key_will_be_removed_and_create_a_new_one() {
    let old: Node<()> = main(vec![], vec![div(vec![key(1)], vec![])]);
    let new = main(vec![], vec![div(vec![key(2)], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![
            Patch::RemoveChildren(&"main", 0, vec![0]),
            Patch::AppendChildren(&"main", 0, vec![&div(vec![key(2)], vec![])]),
        ],
        "If two nodes have different keys always generate a full replace.",
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
            Patch::ChangeText(7, "item3 with changes"),
            Patch::RemoveChildren(&"section", 1, vec![0])
        ]
    );
}
