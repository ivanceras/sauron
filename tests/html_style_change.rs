#![deny(warnings)]
use sauron::*;

#[test]
fn style_calcd_changed() {
    let old: Node<&'static str> = div(
        vec![style! {width: format!("calc(50% + {}", px(0))}],
        vec![],
    );

    let new: Node<&'static str> = div(
        vec![style! {width: format!("calc(50% + {}", px(200))}],
        vec![],
    );

    let patches: Vec<Patch<&'static str>> = diff(&old, &new).unwrap();
    let styl = style! {width: format!("calc(50% + {}", px(200))};
    let expected: Vec<Patch<&'static str>> =
        vec![Patch::add_attributes(&"div", TreePath::new([]), [&styl])];
    assert_eq!(expected, patches);
}

#[test]
fn style_calcd_changed_with_event() {
    let evt = on_click(|_| ());
    let old: Node<()> = div(
        vec![evt.clone(), style! {width: format!("calc(50% + {}", px(0))}],
        vec![],
    );

    let new: Node<()> = div(
        vec![
            evt.clone(),
            style! {width: format!("calc(50% + {}", px(200))},
        ],
        vec![],
    );

    let patches: Vec<Patch<()>> = diff(&old, &new).unwrap();
    let styl = style! {width: format!("calc(50% + {}", px(200))};
    let expected: Vec<Patch<()>> = vec![Patch::add_attributes(&"div", TreePath::new([]), [&styl])];
    assert_eq!(expected, patches);
}

#[test]
fn app_editor_width_allocation_bug() {
    let evt = on_click(|_| ());
    let old: Node<()> = main(
        [class("app")],
        [
            div(
                [
                    class("editor"),
                    evt.clone(),
                    style! {width: format!("calc(50% + {}", px(0))},
                ],
                [],
            ),
            div([class("separator")], []),
            div(
                [
                    class("svg_view"),
                    style! {width: format!("calc(50% - {}", px(0))},
                ],
                [],
            ),
        ],
    );

    let new_width = 200;

    let new: Node<()> = main(
        [class("app")],
        [
            div(
                [
                    class("editor"),
                    evt.clone(),
                    style! {width: format!("calc(50% + {}", px(new_width))},
                ],
                [],
            ),
            div([class("separator")], []),
            div(
                [
                    class("svg_view"),
                    style! {width: format!("calc(50% - {}", px(new_width))},
                ],
                [],
            ),
        ],
    );

    let patches: Vec<Patch<()>> = diff(&old, &new).unwrap();
    let styl_1 = style! {width: format!("calc(50% + {}", px(200))};
    let styl_2 = style! {width: format!("calc(50% - {}", px(200))};
    let expected: Vec<Patch<()>> = vec![
        Patch::add_attributes(&"div", TreePath::new([0]), [&styl_1]),
        Patch::add_attributes(&"div", TreePath::new([2]), [&styl_2]),
    ];
    assert_eq!(expected, patches);
}
