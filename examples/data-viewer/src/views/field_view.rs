use crate::widgets::*;
use restq::{
    ast::Value, data_value::cast_data_value, ColumnDef, DataType, DataValue,
};
use sauron::{
    html::{
        attributes::{class, classes_flag, r#type, styles},
        events::*,
        units::px,
        *,
    },
    Application, Attribute, Cmd, Node,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    TextChange(String),
    PrimaryClicked,
    CheckedChange(String),
}

#[derive(Clone)]
pub struct FieldView {
    pub column: ColumnDef,
    pub value: DataValue,
    pub new_value: DataValue,
    /// is part of a frozen row, serves no
    /// other purposed other than coloring in css style
    pub is_frozen_row: bool,
    /// is part of a frozen column, serves no
    /// other puposed other than coloring in css style
    pub is_frozen_column: bool,
    pub width: i32,
    pub height: i32,
}

impl FieldView {
    pub fn new(value: &DataValue, column: &ColumnDef) -> Self {
        info!("field value: {:?}", value);
        FieldView {
            new_value: value.clone(),
            value: value.clone(),
            column: column.clone(),
            is_frozen_row: false,
            is_frozen_column: false,
            width: 200,
            height: 23,
        }
    }

    pub fn padding_top() -> i32 {
        6
    }

    pub fn padding_bottom() -> i32 {
        0
    }

    pub fn side_padding() -> i32 {
        5
    }

    pub fn is_immovable(&self) -> bool {
        self.is_frozen_row && self.is_frozen_column
    }

    pub fn is_normal_field(&self) -> bool {
        !self.is_frozen_row && !self.is_frozen_column
    }

    pub fn is_changed(&self) -> bool {
        self.value != self.new_value
    }

    pub fn set_is_frozen_row(&mut self, frozen: bool) {
        self.is_frozen_row = frozen;
    }

    pub fn set_is_frozen_column(&mut self, frozen: bool) {
        self.is_frozen_column = frozen;
    }

    fn css_classes(&self) -> Attribute<Msg> {
        classes_flag(vec![
            ("field_view__value", true),
            ("field_view__value--frozen_row", self.is_frozen_row),
            ("field_view__value--frozen_column", self.is_frozen_column),
            ("field_view__value--modified", self.is_changed()),
        ])
    }

    fn css_size(&self) -> Attribute<Msg> {
        styles(vec![("width", px(self.width)), ("height", px(self.height))])
    }

    fn css_padding(&self) -> Attribute<Msg> {
        styles(vec![(
            "padding",
            [
                px(Self::padding_top()),
                px(Self::side_padding()),
                px(Self::padding_bottom()),
                px(Self::side_padding()),
            ],
        )])
    }

    fn view_value_as_primary(&self) -> Node<Msg> {
        let classes = self.css_classes();
        let size = self.css_size();
        let padding = self.css_padding();

        match &self.value {
            DataValue::U32(v) => text_link(
                v.to_string(),
                format!("#{}", v),
                vec![classes, size, padding, on_click(|_| Msg::PrimaryClicked)],
            ),

            DataValue::S32(v) => text_link(
                v.to_string(),
                format!("#{}", v),
                vec![classes, size, padding, on_click(|_| Msg::PrimaryClicked)],
            ),
            _ => {
                trace!("todo primary: {:?}", self.value);
                text("unknown")
            }
        }
    }

    fn view_value(&self) -> Node<Msg> {
        let classes = self.css_classes();
        let size = self.css_size();
        let padding = self.css_padding();

        match &self.value {
            DataValue::Nil => match self.column.data_type_def.data_type {
                DataType::Bool => checkbox(
                    false,
                    vec![classes, size, padding],
                    vec![on_change(|input| {
                        Msg::CheckedChange(input.value.to_string())
                    })],
                ),
                _ => textbox("", vec![r#type("text"), classes]),
            },
            DataValue::Text(v) => textbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::Uuid(v) => textbox(
                v.to_string(),
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::Bool(v) => checkbox(
                *v,
                vec![classes, size, padding],
                vec![on_change(|input| {
                    Msg::CheckedChange(input.value.to_string())
                })],
            ),
            DataValue::S8(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::S16(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::S32(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::S64(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::U8(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::U16(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::U32(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::U64(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::F32(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::F64(v) => numberbox(
                v,
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::Utc(v) => datebox(
                v.format("%Y-%m-%d").to_string(),
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            DataValue::Local(v) => datebox(
                v.format("%Y-%m-%d").to_string(),
                vec![
                    classes,
                    size,
                    padding,
                    on_change(|input| Msg::TextChange(input.value.to_string())),
                ],
            ),
            _ => {
                trace!("todo for: {:?}", self.value);
                text("unknown")
            }
        }
    }

    pub fn view_in_detail(&self) -> Node<Msg> {
        div(
            vec![
                class("field_view--detail flex-row"),
                classes_flag(vec![
                    ("field_view--detail--frozen_row", self.is_frozen_row),
                    (
                        "field_view--detail--frozen_column",
                        self.is_frozen_column,
                    ),
                ]),
            ],
            vec![
                label(
                    vec![class("field_view__column--detail")],
                    vec![text(&self.column.column.name)],
                ),
                if self.column.is_primary() {
                    self.view_value_as_primary()
                } else {
                    self.view_value()
                },
            ],
        )
    }
}

impl Application<Msg> for FieldView {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        trace!("field updated: {:?}", msg);
        match msg {
            Msg::TextChange(value) => {
                debug!("text changed..{}", value);
                //TODO: cast to the original data type
                self.new_value = cast_data_value(
                    &Value::String(value),
                    &self.column.data_type_def.data_type,
                );
                Cmd::none()
            }
            Msg::PrimaryClicked => {
                trace!("Primary clicked");
                Cmd::none()
            }
            Msg::CheckedChange(_value) => {
                let bnew = if let DataValue::Bool(bvalue) = self.new_value {
                    !bvalue
                } else {
                    true
                };
                self.new_value = DataValue::Bool(bnew);
                trace!("new value: {:?}", self.new_value);
                Cmd::none()
            }
        }
    }

    /// when viewed as row
    fn view(&self) -> Node<Msg> {
        div(
            vec![
                class("field_view"),
                classes_flag(vec![
                    ("field_view--frozen_row", self.is_frozen_row),
                    ("field_view--frozen_column", self.is_frozen_column),
                ]),
            ],
            vec![self.view_value()],
        )
    }
}
