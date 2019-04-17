#![deny(warnings)]
use sauron::svg::attributes::*;
use sauron::svg::*;
use sauron::Node;

#[test]
fn children() {
    let lines: Vec<Node<()>> = (0..5)
        .map(|_| line([x1(100), x2(100), y1(100), y2(200)], []))
        .collect();
    let html = svg([], [circle([], [])]).children(lines);
    let expect = svg(
        [],
        [
            circle([], []),
            line([x1(100), x2(100), y1(100), y2(200)], []),
            line([x1(100), x2(100), y1(100), y2(200)], []),
            line([x1(100), x2(100), y1(100), y2(200)], []),
            line([x1(100), x2(100), y1(100), y2(200)], []),
            line([x1(100), x2(100), y1(100), y2(200)], []),
        ],
    );
    assert_eq!(html, expect, "Should be the same");
}
