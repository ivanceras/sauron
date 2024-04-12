use crate::{assets, widgets::search_widget, ColumnDef};
use sauron::{
    html::{attributes::*, events::*, units::*, *},
    style, Component, Effects, Node,
};

#[derive(Debug, PartialEq)]
pub enum Msg {
    ChangeSearch(String),
}

pub struct ColumnView {
    pub column: ColumnDef,
    pub width: i32,
    pub height: i32,
    pub is_frozen: bool,
}

impl ColumnView {
    pub fn new(column: ColumnDef) -> Self {
        ColumnView {
            column,
            width: 220,
            height: 70,
            is_frozen: false,
        }
    }
}

impl Component for ColumnView {
    type MSG = Msg;
    type XMSG = ();

    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::ChangeSearch(search) => {
                trace!("Search term change: {}", search);
                Effects::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        self.column_view_controls()
    }
}

impl ColumnView {
    /// calculated width for css style, this includes the padding,
    /// margins and borders
    pub fn css_width(&self) -> i32 {
        self.width + Self::padding() + Self::grip_width() + Self::border()
    }

    /// size width used in calculating other widths
    pub fn calc_width(&self) -> i32 {
        self.width + Self::grip_width()
    }

    /// left and right padding for the input fields
    fn padding() -> i32 {
        Self::side_padding_width() * 2
    }

    /// The padding at the left and right are equal
    fn side_padding_width() -> i32 {
        5
    }

    /// 5px on the left, 5px on the right
    pub fn grip_width() -> i32 {
        10
    }

    /// 1px border at the right side
    fn border() -> i32 {
        1
    }

    fn column_view_controls(&self) -> Node<Msg> {
        let mut controls_width = self.calc_width();
        if controls_width < 0 {
            controls_width = 0;
        }
        let self_width = self.width;
        div(
            [
                class("column_view__controls flex-column"),
                classes_flag([("column_view__controls--frozen", self.is_frozen)]),
                style! {
                    height: px(self.height),
                    width: px(controls_width),
                },
            ],
            [
                div(
                    [
                        class("column_controls flex-row"),
                        style! {
                            width: px(self_width),
                            padding_left: px(Self::side_padding_width()),
                            padding_right: px(Self::side_padding_width()),
                        },
                    ],
                    [
                        div(
                            [class("column_controls__column_name")],
                            [text(&self.column.column.name)],
                        ),
                        div(
                            [class("column_controls__sort_btn")],
                            [assets::sort_btn_asc(18, 18, "#888")],
                        ),
                    ],
                ),
                div(
                    [class("column_view__search")],
                    [search_widget(
                        self.width,
                        [on_input(|input| Msg::ChangeSearch(input.value()))],
                    )],
                ),
            ],
        )
    }
}
