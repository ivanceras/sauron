use sauron::dom::Callback;
use sauron::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    TimeOrDateModified(String),
    BtnClick,
}

#[derive(Debug, Clone, Default)]
pub struct DateTimeWidget {
    date: String,
    time: String,
    cnt: i32,
    time_change_listener: Vec<Callback<String, ()>>,
}

impl DateTimeWidget {
    pub fn new(date: &str, time: &str) -> Self {
        DateTimeWidget {
            date: date.to_string(),
            time: time.to_string(),
            cnt: 0,
            time_change_listener: vec![],
        }
    }

    fn date_time(&self) -> String {
        format!("{} {}", self.date, self.time)
    }

    pub fn on_date_time_change<F>(mut self, f: F) -> Self
    where
        F: Fn(String) + 'static,
    {
        self.time_change_listener.push(Callback::from(f));
        self
    }

    pub fn observed_attributes() -> Vec<&'static str> {
        vec!["date", "time"]
    }

    pub fn attribute_changed(&mut self, attributes: BTreeMap<String, String>) {
        log::info!("attributes: {:#?}", attributes);
        for (key, value) in attributes {
            match &*key {
                "date" => self.date = value,
                "time" => self.time = value,
                _ => log::info!("unused attribute: {}", key),
            }
        }
    }
}

impl Application<Msg> for DateTimeWidget {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::DateChange(date) => {
                log::trace!("date is changed to: {}", date);
                self.date = date;
                Cmd::batch_msg(vec![Msg::TimeOrDateModified(self.date_time())])
            }
            Msg::TimeChange(time) => {
                log::trace!("time is changed to: {}", time);
                self.time = time;
                Cmd::batch_msg(vec![Msg::TimeOrDateModified(self.date_time())])
            }
            Msg::TimeOrDateModified(date_time) => {
                log::trace!("time or date is changed: {}", date_time);
                let mut parent_msg = vec![];
                for listener in self.time_change_listener.iter() {
                    let pmsg = listener.emit(self.date_time());
                    parent_msg.push(pmsg);
                }
                log::trace!("sending this to parent: {:?}", parent_msg);
                Cmd::none()
            }
            Msg::BtnClick => {
                log::trace!("btn is clicked..");
                self.cnt += 1;
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [class("datetimebox")],
            [
                input(
                    [
                        r#type("date"),
                        class("datetimebox__date"),
                        on_change(|input| {
                            log::trace!("input: {:?}", input);
                            Msg::DateChange(input.value)
                        }),
                        value(&self.date),
                    ],
                    [],
                ),
                input(
                    [
                        r#type("time"),
                        class("datetimebox__time"),
                        on_change(|input| Msg::TimeChange(input.value)),
                        value(&self.time),
                    ],
                    [],
                ),
                input([r#type("text"), value(self.cnt)], []),
                button(
                    [on_click(move |_| Msg::BtnClick)],
                    [text("Do something")],
                ),
            ],
        )
    }
}
