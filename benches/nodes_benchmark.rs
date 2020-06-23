#[macro_use]
extern crate criterion;

use criterion::{
    Criterion,
    Fun,
};

use sauron::{
    diff,
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

fn diff_100() {
    let view1: Node<()> = div(
        vec![class("some-class")],
        (0..100)
            .into_iter()
            .map(|n| {
                div(
                    vec![class(format!("child-div_{}", n))],
                    vec![text(format!("node: {}", n))],
                )
            })
            .collect::<Vec<Node<()>>>(),
    );

    let view2: Node<()> = div(
        vec![class("some-class")],
        (0..100)
            .into_iter()
            .map(|n| {
                div(
                    vec![class(format!("child-div_{}", n + 1))],
                    vec![text(format!("node: {}", n))],
                )
            })
            .collect::<Vec<Node<()>>>(),
    );
    let node_diff = diff(&view1, &view2);
    assert_eq!(node_diff.len(), 100)
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
            Fun::new("diff_100", |b, _i| b.iter(|| diff_100())),
        ],
        0,
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
