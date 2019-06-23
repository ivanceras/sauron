#![feature(type_alias_enum_variants)]
use sauron::{
    diff,
    html::{
        attributes::*,
        events::*,
        *,
    },
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
use std::{
    collections::BTreeMap,
    iter::FromIterator,
};

use sauron::Event;

#[test]
fn simple_builder() {
    let mut div: Element<()> = div(vec![], vec![]);
    div.add_attributes(vec![attr("class", "some-class")]);
    let expected: Element<()> = Element {
        tag: "div",
        attrs: BTreeMap::from_iter(vec![("class", "some-class".into())]),
        events: BTreeMap::new(),
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
    let mut div: Element<()> = Element::new("div");
    div.add_event_listener("click", callback.clone());
    let expected: Element<()> = Element {
        tag: "div",
        events: BTreeMap::from_iter(vec![("click", callback.clone())]),
        attrs: BTreeMap::new(),
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
    let mut div: Element<()> = Element::new("div");
    div.add_attributes(vec![attr("class", "some-class")]);
    div.add_children(vec![Node::Text(Text {
        text: "Hello".to_string(),
    })]);
    let expected = Element {
        tag: "div",
        attrs: BTreeMap::from_iter(vec![("class", "some-class".into())]),
        children: vec![Node::Text(Text {
            text: "Hello".to_string(),
        })],
        events: BTreeMap::new(),
        namespace: None,
    };

    assert_eq!(div, expected);
}

#[test]
fn div_builder() {
    let clicked = |_| {
        println!("clicked");
    };
    let cb: Callback<Event, ()> = clicked.into();
    let div: Node<()> = div(
        [
            class("some-class"),
            r#type("submit"),
            on("click", cb.clone()),
        ],
        [div([class("some-class")], [text("Hello world!")])],
    );
    println!("{:#?}", div);
    let expected = Node::Element(Element {
        tag: "div",
        attrs: BTreeMap::from_iter(vec![
            ("class", "some-class".into()),
            ("type", "submit".into()),
        ]),
        events: BTreeMap::from_iter(vec![("click", cb.clone())]),
        namespace: None,
        children: vec![Node::Element(Element {
            tag: "div",
            attrs: BTreeMap::from_iter(vec![("class", "some-class".into())]),
            children: vec![Node::Text(Text {
                text: "Hello world!".into(),
            })],
            events: BTreeMap::new(),
            namespace: None,
        })],
    });
    assert_eq!(div, expected)
}

#[test]
fn replace_node() {
    let old: Node<()> = div([], []);
    let new = span([], []);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(0, &span([], []))],
        "Replace the root if the tag changed"
    );

    let old: Node<()> = div([], [b([], [])]);
    let new = div([], [strong([], [])]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(1, &strong([], []))],
        "Replace a child node"
    );

    let old: Node<()> = div([], [b([], [text("1")]), b([], [])]);
    let new = div([], [i([], [text("1")]), i([], [])]);
    assert_eq!(
        diff(&old, &new),
        vec![
            Patch::Replace(1, &i([], [text("1")])),
            Patch::Replace(3, &i([], [])),
        ],
        "Replace node with a child",
    )
}

#[test]
fn add_children() {
    let old: Node<()> = div([], [b([], [])]); //{ <div> <b></b> </div> },
    let new = div([], [b([], []), html_element("new", [], [])]); //{ <div> <b></b> <new></new> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AppendChildren(0, vec![&html_element("new", [], [])])],
        "Added a new node to the root node",
    )
}

#[test]
fn remove_nodes() {
    let old: Node<()> = div([], [b([], []), span([], [])]); //{ <div> <b></b> <span></span> </div> },
    let new = div([], []); //{ <div> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::TruncateChildren(0, 0)],
        "Remove all child nodes at and after child sibling index 1",
    );

    let old: Node<()> = div(
        [],
        [
            span(
                [],
                [
                    b([], []),
                    // This `i` tag will get removed
                    i([], []),
                ],
            ),
            // This `strong` tag will get removed
            strong([], []),
        ],
    );

    let new = div([], [span([], [b([], [])])]);

    assert_eq!(
        diff(&old, &new),
        vec![Patch::TruncateChildren(0, 1), Patch::TruncateChildren(1, 1)],
        "Remove a child and a grandchild node",
    );

    let old: Node<()> = div([], [b([], [i([], []), i([], [])]), b([], [])]); //{ <div> <b> <i></i> <i></i> </b> <b></b> </div> },
    let new = div([], [b([], [i([], [])]), i([], [])]); //{ <div> <b> <i></i> </b> <i></i> </div>},
    assert_eq!(
        diff(&old, &new),
        vec![Patch::TruncateChildren(1, 1), Patch::Replace(4, &i([], [])),],
        "Removing child and change next node after parent",
    )
}

#[test]
fn add_attributes() {
    let hello: Value = "hello".into();
    let attributes = BTreeMap::from_iter(vec![("id", &hello)]);

    let old: Node<()> = div([], []); //{ <div> </div> },
    let new = div([id("hello")], []); //{ <div id="hello"> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(0, attributes.clone())],
        "Add attributes",
    );

    let old: Node<()> = div([id("foobar")], []); //{ <div id="foobar"> </div> },
    let new = div([id("hello")], []); //{ <div id="hello"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(0, attributes)],
        "Change attribute",
    );
}

#[test]
fn new_different_event_will_replace_what_was_first_set() {
    let func = |_| {
        println!("hello");
    };
    let hello: Callback<Event, ()> = func.into();
    let events = BTreeMap::from_iter(vec![("click", &hello)]);

    let old: Node<()> = div([], []);
    let new: Node<()> = div([on("click", hello.clone())], []);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddEventListener(0, events.clone())],
        "Add event listener",
    );

    let hello2: Callback<Event, ()> = func.into(); //recreated from the func closure, it will not be equal to the callback since the Rc points to a different address.
    assert_ne!(hello, hello2, "Same function, different Rc::new()");
    let old = div([on("click", hello.clone())], []);
    let new = div([on("click", hello2.clone())], []);

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
    let old: Node<()> = div([id("hey-there")], []); //{ <div id="hey-there"></div> },
    let new = div([], []); //{ <div> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::RemoveAttributes(0, vec!["id"])],
        "Remove attributes",
    );
}

#[test]
fn remove_events() {
    let old: Node<()> = div([onclick(|_| println!("hi"))], []);
    let new = div([], []);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::RemoveEventListener(0, vec!["click"])],
        "Remove events",
    );
}

#[test]
fn change_attribute() {
    let changed: Value = "changed".into();
    let attributes = BTreeMap::from_iter(vec![("id", &changed)]);

    let old: Node<()> = div([id("hey-there")], []); //{ <div id="hey-there"></div> },
    let new = div([id("changed")], []); //{ <div id="changed"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(0, attributes)],
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
    let old: Node<()> = div([key(1)], []); //{ <div key="1"> </div> },
    let new = div([key(2)], []); //{ <div key="2"> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(0, &div([key(2)], []))],
        "If two nodes have different keys always generate a full replace.",
    );
}
