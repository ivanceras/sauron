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
    Element,
    Node,
    Patch,
};
use sauron_vdom::{
    builder::{
        attr,
        text,
    },
    Callback,
    Text,
    Value,
};

use sauron::Event;

#[test]
fn test_macros() {
    let html: Node<()> = div!([class("class1"), class("class2")], []);
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
}

#[test]
fn test_macros_trailing_commas() {
    let html: Node<()> = div!([class("class1"), class("class2"),], [],);
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
}

#[test]
fn test_macros_trailing_commas_in_attributes_only() {
    let html: Node<()> = div!([class("class1"), class("class2")], []);
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
}

#[test]
fn test_macros_trailing_commas_in_children_only() {
    let html: Node<()> =
        div!([class("class1"), class("class2")], [text("This is input"),]);
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
}

#[test]
fn test_macros_trailing_commas_in_children_and_params() {
    let html: Node<()> =
        div!([class("class1"), class("class2")], [text("This is input"),],);
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
}

#[test]
fn test_macros_trailing_commas_in_attribute_and_children() {
    let html: Node<()> = div!(
        [class("class1"), class("class2"),],
        [text("This is input"),]
    );
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
}

#[test]
fn test_macros_with_lines() {
    let html: Node<()> = div!(
        [class("class1"), class("class2")],
        [input!([], [text("This is an input")])]
    );
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
}

#[test]
fn should_merge_classes() {
    let html: Node<()> = div(vec![class("class1"), class("class2")], vec![]);
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
    let elm = html.as_element_ref().expect("expecting an element");
    let classes = elm.get_attr_value("class");
    assert_eq!(classes, Some(Value::from(["class1", "class2"])));
}

#[test]
fn should_merge_classes_flag() {
    let html: Node<()> = div(
        vec![class("class1"), classes_flag([("class_flag", true)])],
        vec![],
    );
    let attrs = html.get_attributes();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
    let elm = html.as_element_ref().expect("expecting an element");
    let classes = elm.get_attr_value("class");
    assert_eq!(
        classes,
        Some(Value::from(("class1", "class_flag".to_string())))
    );
}

#[test]
fn simple_builder() {
    let mut div: Element<()> = Element {
        tag: "div",
        attrs: vec![],
        children: vec![],
        namespace: None,
    };
    div.add_attributes(vec![attr("class", "some-class")]);
    let expected: Element<()> = Element {
        tag: "div",
        attrs: vec![class("some-class")],
        children: vec![],
        namespace: None,
    };

    assert_eq!(div, expected);
}

#[test]
fn builder_with_event() {
    let cb = |x: Event| {
        println!("hello! {:?}", x);
    };
    let callback: Callback<Event, ()> = cb.into();
    let mut div: Element<()> = Element::with_tag("div");
    div.add_event_listener("click", callback.clone());
    let expected: Element<()> = Element {
        tag: "div",
        attrs: vec![on("click", callback.clone())],
        children: vec![],
        namespace: None,
    };

    assert_eq!(
        div, expected,
        "Cloning a callback should only clone the reference"
    );
}

#[test]
fn builder_with_children() {
    let mut div: Element<()> = Element::with_tag("div");
    div.add_attributes(vec![attr("class", "some-class")]);
    div.add_children(vec![Node::Text(Text {
        text: "Hello".to_string(),
    })]);
    let expected = Element {
        tag: "div",
        attrs: vec![class("some-class")],
        children: vec![Node::Text(Text {
            text: "Hello".to_string(),
        })],
        namespace: None,
    };

    assert_eq!(div, expected);
}

#[test]
fn replace_node() {
    let old: Node<()> = div(vec![], vec![]);
    let new = span(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(0, &span(vec![], vec![]))],
        "Replace the root if the tag changed"
    );

    let old: Node<()> = div(vec![], vec![b(vec![], vec![])]);
    let new = div(vec![], vec![strong(vec![], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(1, &strong(vec![], vec![]))],
        "Replace a child node"
    );

    let old: Node<()> =
        div(vec![], vec![b(vec![], vec![text("1")]), b(vec![], vec![])]);
    let new = div(vec![], vec![i(vec![], vec![text("1")]), i(vec![], vec![])]);
    assert_eq!(
        diff(&old, &new),
        vec![
            Patch::Replace(1, &i(vec![], vec![text("1")])),
            Patch::Replace(3, &i(vec![], vec![])),
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
        vec![Patch::TruncateChildren(0, 0)],
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
        vec![Patch::TruncateChildren(0, 1), Patch::TruncateChildren(1, 1)],
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
            Patch::TruncateChildren(1, 1),
            Patch::Replace(4, &i(vec![], vec![])),
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
        vec![Patch::AddAttributes(0, vec![id("hello")])],
        "Add attributes",
    );

    let old: Node<()> = div(vec![id("foobar")], vec![]); //{ <div id="foobar"> </div> },
    let new = div(vec![id("hello")], vec![]); //{ <div id="hello"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(0, vec![id("hello")])],
        "Change attribute",
    );
}

#[test]
fn new_different_event_will_replace_what_was_first_set() {
    let func = |_| {
        println!("hello");
    };
    let hello: Callback<Event, ()> = func.into();

    let old: Node<()> = div(vec![], vec![]);
    let new: Node<()> = div(vec![on("click", hello.clone())], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddEventListener(
            0,
            vec![&on("click", hello.clone())]
        )],
        "Add event listener",
    );

    let hello2: Callback<Event, ()> = func.into(); //recreated from the func closure, it will not be equal to the callback since the Rc points to a different address.
    assert_ne!(hello, hello2, "Same function, different Rc::new()");
    let old = div(vec![on("click", hello.clone())], vec![]);
    let new = div(vec![on("click", hello2.clone())], vec![]);

    assert_eq!(
            diff(&old, &new),
            vec![],
            "Even though a new callback is recated from the same closure
            It will point to a different Rc, which are not equal.
            However, since comparing the wrapped Fn is just not possible
            The diffing algorithmn will just leave what was first set as the event listener
            ",
        );
}

#[test]
fn remove_attributes() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]); //{ <div id="hey-there"></div> },
    let new = div(vec![], vec![]); //{ <div> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::RemoveAttributes(0, vec!["id"])],
        "Remove attributes",
    );
}

#[test]
fn remove_events() {
    let old: Node<()> = div(vec![onclick(|_| println!("hi"))], vec![]);
    let new = div(vec![], vec![]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::RemoveEventListener(0, vec!["click"])],
        "Remove events",
    );
}

#[test]
fn change_attribute() {
    let old: Node<()> = div(vec![id("hey-there")], vec![]); //{ <div id="hey-there"></div> },
    let new = div(vec![id("changed")], vec![]); //{ <div id="changed"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(0, vec![id("changed")])],
        "Add attributes",
    );
}

#[test]
fn replace_text_node() {
    let old: Node<()> = text("Old"); //{ Old },
    let new = text("New"); //{ New },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::ChangeText(0, &Text::new("New"))],
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
        vec![Patch::Replace(0, &div(vec![key(2)], vec![]))],
        "If two nodes have different keys always generate a full replace.",
    );
}
