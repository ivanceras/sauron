use sauron::{
    html::{attributes::*, units::px, *},
    Attribute, Node,
};
use search_widget::SearchWidget;

mod search_widget;

pub(crate) fn textbox<MSG, V: ToString>(
    v: V,
    attributes: Vec<Attribute<MSG>>,
) -> Node<MSG> {
    input(
        vec![r#type("text"), class("textbox"), value(v.to_string())],
        vec![],
    )
    .add_attributes(attributes)
}

pub(crate) fn numberbox<MSG, V: ToString>(
    v: V,
    attributes: Vec<Attribute<MSG>>,
) -> Node<MSG> {
    input(
        vec![r#type("number"), class("numberbox"), value(v.to_string())],
        vec![],
    )
    .add_attributes(attributes)
}

pub(crate) fn text_link<MSG, V: ToString>(
    label: V,
    link: V,
    attributes: Vec<Attribute<MSG>>,
) -> Node<MSG> {
    a(
        vec![class("linkbox"), href(link.to_string())],
        vec![text(label.to_string())],
    )
    .add_attributes(attributes)
}

pub(crate) fn datebox<MSG>(
    v: String,
    attributes: Vec<Attribute<MSG>>,
) -> Node<MSG> {
    input(vec![r#type("date"), class("datebox"), value(v)], vec![])
        .add_attributes(attributes)
}

/// accepts the checked, container attributes and the actual checkbox attributes
pub(crate) fn checkbox<MSG>(
    checked: bool,
    container_attributes: Vec<Attribute<MSG>>,
    attributes: Vec<Attribute<MSG>>,
) -> Node<MSG> {
    div(
        vec![class("checkbox")],
        vec![input(vec![r#type("checkbox")], vec![])
            .add_attributes(attrs_flag([("checked", "checked", checked)]))
            .add_attributes(attributes)],
    )
    .add_attributes(container_attributes)
}

pub(crate) fn selector_box<MSG>(
    checked: bool,
    container_attributes: Vec<Attribute<MSG>>,
    attributes: Vec<Attribute<MSG>>,
) -> Node<MSG> {
    div(
        vec![class("selector_box")],
        vec![input(
            vec![
                r#type("checkbox"),
                class("selector_box__checkbox"),
                styles([("width", px(30))]),
            ],
            vec![],
        )
        .add_attributes(attrs_flag([("checked", "checked", checked)]))
        .add_attributes(attributes)],
    )
    .add_attributes(container_attributes)
}

pub fn search_widget<MSG>(
    width: i32,
    attributes: Vec<Attribute<MSG>>,
) -> Node<MSG>
where
    MSG: Clone,
{
    SearchWidget::new(width, attributes)
}
