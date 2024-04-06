use crate::{
    views::{row_view, RowView},
    widgets::selector_box,
};
use restq::{ColumnDef, DataValue};
use sauron::{
    html::{
        attributes::{class, key},
        units::*,
        *,
    },
    style, Component, Effects, Node,
};

#[derive(Debug, PartialEq)]
pub enum Msg {
    RowMsg(usize, row_view::Msg),
}

pub struct PageView {
    pub data_columns: Vec<ColumnDef>,
    pub row_views: Vec<RowView>,
    /// Which columns of the rows are to be frozen on the left side of the table
    frozen_rows: Vec<usize>,
    frozen_columns: Vec<usize>,
    pub scroll_top: i32,
    #[allow(dead_code)]
    allocated_width: i32,
    /// the total number of rows count in the table
    #[allow(dead_code)]
    total_rows: usize,
    pub current_page: usize,
    is_visible: bool,
    pub page_height: i32,
}

impl PageView {
    pub fn new(data_columns: &[ColumnDef], data: &[Vec<DataValue>]) -> Self {
        let mut page_view = PageView {
            data_columns: data_columns.to_vec(),
            row_views: vec![],
            frozen_rows: vec![],
            frozen_columns: vec![],
            scroll_top: 0,
            allocated_width: 0,
            total_rows: 0,
            current_page: 1,
            is_visible: true,
            page_height: 0,
        };
        page_view.set_page(data, 1, 1);
        page_view
    }
}

impl Component for PageView {
    type MSG = Msg;
    type XMSG = ();

    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::RowMsg(row_index, row_msg) => {
                let effects = self.row_views[row_index].update(row_msg);
                effects.map_msg(move |follow_up| Msg::RowMsg(row_index, follow_up))
            }
        }
    }

    /// A grid of 2x2  containing 4 major parts of the table
    fn view(&self) -> Node<Msg> {
        if self.is_visible {
            ol(
                [
                    class("page_view"),
                    key(format!("page_view_{}", self.current_page)),
                ],
                //TODO: Views should also be automatically mapped
                self.row_views
                    .iter()
                    .enumerate()
                    .filter(|(_index, row_view)| !row_view.is_frozen)
                    .map(|(index, row_view)| {
                        row_view
                            .view()
                            .map_msg(move |row_msg| Msg::RowMsg(index, row_msg))
                    })
                    .collect::<Vec<Node<Msg>>>(),
            )
        } else {
            div(
                [
                    class("page_view__page_holder"),
                    style! {height: px(self.height())},
                ],
                [],
            )
        }
    }
}

impl PageView {
    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }

    pub fn get_row(&self, row_index: usize) -> Option<&RowView> {
        self.row_views.iter().find(|row| row.index == row_index)
    }

    pub fn row_count(&self) -> usize {
        self.row_views.len()
    }

    pub fn set_page(&mut self, data: &[Vec<DataValue>], current_page: usize, total_rows: usize) {
        trace!("setting pages in page_view: {:#?}", data);
        self.set_data_rows(data, current_page, total_rows);
    }

    /// replace all the data with a new data row
    /// TODO: also update the freeze_columns for each row_views
    pub fn set_data_rows(
        &mut self,
        data_row: &[Vec<DataValue>],
        current_page: usize,
        total_rows: usize,
    ) {
        self.row_views = data_row
            .iter()
            .enumerate()
            .map(|(index, row)| RowView::new(index, row, &self.data_columns))
            .collect();
        self.update_freeze_columns();
        self.total_rows = total_rows;
        self.current_page = current_page;
        self.page_height = self.height();
    }

    pub fn freeze_rows(&mut self, rows: &[usize]) {
        self.frozen_rows = rows.to_vec();
        self.update_frozen_rows();
    }

    pub fn set_column_widths(&mut self, column_widths: &[i32]) {
        self.row_views
            .iter_mut()
            .for_each(|row| row.set_column_widths(column_widths));
    }

    pub fn set_field_width(&mut self, field_index: usize, field_width: i32) {
        self.row_views
            .iter_mut()
            .for_each(|row| row.set_field_width(field_index, field_width))
    }

    /// call this is frozen rows selection are changed
    fn update_frozen_rows(&mut self) {
        let frozen_rows = &self.frozen_rows;
        self.row_views
            .iter_mut()
            .enumerate()
            .for_each(|(index, row_view)| {
                if frozen_rows.contains(&index) {
                    row_view.set_is_frozen(true)
                } else {
                    row_view.set_is_frozen(false)
                }
            })
    }

    /// Keep updating which columns are frozen
    /// call these when new rows are set or added
    pub fn update_freeze_columns(&mut self) {
        let frozen_columns = self.frozen_columns.clone();
        self.row_views
            .iter_mut()
            .for_each(|row_view| row_view.freeze_columns(frozen_columns.clone()))
    }

    pub fn freeze_columns(&mut self, columns: &[usize]) {
        self.frozen_columns = columns.to_vec();
        self.update_freeze_columns();
    }

    /// This is the allocated height set by the parent tab
    pub fn set_allocated_size(&mut self, (width, _height): (i32, i32)) {
        self.allocated_width = width;
    }

    /// calculate the height of the content
    /// it rows * row_height
    fn height(&self) -> i32 {
        trace!("row views: {}", self.row_views.len());
        self.row_views.len() as i32 * RowView::row_height()
    }

    /// These are values in a row that is under the frozen columns
    /// Can move up and down
    /// frozen column, but not frozen_rows
    pub fn view_frozen_columns(&self) -> Node<Msg> {
        // can move up and down
        ol(
            [class("page_view__frozen_columns")],
            self.row_views
                .iter()
                .enumerate()
                .filter(|(_index, row_view)| !row_view.is_frozen)
                .map(|(index, row_view)| {
                    // The checkbox selection and the rows of the frozen
                    // columns
                    div(
                        [class(
                            "page_view__frozen_columns__selector__frozen_column_rows flex-row",
                        )],
                        [
                            selector_box(false, [], [style! {width: px(30)}]),
                            row_view
                                .view_frozen_columns()
                                .map_msg(move |row_msg| Msg::RowMsg(index, row_msg)),
                        ],
                    )
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// frozen_row and frozen_columns
    pub fn view_immovable_rows(&self) -> Node<Msg> {
        ol(
            [class("page_view__immovable_rows")],
            self.row_views
                .iter()
                .enumerate()
                .filter(|(_index, row_view)| row_view.is_frozen)
                .map(|(index, row_view)| {
                    div(
                        [class("page_view__selector_box__immovable_rows flex-row")],
                        [
                            selector_box(
                                false,
                                [class("immovable_rows__selector_box")],
                                [style! {width: px(30)}],
                            ),
                            row_view
                                .view_immovable_fields()
                                .map_msg(move |row_msg| Msg::RowMsg(index, row_msg)),
                        ],
                    )
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// frozen rows but not frozen columns
    pub fn view_frozen_rows(&self) -> Node<Msg> {
        // can move left and right, but not up and down
        div(
            [class("page_view__frozen_page")],
            self.row_views
                .iter()
                .enumerate()
                .filter(|(_index, row_view)| row_view.is_frozen)
                .map(|(index, row_view)| {
                    row_view
                        .view_frozen()
                        .map_msg(move |row_msg| Msg::RowMsg(index, row_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }
}
