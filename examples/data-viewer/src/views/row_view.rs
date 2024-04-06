use crate::views::{field_view, FieldView};
use restq::{ColumnDef, DataValue};
use sauron::{
    html::{attributes::*, events::*, units::*, *},
    style, Component, Effects, Node,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum Msg {
    FieldMsg(usize, field_view::Msg),
    DoubleClick,
    Click,
}

pub struct RowView {
    pub index: usize,
    pub fields: Vec<Rc<RefCell<FieldView>>>,
    frozen_fields: Vec<usize>,
    pub is_frozen: bool,
}

impl RowView {
    pub fn new(index: usize, data_rows: &[DataValue], data_columns: &[ColumnDef]) -> Self {
        RowView {
            index,
            fields: data_rows
                .iter()
                .zip(data_columns.iter())
                .map(|(value, column)| Rc::new(RefCell::new(FieldView::new(value, column))))
                .collect(),
            frozen_fields: vec![],
            is_frozen: false,
        }
    }
}

impl Component for RowView {
    type MSG = Msg;
    type XMSG = ();

    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::FieldMsg(field_index, field_msg) => {
                self.fields[field_index].borrow_mut().update(field_msg);
                Effects::none()
            }
            Msg::DoubleClick => Effects::none(),
            Msg::Click => Effects::none(),
        }
    }

    fn view(&self) -> Node<Msg> {
        self.view_with_filter(|(_index, field)| field.borrow().is_normal_field())
    }
}

impl RowView {
    /// is value of any field modified
    pub fn is_changed(&self) -> bool {
        self.fields.iter().any(|field| field.borrow().is_changed())
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
        self.frozen_fields = columns;
        self.update_frozen_column_fields();
    }

    /// set the width of all the fields initialized
    /// when the data is first loaded
    pub fn set_column_widths(&mut self, column_widths: &[i32]) {
        self.fields
            .iter()
            .zip(column_widths.iter())
            .for_each(|(field, width)| field.borrow_mut().width = *width);
    }

    /// set only the width of the currently actively field width being dragged
    pub fn set_field_width(&mut self, field_index: usize, field_width: i32) {
        if field_width >= 0 {
            self.fields[field_index].borrow_mut().width = field_width;
        }
    }

    fn view_with_filter<F>(&self, filter: F) -> Node<Msg>
    where
        F: Fn(&(usize, &Rc<RefCell<FieldView>>)) -> bool,
    {
        li(
            [
                class("row_view flex-row"),
                // IMPORTANT: key is needed here to avoid sauron
                // reusing dom elements of similar rows, this is needed
                // so as to complete remove the dom and it's attached events
                // since events attached in a dom are not compared
                // and is not replaced.
                key(format!("row_{}", self.index)),
                classes_flag([
                    ("row_view--frozen_row", self.is_frozen),
                    ("row_view--modified", self.is_changed()),
                ]),
                style! {height: px(Self::row_height())},
                on_click(|_| Msg::Click),
                on_dblclick(|_| Msg::DoubleClick),
            ],
            self.fields
                .iter()
                .enumerate()
                .filter(filter)
                .map(|(index, field)| {
                    field
                        .borrow()
                        .view()
                        .map_msg(move |field_msg| Msg::FieldMsg(index, field_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// These are the rows that are completely immovable at the top left corner of the table
    /// they can not move since both their rows and columns are in frozen state.
    pub fn view_immovable_fields(&self) -> Node<Msg> {
        self.view_with_filter(|(_index, field)| field.borrow().is_immovable())
    }

    /// frozen columns field, but not a frozen row field
    pub fn view_frozen_columns(&self) -> Node<Msg> {
        self.view_with_filter(|(_index, field)| {
            field.borrow().is_frozen_column && !field.borrow().is_frozen_row
        })
    }

    pub fn row_height() -> i32 {
        30
    }

    pub fn set_is_frozen(&mut self, is_frozen: bool) {
        self.is_frozen = is_frozen;
        self.update_frozen_row_fields();
    }

    pub fn update_frozen_row_fields(&mut self) {
        self.fields
            .iter()
            .for_each(|field| field.borrow_mut().set_is_frozen_row(self.is_frozen))
    }

    pub fn update_frozen_column_fields(&mut self) {
        self.fields.iter().enumerate().for_each(|(index, field)| {
            if self.frozen_fields.contains(&index) {
                field.borrow_mut().set_is_frozen_column(true)
            } else {
                field.borrow_mut().set_is_frozen_column(false)
            }
        })
    }

    pub fn view_frozen(&self) -> Node<Msg> {
        self.view_with_filter(|(_index, field)| {
            field.borrow().is_frozen_row && !field.borrow().is_frozen_column
        })
    }
}
