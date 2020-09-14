use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    Cmd,
    Component,
    Node,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    FieldClick,
}

pub struct Field {
    field_clicks: u32,
    field_name: String,
}

impl Field {
    pub fn new(field_name: String) -> Self {
        Field {
            field_clicks: 0,
            field_name,
        }
    }
}

impl Component<Msg> for Field {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::FieldClick => self.field_clicks += 1,
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        button(
            vec![class("field"), on_click(|_| Msg::FieldClick)],
            vec![text(format!("{} ({})", self.field_name, self.field_clicks))],
        )
    }
}
