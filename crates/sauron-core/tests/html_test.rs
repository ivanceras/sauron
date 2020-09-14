//#![deny(warnings)]
use sauron_core::mt_dom::patch::*;
use sauron_core::prelude::*;
use sauron_core::Patch;

#[test]
fn test_macros() {
    let html: Node<()> = div(vec![class("class1"), class("class2")], vec![]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas() {
    let html: Node<()> = div(vec![class("class1"), class("class2")], vec![]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_attributes_only() {
    let html: Node<()> = div(vec![class("class1"), class("class2")], vec![]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_children_only() {
    let html: Node<()> = div(
        vec![class("class1"), class("class2")],
        vec![text("This is input")],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_children_and_params() {
    let html: Node<()> = div(
        vec![class("class1"), class("class2")],
        vec![text("This is input")],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_trailing_commas_in_attribute_and_children() {
    let html: Node<()> = div(
        vec![class("class1"), class("class2")],
        vec![text("This is input")],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
}

#[test]
fn test_macros_with_lines() {
    let html: Node<()> = div(
        vec![class("class1"), class("class2")],
        vec![input(vec![], vec![text("This is an input")])],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
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
        vec![ReplaceNode::new(&"div", 0, &span(vec![], vec![])).into()],
        "ReplaceNode the root if the tag changed"
    );

    let old: Node<()> = div(vec![], vec![b(vec![], vec![])]);
    let new = div(vec![], vec![strong(vec![], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![ReplaceNode::new(&"b", 1, &strong(vec![], vec![])).into()],
        "ReplaceNode a child node"
    );

    let old: Node<()> =
        div(vec![], vec![b(vec![], vec![text("1")]), b(vec![], vec![])]);
    let new = div(vec![], vec![i(vec![], vec![text("1")]), i(vec![], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![
            ReplaceNode::new(&"b", 1, &i(vec![], vec![text("1")])).into(),
            ReplaceNode::new(&"b", 3, &i(vec![], vec![])).into(),
        ],
        "ReplaceNode node with a child",
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
        vec![AppendChildren::new(
            &"div",
            0,
            vec![&html_element("new", vec![], vec![])]
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
        vec![RemoveChildren::new(&"div", 0, vec![0, 1]).into()],
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
            RemoveChildren::new(&"span", 1, vec![1]).into(),
            RemoveChildren::new(&"div", 0, vec![1]).into(),
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
            RemoveChildren::new(&"b", 1, vec![1]).into(),
            ReplaceNode::new(&"b", 4, &i(vec![], vec![])).into(),
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
        vec![AddAttributes::new(&"div", 0, vec![&id("hello")]).into()],
        "Add attributes",
    );

    let old: Node<()> = div(vec![id("foobar")], vec![]);
    let new = div(vec![id("hello")], vec![]);

    assert_eq!(
        diff(&old, &new),
        vec![AddAttributes::new(&"div", 0, vec![&id("hello")]).into()],
        "Change attribute",
    );
}

#[test]
fn remove_attributes() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]);
    let new = div(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![RemoveAttributes::new(&"div", 0, vec![&id("hey-there")]).into()],
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
            vec![&on_click(|_| println!("hi"))]
        )
        .into()],
        "Remove events",
    );
}

#[test]
fn change_attribute() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]);
    let new = div(vec![id("changed")], vec![]);

    assert_eq!(
        diff(&old, &new),
        vec![AddAttributes::new(&"div", 0, vec![&id("changed")]).into()],
        "Add attributes",
    );
}

#[test]
fn replace_text_node() {
    let old: Node<()> = text("Old"); //{ Old },
    let new = text("New"); //{ New },

    assert_eq!(
        diff(&old, &new),
        vec![ChangeText::new(0, "Old", "New").into()],
        "ReplaceNode text node",
    );
}
