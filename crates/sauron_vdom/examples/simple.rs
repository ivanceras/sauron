#![deny(warnings)]
use sauron_vdom::{
    builder::*,
    diff,
    Node,
};

fn main() {
    let old: Node<&'static str, &'static str, (), ()> = element(
        "div",
        vec![
            attr("class", "some-class"),
            attr("id", "some-id"),
            on("click", |_| {
                println!("clicked");
            }),
            attr("data-id", 1111),
            on("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        vec![element("div", vec![], vec![text("Hello world!")])],
    );

    let new = element(
        "div",
        vec![
            attr("class", "some-class2"),
            attr("id", "some-id2"),
            on("click", |_| {
                println!("clicked2");
            }),
            attr("data-id", 2222),
            on("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        vec![element("div", vec![], vec![text("Wazzup!")])],
    );

    println!("old: {}", old);
    println!("new: {}", new);
    let patches = diff(&old, &new);
    println!("patches: {:#?}", patches);
}
