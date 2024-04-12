use sauron::{
    html::{attributes::*, units::px, *},
    style, Attribute, Node,
};
use search_widget::SearchWidget;

mod search_widget;

pub(crate) fn textbox<MSG, V: ToString>(
    v: V,
    attributes: impl IntoIterator<Item = Attribute<MSG>>,
) -> Node<MSG> {
    input([r#type("text"), class("textbox"), value(v.to_string())], []).with_attributes(attributes)
}

pub(crate) fn numberbox<MSG, V: ToString>(
    v: V,
    attributes: impl IntoIterator<Item = Attribute<MSG>>,
) -> Node<MSG> {
    input(
        [r#type("number"), class("numberbox"), value(v.to_string())],
        [],
    )
    .with_attributes(attributes)
}

pub(crate) fn text_link<MSG, V: ToString>(
    label: V,
    link: V,
    attributes: impl IntoIterator<Item = Attribute<MSG>>,
) -> Node<MSG> {
    a(
        [class("linkbox"), href(link.to_string())],
        [text(label.to_string())],
    )
    .with_attributes(attributes)
}

pub(crate) fn datebox<MSG>(
    v: String,
    attributes: impl IntoIterator<Item = Attribute<MSG>>,
) -> Node<MSG> {
    input([r#type("date"), class("datebox"), value(v)], []).with_attributes(attributes)
}

/// accepts the checked, container attributes and the actual checkbox attributes
pub(crate) fn checkbox<MSG>(
    checked: bool,
    container_attributes: impl IntoIterator<Item = Attribute<MSG>>,
    attributes: impl IntoIterator<Item = Attribute<MSG>>,
) -> Node<MSG> {
    div(
        [class("checkbox")],
        [input([r#type("checkbox")], [])
            .with_attributes(attrs_flag([("checked", "checked", checked)]))
            .with_attributes(attributes)],
    )
    .with_attributes(container_attributes)
}

pub(crate) fn selector_box<MSG>(
    checked: bool,
    container_attributes: impl IntoIterator<Item = Attribute<MSG>>,
    attributes: impl IntoIterator<Item = Attribute<MSG>>,
) -> Node<MSG> {
    div(
        [class("selector_box")],
        [input(
            [
                r#type("checkbox"),
                class("selector_box__checkbox"),
                style! {width: px(30)},
            ],
            [],
        )
        .with_attributes(attrs_flag([("checked", "checked", checked)]))
        .with_attributes(attributes)],
    )
    .with_attributes(container_attributes)
}

pub fn search_widget<MSG>(
    width: i32,
    attributes: impl IntoIterator<Item = Attribute<MSG>>,
) -> Node<MSG> {
    SearchWidget::new(width, attributes)
}
