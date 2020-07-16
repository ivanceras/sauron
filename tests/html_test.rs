#![deny(warnings)]
use sauron::{
    diff,
    div,
    html::{
        attributes::*,
        events::*,
        *,
    },
    input,
    mt_dom::AttValue,
    Element,
    Event,
    Node,
    Patch,
};

#[test]
fn test_macros() {
    let html: Node<()> = div!([class("class1"), class("class2")], []);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas() {
    let html: Node<()> = div!([class("class1"), class("class2"),], [],);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_attributes_only() {
    let html: Node<()> = div!([class("class1"), class("class2")], []);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_children_only() {
    let html: Node<()> =
        div!([class("class1"), class("class2")], [text("This is input"),]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_children_and_params() {
    let html: Node<()> =
        div!([class("class1"), class("class2")], [text("This is input"),],);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_attribute_and_children() {
    let html: Node<()> = div!(
        [class("class1"), class("class2"),],
        [text("This is input"),]
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_with_lines() {
    let html: Node<()> = div!(
        [class("class1"), class("class2")],
        [input!([], [text("This is an input")])]
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn will_not_merge_multiple_class_calls() {
    let html: Node<()> = div(vec![class("class1"), class("class2")], vec![]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 2);
    let elm = html.as_element_ref().expect("expecting an element");
    let classes = elm.get_attribute_values(&"class");
    assert_eq!(
        classes,
        vec![
            &AttValue::Plain(AttributeValue::from_value("class1".into())),
            &AttValue::Plain(AttributeValue::from_value("class2".into()))
        ]
    );
}

#[test]
fn should_merge_classes_flag() {
    let html: Node<()> = div(
        vec![classes_flag([("class1", true), ("class_flag", true)])],
        vec![],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
    let elm = html.as_element_ref().expect("expecting an element");
    let classes = elm.get_attribute_values(&"class");
    assert_eq!(
        classes,
        vec![&AttValue::Plain(AttributeValue::from_value(
            "class1 class_flag".to_string().into()
        ))]
    );
}

#[test]
fn simple_builder() {
    let mut div: Element<()> = Element::new(None, "div", vec![], vec![]);
    div.add_attributes(vec![attr("class", "some-class")]);
    let expected: Element<()> =
        Element::new(None, "div", vec![class("some-class")], vec![]);

    assert_eq!(div, expected);
}

#[test]
fn builder_with_event() {
    let cb = |x: Event| {
        println!("hello! {:?}", x);
    };
    let mut div: Element<()> = Element::new(None, "div", vec![], vec![]);
    div.add_attributes(vec![on("click", cb.clone())]);
    let expected: Element<()> =
        Element::new(None, "div", vec![on("click", cb)], vec![]);

    assert_eq!(
        div, expected,
        "Cloning a callback should only clone the reference"
    );
}

#[test]
fn builder_with_children() {
    let mut div: Element<()> = Element::new(None, "div", vec![], vec![]);
    div.add_attributes(vec![attr("class", "some-class")]);
    div.add_children(vec![Node::Text("Hello".to_string())]);
    let expected = Element::new(
        None,
        "div",
        vec![class("some-class")],
        vec![Node::Text("Hello".to_string())],
    );

    assert_eq!(div, expected);
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
        vec![Patch::TruncateChildren(&"div", 0, 0)],
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

    let new = div(vec![], vec![span(vec![], vec![b(vec![], vec![])])]);

    assert_eq!(
        diff(&old, &new),
        vec![
            Patch::TruncateChildren(&"div", 0, 1),
            Patch::TruncateChildren(&"span", 1, 1)
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
            Patch::TruncateChildren(&"b", 1, 1),
            Patch::Replace(&"b", 4, &i(vec![], vec![])),
        ],
        "Removing child and change next node after parent",
    )
}

#[test]
fn add_attributes() {
    let old: Node<()> = div(vec![], vec![]);
    let new = div(vec![id("hello")], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(&"div", 0, vec![&id("hello")])],
        "Add attributes",
    );

    let old: Node<()> = div(vec![id("foobar")], vec![]);
    let new = div(vec![id("hello")], vec![]);

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(&"div", 0, vec![&id("hello")])],
        "Change attribute",
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
    let old: Node<()> = div(vec![id("hey-there")], vec![]);
    let new = div(vec![id("changed")], vec![]);

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(&"div", 0, vec![&id("changed")])],
        "Add attributes",
    );
}

#[test]
fn replace_text_node() {
    let old: Node<()> = text("Old"); //{ Old },
    let new = text("New"); //{ New },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::ChangeText(0, "New")],
        "Replace text node",
    );
}

// Initially motivated by having two elements where all that changed was an event listener
// because right now we don't patch event listeners. So.. until we have a solution
// for that we can just give them different keys to force a replace.
#[test]
fn replace_if_different_keys() {
    let old: Node<()> = div(vec![key(1)], vec![]); //{ <div key="1"> </div> },
    let new = div(vec![key(2)], vec![]); //{ <div key="2"> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(&"div", 0, &div(vec![key(2)], vec![]))],
        "If two nodes have different keys always generate a full replace.",
    );
}
