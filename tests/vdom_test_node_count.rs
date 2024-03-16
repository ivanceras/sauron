#![deny(warnings)]
use sauron::vdom::*;

#[test]
fn node_count1() {
    let old: Node<()> = element("div", vec![], vec![]);

    assert_eq!(1, old.node_count());
    assert_eq!(0, old.descendant_node_count());
}

#[test]
fn node_count3() {
    let old: Node<()> = element("div", vec![], vec![leaf("0"), leaf("1")]);

    // 4 instead of 3, because we automatically inserted a separator in between 2 sibling texts
    assert_eq!(3, old.node_count());
}

#[test]
fn node_count5() {
    let old: Node<()> = element(
        "div",
        vec![],
        vec![
            element(
                "b",
                vec![],
                vec![element("i", vec![], vec![]), element("i", vec![], vec![])],
            ),
            element("b", vec![], vec![]),
        ],
    );

    assert_eq!(5, old.node_count());
    assert_eq!(4, old.descendant_node_count());
}

#[test]
fn node_count6() {
    let old: Node<()> = element(
        "div",
        vec![],
        vec![
            element(
                "b",
                vec![],
                vec![
                    element("i", vec![], vec![]),
                    element("i", vec![], vec![leaf("hi")]),
                ],
            ),
            element("b", vec![], vec![]),
        ],
    );

    assert_eq!(6, old.node_count());
    assert_eq!(5, old.descendant_node_count());
}
