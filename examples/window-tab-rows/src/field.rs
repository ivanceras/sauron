use sauron::{
    html::{attributes::*, events::*, *},
    Node, Widget,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Interaction {
    Click,
    Modify(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    FieldClick,
    InputChange(String),
    Interacted(Interaction),
}

pub struct Field<PMSG> {
    field_clicks: u32,
    field_name: String,
    on_interact: Vec<Box<dyn Fn(Interaction) -> PMSG>>,
}

impl<PMSG> Field<PMSG> {
    pub fn new(field_name: String) -> Self {
        Field {
            field_clicks: 0,
            field_name,
            on_interact: vec![],
        }
    }

    pub fn add_interaction_listener(
        &mut self,
        listener: Box<dyn Fn(Interaction) -> PMSG>,
    ) {
        self.on_interact.push(listener);
    }
}

impl<PMSG> Widget<Msg, PMSG> for Field<PMSG> {
    fn update(&mut self, msg: Msg) -> (Vec<Msg>, Vec<PMSG>) {
        match msg {
            Msg::FieldClick => {
                self.field_clicks += 1;
                (vec![Msg::Interacted(Interaction::Click)], vec![])
            }
            Msg::InputChange(input) => {
                (vec![Msg::Interacted(Interaction::Modify(input))], vec![])
            }
            Msg::Interacted(interaction) => (
                vec![],
                self.on_interact
                    .iter()
                    .map(|listener| listener(interaction.clone()))
                    .collect(),
            ),
        }
    }

    fn view(&self) -> Node<Msg> {
        input(
            vec![
                r#type("text"),
                class("field"),
                on_click(|_| Msg::FieldClick),
                on_input(|input| Msg::InputChange(input.value)),
            ],
            vec![text(format!("{} ({})", self.field_name, self.field_clicks))],
        )
    }
}
