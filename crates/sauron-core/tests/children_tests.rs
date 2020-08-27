#![deny(warnings)]
use sauron_core::{
    svg::{attributes::*, tags::line, *},
    Node,
};

#[test]
fn children() {
    let lines: Vec<Node<()>> = (0..5)
        .map(|_| line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]))
        .collect();
    let html = svg(vec![], vec![circle(vec![], vec![])]).add_children(lines);
    let expect = svg(
        vec![],
        vec![
            circle(vec![], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
        ],
    );
    assert_eq!(html, expect, "Should be the same");
}

#[test]
fn children_using_macro_mix() {
    let lines: Vec<Node<()>> = (0..5)
        .map(|_| line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]))
        .collect();
    let html = svg(vec![], vec![circle(vec![], vec![])]).add_children(lines);
    let expect = svg(
        vec![],
        vec![
            circle(vec![], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
            line(vec![x1(100), x2(100), y1(100), y2(200)], vec![]),
        ],
    );
    assert_eq!(html, expect, "Should be the same");
}
