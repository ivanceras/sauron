use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sauron_core::{
    html::{attributes::*, *},
    Node,
};

fn bench_view_builing_100_child_nodes(c: &mut Criterion) {
    c.bench_function("100 nodes", |b| {
        b.iter(|| {
            let _view: Node<()> = black_box(div(
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
            ));
        })
    });
}

fn bench_view_builing_100_nodes_with_100_child_nodes(c: &mut Criterion) {
    c.bench_function("100 100", |b| {
        b.iter(|| {
            let _view: Node<()> = black_box(div(
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
            ));
        })
    });
}

criterion_group!(benches, bench_view_builing_100_nodes_with_100_child_nodes);
criterion_main!(benches);
