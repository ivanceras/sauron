use crate::field::{self, Field, Interaction};
use sauron::{
    html::{attributes::*, *},
    prelude::*,
    Node,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    FieldMsg(usize, field::Msg),
    FieldInteracted(field::Interaction),
}

pub struct Row {
    field_clicks: u32,
    field_change: u32,
    fields: Vec<Field<Msg>>,
    row_name: String,
}

impl Row {
    pub fn new(row_name: String) -> Self {
        Row {
            field_clicks: 0,
            field_change: 0,
            fields: (0..8)
                .into_iter()
                .map(|index| {
                    let mut field = Field::new(format!("Field {}", index));
                    field.add_interaction_listener(Box::new(|action| {
                        Msg::FieldInteracted(action)
                    }));
                    field
                })
                .collect(),
            row_name,
        }
    }
}

impl Component<Msg, ()> for Row {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::FieldMsg(index, field_msg) => {
                let Effects {
                    follow_ups,
                    effects,
                } = self.fields[index].update(field_msg);

                Effects::with_follow_ups(
                    effects
                        .into_iter()
                        .chain(
                            follow_ups.into_iter().map(|follow_up| {
                                Msg::FieldMsg(index, follow_up)
                            }),
                        )
                        .collect(),
                )
            }
            Msg::FieldInteracted(interaction) => {
                log::trace!("interacted: {:?}", interaction);
                match interaction {
                    Interaction::Click => self.field_clicks += 1,
                    Interaction::Modify(_) => self.field_change += 1,
                }
                Effects::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![class("row")],
            vec![
                text(&self.row_name),
                input(vec![class("row-selector"), r#type("checkbox")], vec![]),
                div(
                    vec![],
                    self.fields
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            field.view().map_msg(move |field_msg| {
                                Msg::FieldMsg(index, field_msg)
                            })
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
                span(
                    vec![],
                    vec![text(format!(
                        "field clicks: {}, field changed: {}",
                        self.field_clicks, self.field_change,
                    ))],
                ),
            ],
        )
    }
}
