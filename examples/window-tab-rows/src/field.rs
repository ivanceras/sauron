use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    Component,
    Effects,
    Node,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Interaction {
    Click,
    Modify(String),
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    FieldClick,
    InputChange(String),
    Interacted(Interaction),
}

pub struct Field<XMSG> {
    field_clicks: u32,
    field_name: String,
    on_interact: Vec<Box<dyn Fn(Interaction) -> XMSG>>,
}

impl<XMSG> Field<XMSG> {
    pub fn new(field_name: String) -> Self {
        Field {
            field_clicks: 0,
            field_name,
            on_interact: vec![],
        }
    }

    pub fn add_interaction_listener(
        &mut self,
        listener: Box<dyn Fn(Interaction) -> XMSG>,
    ) {
        self.on_interact.push(listener);
    }
}

impl<XMSG> Component<Msg, XMSG> for Field<XMSG> {
    fn update(&mut self, msg: Msg) -> Effects<Msg, XMSG> {
        match msg {
            Msg::FieldClick => {
                self.field_clicks += 1;
                Effects::with_local(vec![Msg::Interacted(Interaction::Click)])
            }
            Msg::InputChange(input) => {
                Effects::with_local(vec![Msg::Interacted(Interaction::Modify(
                    input,
                ))])
            }
            Msg::Interacted(interaction) => {
                Effects::with_external(
                    self.on_interact
                        .iter()
                        .map(|listener| listener(interaction.clone())),
                )
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        input(
            [
                r#type("text"),
                class("field"),
                on_click(|_| Msg::FieldClick),
                on_input(|input| Msg::InputChange(input.value)),
            ],
            [text(format!("{} ({})", self.field_name, self.field_clicks))],
        )
    }
}
