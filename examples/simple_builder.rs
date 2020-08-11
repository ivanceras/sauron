#![deny(warnings)]
use sauron::{
    diff,
    html::{attributes::*, events::*, *},
    Node,
};

fn main() {
    let old: Node<()> = div(
        vec![
            class("some-class"),
            id("some-id"),
            on_click(|_| {
                println!("clicked");
            }),
            attr("data-id", 1),
            on("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        vec![input(vec![class("client"), r#type("checkbox")], vec![])],
    );
    let new = div(
        vec![
            class("some-class2"),
            id("some-id2"),
            on_click(|_| {
                println!("clicked2");
            }),
            attr("data-id", 2),
            on("mouseover", |_| {
                println!("i've been clicked2");
            }),
        ],
        vec![input(vec![class("client"), r#type("checkbox")], vec![])],
    );
    println!("{:#?}", old);
    println!("{:#?}", new);
    let patches = diff(&old, &new);
    println!("patches: {:#?}", patches);
}
