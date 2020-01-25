#[macro_use]
extern crate criterion;

use criterion::{
    Criterion,
    Fun,
};

use sauron::{
    html::{
        attributes::*,
        *,
    },
    Node,
};

fn build_100_child_nodes() {
    let _view: Node<()> = div(
        vec![class("some-class")],
        (0..100)
            .into_iter()
            .map(|n| {
                div(
                    vec![class("child-div")],
                    vec![text(format!("node: {}", n))],
                )
            })
            .collect::<Vec<Node<()>>>(),
    );
}

fn build_100_nodes_with_100_child_nodes() {
    let _view: Node<()> = div(
        vec![class("some-class")],
        (0..100)
            .into_iter()
            .map(|n| {
                div(
                    vec![class("parent"), class(n)],
                    (0..100)
                        .into_iter()
                        .map(|n2| {
                            div(
                                vec![class("child-div")],
                                vec![text(format!("node: {}", n2))],
                            )
                        })
                        .collect::<Vec<Node<()>>>(),
                )
            })
            .collect::<Vec<Node<()>>>(),
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_functions(
        "nodes",
        vec![
            Fun::new("100x100", |b, _i| {
                b.iter(|| build_100_nodes_with_100_child_nodes())
            }),
            Fun::new("100", |b, _i| b.iter(|| build_100_child_nodes())),
        ],
        0,
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
