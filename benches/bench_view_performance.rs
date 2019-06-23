#![feature(test)]
extern crate test;
use test::{
    black_box,
    Bencher,
};

use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    Node,
};

#[bench]
fn bench_view_builing_100_child_nodes(b: &mut Bencher) {
    b.iter(|| {
        let view: Node<()> = black_box(div(
            [class("some-class")],
            (0..100)
                .into_iter()
                .map(|n| {
                    div([class("child-div")], vec![text(format!("node: {}", n))])
                })
                .collect::<Vec<Node<()>>>(),
        ));
    });
}

#[bench]
fn bench_view_builing_100_nodes_with_100_child_nodes(b: &mut Bencher) {
    b.iter(|| {
        let view: Node<()> = black_box(div(
            [class("some-class")],
            (0..100)
                .into_iter()
                .map(|n| {
                    div(
                        [class("parent"), class(n)],
                        (0..100)
                            .into_iter()
                            .map(|n2| {
                                div(
                                    [class("child-div")],
                                    vec![text(format!("node: {}", n2))],
                                )
                            })
                            .collect::<Vec<Node<()>>>(),
                    )
                })
                .collect::<Vec<Node<()>>>(),
        ));
    });
}
