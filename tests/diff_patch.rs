#![deny(warnings)]
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Node;
use sauron_vdom::diff;
use sauron_vdom::{Callback, Event, Patch, Text, Value};

use maplit::btreemap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn replace_node() {
    let old = div([], []);
    let new = span([], []);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(0, &span([], []))],
        "Replace the root if the tag changed"
    );

    let old = div([], [b([], [])]);
    let new = div([], [strong([], [])]);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(1, &strong([], []))],
        "Replace a child node"
    );

    let old = div([], [b([], [text("1")]), b([], [])]);
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

#[wasm_bindgen_test]
fn add_children() {
    let old = div([], [b([], [])]); //{ <div> <b></b> </div> },
    let new = div([], [b([], []), html_element("new", [], [])]); //{ <div> <b></b> <new></new> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AppendChildren(0, vec![&html_element("new", [], [])])],
        "Added a new node to the root node",
    )
}

#[wasm_bindgen_test]
fn remove_nodes() {
    let old = div([], [b([], []), span([], [])]); //{ <div> <b></b> <span></span> </div> },
    let new = div([], []); //{ <div> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::TruncateChildren(0, 0)],
        "Remove all child nodes at and after child sibling index 1",
    );

    let old = div(
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

    let old = div([], [b([], [i([], []), i([], [])]), b([], [])]); //{ <div> <b> <i></i> <i></i> </b> <b></b> </div> },
    let new = div([], [b([], [i([], [])]), i([], [])]); //{ <div> <b> <i></i> </b> <i></i> </div>},
    assert_eq!(
        diff(&old, &new),
        vec![Patch::TruncateChildren(1, 1), Patch::Replace(4, &i([], [])),],
        "Removing child and change next node after parent",
    )
}

#[wasm_bindgen_test]
fn add_attributes() {
    let hello: Value = "hello".into();
    let attributes = btreemap! {
    "id" => &hello,
    };

    let old = div([], []); //{ <div> </div> },
    let new = div([id("hello")], []); //{ <div id="hello"> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(0, attributes.clone())],
        "Add attributes",
    );

    let old = div([id("foobar")], []); //{ <div id="foobar"> </div> },
    let new = div([id("hello")], []); //{ <div id="hello"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(0, attributes)],
        "Change attribute",
    );
}

#[wasm_bindgen_test]
fn add_events() {
    let func = |_| {
        println!("hello");
    };
    let hello: Callback<Event> = func.into();
    let events = btreemap! {
    "click" => &hello,
    };

    let old = div([], []);
    let new = div([onclick(hello.clone())], []);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddEventListener(0, events.clone())],
        "Add event listener",
    );

    let hello2: Callback<Event> = func.into(); //recreated from the func closure, it will not be equal to the callback since the Rc points to a different address.
    let events2 = btreemap! {
    "click" => &hello2,
    };
    let old = div([onclick(hello.clone())], []);
    let new = div([onclick(hello2.clone())], []);

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddEventListener(0, events2.clone())],
        "Change event listener",
    );
}

#[wasm_bindgen_test]
fn remove_attributes() {
    let old = div([id("hey-there")], []); //{ <div id="hey-there"></div> },
    let new = div([], []); //{ <div> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::RemoveAttributes(0, vec!["id"])],
        "Remove attributes",
    );
}

#[wasm_bindgen_test]
fn remove_events() {
    let old = div([onclick(|_| println!("hi"))], []);
    let new = div([], []);
    assert_eq!(
        diff(&old, &new),
        vec![Patch::RemoveEventListener(0, vec!["click"])],
        "Remove events",
    );
}

#[wasm_bindgen_test]
fn change_attribute() {
    let changed: Value = "changed".into();
    let attributes = btreemap! {
    "id" => &changed,
    };

    let old = div([id("hey-there")], []); //{ <div id="hey-there"></div> },
    let new = div([id("changed")], []); //{ <div id="changed"> </div> },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::AddAttributes(0, attributes)],
        "Add attributes",
    );
}

#[wasm_bindgen_test]
fn replace_text_node() {
    let old: Node = text("Old"); //{ Old },
    let new: Node = text("New"); //{ New },

    assert_eq!(
        diff(&old, &new),
        vec![Patch::ChangeText(0, &Text::new("New"))],
        "Replace text node",
    );
}

// Initially motivated by having two elements where all that changed was an event listener
// because right now we don't patch event listeners. So.. until we have a solution
// for that we can just give them different keys to force a replace.
#[wasm_bindgen_test]
fn replace_if_different_keys() {
    let old = div([key(1)], []); //{ <div key="1"> </div> },
    let new = div([key(2)], []); //{ <div key="2"> </div> },
    assert_eq!(
        diff(&old, &new),
        vec![Patch::Replace(0, &div([key(2)], []))],
        "If two nodes have different keys always generate a full replace.",
    );
}
