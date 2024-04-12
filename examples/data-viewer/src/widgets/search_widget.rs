use crate::assets;

use sauron::{
    html::{attributes::*, units::px, *},
    style, Attribute, Node,
};

pub struct SearchWidget {}

pub struct SearchIcon {}

impl SearchWidget {
    pub fn new<MSG>(width: i32, attributes: impl IntoIterator<Item = Attribute<MSG>>) -> Node<MSG> {
        let mut input_width = width - SearchIcon::css_width();
        if input_width < 0 {
            input_width = 0;
        }
        div(
            [class("search_widget flex-row")],
            [
                div(
                    [class("search_widget__search_icon")],
                    [assets::svg_search_icon(18, 18, "#888")],
                ),
                input(
                    [
                        r#type("text"),
                        class("search_widget__column_filter"),
                        style! {width: px(input_width)},
                    ],
                    [],
                )
                .with_attributes(attributes),
            ],
        )
    }
}

impl SearchIcon {
    fn search_icon_width() -> i32 {
        16
    }

    fn padding_left() -> i32 {
        3
    }

    fn padding_right() -> i32 {
        2
    }

    fn border_left() -> i32 {
        1
    }

    fn border_right() -> i32 {
        0
    }

    fn css_width() -> i32 {
        Self::search_icon_width()
            + Self::padding_left()
            + Self::padding_right()
            + Self::border_left()
            + Self::border_right()
    }
}
