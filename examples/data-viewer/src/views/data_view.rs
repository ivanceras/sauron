use crate::{
    views::{column_view, page_view, ColumnView, FieldView, PageView, RowView},
    widgets::selector_box,
    ColumnDef, DataValue, Error,
};
use restq::{bytes_to_chars, table_def, CsvRows, TableName};
use sauron::{
    html::{
        attributes::{class, key},
        events::*,
        units::*,
        *,
    },
    style, Component, Effects, Node,
};
use std::{
    cell::RefCell,
    io::{BufRead, BufReader, Cursor},
    rc::Rc,
};

#[derive(Debug, PartialEq)]
pub enum Msg {
    ColumnMsg(usize, column_view::Msg),
    PageMsg(usize, page_view::Msg),
    Scrolled((i32, i32)),
    MouseMove(i32, i32),
    ColumnEndResize(i32, i32),
    ColumnStartResize(usize, Grip, i32, i32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Grip {
    Right,
    Left,
}

pub struct DataView {
    pub table_name: TableName,
    pub data_columns: Vec<ColumnDef>,
    pub column_views: Vec<ColumnView>,
    pub page_views: Vec<PageView>,
    /// Which columns of the rows are to be frozen on the left side of the table
    frozen_rows: Vec<(usize, Vec<usize>)>,
    frozen_columns: Vec<usize>,
    pub scroll_top: i32,
    scroll_left: i32,
    #[allow(dead_code)]
    pub allocated_width: i32,
    pub allocated_height: i32,
    /// the total number of rows count in the table
    #[allow(dead_code)]
    total_rows: usize,
    #[allow(dead_code)]
    current_page: usize,
    visible_page: usize,
    active_resize: Option<(usize, Grip)>,
    start_x: i32,
}

impl Component for DataView {
    type MSG = Msg;
    type XMSG = ();

    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::PageMsg(page_index, page_msg) => {
                let effects = self.page_views[page_index].update(page_msg);
                effects.map_msg(move |follow_up| Msg::PageMsg(page_index, follow_up))
            }
            Msg::ColumnMsg(column_index, column_msg) => {
                let effects = self.column_views[column_index].update(column_msg);
                effects.map_msg(move |follow_up| Msg::ColumnMsg(column_index, follow_up))
            }
            Msg::Scrolled((scroll_top, scroll_left)) => {
                self.scroll_top = scroll_top;
                self.scroll_left = scroll_left;
                let visible_page = self.visible_page();
                if self.visible_page != visible_page {
                    self.visible_page = visible_page;
                    self.update_visible_pages();
                }
                Effects::none()
            }
            Msg::ColumnEndResize(_client_x, _client_y) => {
                self.active_resize = None;
                Effects::none()
            }
            Msg::MouseMove(client_x, _client_y) => {
                if let Some((column_index, active_resize)) = self.active_resize.clone() {
                    match active_resize {
                        Grip::Left => {}
                        Grip::Right => {
                            let delta_x = client_x - self.start_x;
                            let column_view = &mut self.column_views[column_index];
                            column_view.width += delta_x;
                            let column_width = column_view.calc_width();
                            self.start_x = client_x;
                            self.set_field_width(column_index, column_width);
                        }
                    }
                }
                Effects::none()
            }
            Msg::ColumnStartResize(column_index, grip, client_x, _client_y) => {
                self.active_resize = Some((column_index, grip));
                let column_view = &mut self.column_views[column_index];
                self.start_x = client_x;
                debug!("width of column {} is {}", column_index, column_view.width);
                Effects::none()
            }
        }
    }

    /// A grid of 2x2  containing 4 major parts of the table
    fn view(&self) -> Node<Msg> {
        main(
            [
                class("data_view grid"),
                style! {
                    width: px(self.allocated_width - 40),
                    min_width: px(self.calculate_min_width()),
                },
                // to ensure no reusing of table view when replaced with
                // another table
                key(format!("data_view_{}", self.table_name.name)),
            ],
            [
                // TOP-LEFT: Content 1
                section(
                    [class(
                        "data_view__spacer__frozen_column_names__immovable_frozen_columns",
                    )],
                    [
                        div(
                            [class("data_view__spacer__frozen_column_names flex-row")],
                            [
                                div(
                                    [class("data_view__spacer flex-column-reverse")],
                                    [div(
                                        [class("data_view__spacer__multi_selector")],
                                        [selector_box(false, [], [])],
                                    )],
                                ),
                                self.view_frozen_column_names(),
                            ],
                        ),
                        // totally immovable rows/columns
                        self.view_immovable_rows(),
                    ],
                ),
                // TOP-RIGHT: Content 2
                section(
                    [
                        class("data_view__normal_column_names__frozen_rows"),
                        style! {
                            width: px(self.calculate_normal_rows_width()),
                            overflow_x: "hidden",
                        },
                    ],
                    [section(
                        [
                            class("normal_column_names__frozen_rows"),
                            style! {
                                margin_left: px(-self.scroll_left)
                            },
                        ],
                        [
                            // can move left and right
                            self.view_normal_column_names(),
                            self.view_frozen_rows(),
                        ],
                    )],
                ),
                // BOTTOM-LEFT: Content 3
                // needed to overflow hide the frozen columns when scrolled up and down
                section(
                    [
                        class("data_view__frozen_columns_container"),
                        style! {
                            height: px(self.calculate_normal_rows_height()),
                            overflow_y: "hidden",
                        },
                    ],
                    [self.view_frozen_columns()],
                ),
                // BOTTOM-RIGHT: Content 4
                self.view_normal_rows(),
            ],
        )
    }
}

impl DataView {
    /// Note: if the data is deformed, only the correctly formed ones will be parsed and shown
    pub fn from_csv_data(csv: Vec<u8>) -> Result<Self, Error> {
        let mut bufread = BufReader::new(Cursor::new(csv));
        let mut first_line = vec![];
        let _header_len = bufread
            .read_until(b'\n', &mut first_line)
            .map_err(Error::HeaderIoError)?;

        let header_input = bytes_to_chars(&first_line);
        let table_def = table_def()
            .parse(&header_input)
            .map_err(Error::HeaderParseError)?;

        let column_defs = table_def.columns.clone();
        trace!("bufread len: {}", bufread.buffer().len());
        let rows_iter = CsvRows::new(bufread);
        let data: Vec<Vec<restq::DataValue>> = rows_iter.into_data_values(&column_defs);
        trace!("rows: {}", data.len());
        let page_view = PageView::new(&column_defs, &data);
        let data_view = DataView {
            table_name: table_def.table.clone(),
            data_columns: table_def.columns.clone(),
            column_views: table_def
                .columns
                .iter()
                .map(|column| ColumnView::new(column.clone()))
                .collect(),
            page_views: vec![page_view],
            frozen_rows: vec![],
            frozen_columns: vec![],
            scroll_top: 0,
            scroll_left: 0,
            allocated_width: 0,
            allocated_height: 0,
            total_rows: 0,
            current_page: 0,
            visible_page: 0,
            active_resize: None,
            start_x: 0,
        };
        Ok(data_view)
    }

    pub fn set_pages(
        &mut self,
        pages: &[Vec<Vec<DataValue>>],
        current_page: usize,
        total_records: usize,
    ) {
        self.page_views = pages
            .iter()
            .map(|page| PageView::new(&self.data_columns, page))
            .collect();
        self.total_rows = total_records;
        self.current_page = current_page;
        self.update_visible_pages();
    }

    pub fn get_fields(&self, page_index: usize, row_index: usize) -> &Vec<Rc<RefCell<FieldView>>> {
        &self.get_row(page_index, row_index).fields
    }

    fn get_row(&self, page_index: usize, row_index: usize) -> &RowView {
        self.page_views[page_index]
            .get_row(row_index)
            .expect("expecting a row")
    }

    pub fn freeze_rows(&mut self, rows: Vec<(usize, Vec<usize>)>) {
        self.frozen_rows = rows.clone();
        self.update_frozen_rows();
    }

    /// call this is frozen rows selection are changed
    fn update_frozen_rows(&mut self) {
        for (page_index, rows) in &self.frozen_rows {
            self.page_views[*page_index].freeze_rows(rows);
        }
    }

    fn frozen_row_height(&self) -> i32 {
        self.frozen_rows.len() as i32 * RowView::row_height() //use the actual row height
    }

    fn frozen_column_width(&self) -> i32 {
        self.column_views.iter().fold(0, |acc, column_view| {
            if column_view.is_frozen {
                acc + column_view.css_width() // there is a  10px padding per input, and 10px grips
            } else {
                acc
            }
        })
    }

    fn selector_width(&self) -> i32 {
        30
    }

    /// Calculate the min width of this table view
    /// based on the frozen_column_width and the selector size
    fn calculate_min_width(&self) -> i32 {
        self.frozen_column_width() + self.selector_width()
    }

    /// Keep updating which columns are frozen
    /// call these when new rows are set or added
    pub fn update_freeze_columns(&mut self) {
        for fc in self.frozen_columns.iter() {
            if let Some(fc) = self.column_views.get_mut(*fc) {
                fc.is_frozen = true;
            }
        }

        let frozen_columns = self.frozen_columns.clone();
        self.page_views
            .iter_mut()
            .for_each(|page_view| page_view.freeze_columns(&frozen_columns))
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
        self.frozen_columns = columns;
        self.update_freeze_columns();
    }

    pub fn set_column_widths(&mut self, column_widths: &[i32]) {
        self.column_views
            .iter_mut()
            .zip(column_widths.iter())
            .for_each(|(column_view, cwidth)| {
                column_view.width = *cwidth;
            });

        // calculate field_widths by adding the the grip width of each column_view
        let field_widths: Vec<i32> = self.column_views.iter().map(|cw| cw.calc_width()).collect();

        self.page_views
            .iter_mut()
            .for_each(|page_view| page_view.set_column_widths(&field_widths));
    }

    /// set the field widths due to a change in column width
    fn set_field_width(&mut self, column_index: usize, column_width: i32) {
        self.page_views
            .iter_mut()
            .for_each(|page_view| page_view.set_field_width(column_index, column_width));
    }

    /// This is the allocated height set by the parent tab
    pub fn set_allocated_size(&mut self, width: i32, height: i32) {
        self.allocated_width = width;
        self.allocated_height = height;
    }

    /// TODO: include the height of the frozen rows
    pub fn calculate_normal_rows_size(&self) -> (i32, i32) {
        let height = self.allocated_height
            - self.frozen_row_height()
            - self.calculate_needed_height_for_auxilliary_spaces();
        let width = self.allocated_width
            - self.frozen_column_width()
            - self.calculate_needed_width_for_auxilliary_spaces();
        let clamped_height = if height < 0 { 0 } else { height };
        let clamped_width = if width < 0 { 0 } else { width };
        (clamped_width, clamped_height)
    }

    fn calculate_normal_rows_height(&self) -> i32 {
        self.calculate_normal_rows_size().1
    }

    fn calculate_normal_rows_width(&self) -> i32 {
        self.calculate_normal_rows_size().0
    }

    /// height from the columns names, padding, margins and borders
    pub fn calculate_needed_height_for_auxilliary_spaces(&self) -> i32 {
        120
    }

    pub fn calculate_needed_width_for_auxilliary_spaces(&self) -> i32 {
        85
    }

    /// calculate the height of the content
    /// it rows * row_height
    fn calculate_content_height(&self) -> i32 {
        self.page_views.iter().fold(0, |mut acc, page| {
            acc += page.page_height;
            acc
        })
    }

    fn visible_page(&self) -> usize {
        let mut acc = 0;
        for (i, page_view) in self.page_views.iter().enumerate() {
            acc += page_view.page_height;
            if acc >= self.scroll_top {
                return i + 1;
            }
        }
        0
    }

    /// calculate the distance of the scrollbar
    /// before hitting bottom
    fn scrollbar_to_bottom(&self) -> i32 {
        let content_height = self.calculate_content_height(); // scroll height
        let row_container_height = self.calculate_normal_rows_height(); // client height
        content_height - (self.scroll_top + row_container_height)
    }

    fn is_scrolled_near_bottom(&self) -> bool {
        let scroll_bottom_allowance = 100;
        self.scrollbar_to_bottom() <= scroll_bottom_allowance
    }

    /// These are values in a row that is under the frozen columns
    /// Can move up and down
    fn view_frozen_columns(&self) -> Node<Msg> {
        // can move up and down
        ol(
            [
                class("data_view__frozen_columns"),
                style! {
                    margin_top: px(-self.scroll_top),
                },
            ],
            self.page_views
                .iter()
                .enumerate()
                .map(|(index, page_view)| {
                    page_view
                        .view_frozen_columns()
                        .map_msg(move |page_msg| Msg::PageMsg(index, page_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// These are the columns of the frozen columns.
    /// Since frozen, they can not move in any direction
    fn view_frozen_column_names(&self) -> Node<Msg> {
        // absolutely immovable frozen column and row
        // can not move in any direction
        header(
            [class("data_view__frozen_column_names flex-row")],
            self.column_views
                .iter()
                .enumerate()
                .filter(|(_index, column)| column.is_frozen)
                .map(|(index, column)| self.column_view_with_resize(index, column))
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// The column names of the normal columns
    /// can move left and right and always follows the alignment of the column of the normal rows
    fn view_normal_column_names(&self) -> Node<Msg> {
        header(
            [class("data_view__normal_column_names flex-row")],
            self.column_views
                .iter()
                .enumerate()
                .filter(|(_index, column)| !column.is_frozen)
                .map(|(index, column)| self.column_view_with_resize(index, column))
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    fn column_view_with_resize(&self, index: usize, column: &ColumnView) -> Node<Msg> {
        div(
            [class("column_view flex-row")],
            [
                column
                    .view()
                    .map_msg(move |column_msg| Msg::ColumnMsg(index, column_msg)),
                div(
                    [
                        class("column_view__grip column_view__grip--right"),
                        style! {
                            width: px(ColumnView::grip_width()),
                        },
                        on_mousedown(move |event| {
                            Msg::ColumnStartResize(
                                index,
                                Grip::Right,
                                event.client_x(),
                                event.client_y(),
                            )
                        }),
                    ],
                    [],
                ),
            ],
        )
    }

    /// The rows are both frozen row and frozen column
    /// Therefore can not move in any direction
    /// These are records that has its rows and columns both frozen
    fn view_immovable_rows(&self) -> Node<Msg> {
        div(
            [class("data_view__immovable_frozen_columns")],
            self.page_views
                .iter()
                .enumerate()
                .map(|(index, page_view)| {
                    page_view
                        .view_immovable_rows()
                        .map_msg(move |page_msg| Msg::PageMsg(index, page_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// These are the pinned columns
    fn view_frozen_rows(&self) -> Node<Msg> {
        // can move left and right, but not up and down
        div(
            [class("data_view__frozen_rows flex-column")],
            self.page_views
                .iter()
                .enumerate()
                .map(|(index, page_view)| {
                    page_view
                        .view_frozen_rows()
                        .map_msg(move |page_msg| Msg::PageMsg(index, page_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// The rest of the columns and move in any direction
    fn view_normal_rows(&self) -> Node<Msg> {
        // can move: left, right, up, down
        div(
            [
                class("data_view__normal_rows flex-column"),
                style! {
                    width: px(self.calculate_normal_rows_width()),
                    height: px(self.calculate_normal_rows_height()),
                },
                on_scroll(Msg::Scrolled),
            ],
            self.page_views
                .iter()
                .enumerate()
                .map(|(index, page_view)| {
                    page_view
                        .view()
                        .map_msg(move |page_msg| Msg::PageMsg(index, page_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    pub fn need_next_page(&self) -> bool {
        self.is_scrolled_near_bottom()
    }

    fn update_visible_pages(&mut self) {
        let visible_page = self.visible_page();
        let visible_pages = [visible_page - 1, visible_page, visible_page + 1];
        self.page_views
            .iter_mut()
            .enumerate()
            .for_each(|(page_index, page_view)| {
                if visible_pages.contains(&page_index) {
                    page_view.set_visible(true)
                } else {
                    page_view.set_visible(false);
                }
            });
    }
}
