use sauron::dom::Callback;
use sauron::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    TimeOrDateModified(String),
    Mount(web_sys::Node),
    BtnClick,
}

#[derive(Clone)]
pub struct DateTimeWidget<PMSG> {
    date: String,
    time: String,
    cnt: i32,
    mounted: bool,
    time_change_listener: Vec<Callback<String, PMSG>>,
}

impl<PMSG> DateTimeWidget<PMSG>
where
    PMSG: 'static,
{
    pub fn new(date: &str, time: &str, mounted: bool) -> Self {
        DateTimeWidget {
            date: date.to_string(),
            time: time.to_string(),
            cnt: 0,
            mounted,
            time_change_listener: vec![],
        }
    }

    fn date_time(&self) -> String {
        format!("{} {}", self.date, self.time)
    }

    pub fn on_date_time_change<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> PMSG + 'static,
    {
        self.time_change_listener.push(Callback::from(f));
        self
    }
}

impl<PMSG> Component<Msg, PMSG> for DateTimeWidget<PMSG>
where
    PMSG: Clone + Debug + 'static,
{
    fn update(&mut self, msg: Msg) -> Effects<Msg, PMSG> {
        match msg {
            Msg::DateChange(date) => {
                log::trace!("date is changed to: {}", date);
                self.date = date;
                Effects::with_local(vec![Msg::TimeOrDateModified(
                    self.date_time(),
                )])
            }
            Msg::TimeChange(time) => {
                log::trace!("time is changed to: {}", time);
                self.time = time;
                Effects::with_local(vec![Msg::TimeOrDateModified(
                    self.date_time(),
                )])
            }
            Msg::TimeOrDateModified(date_time) => {
                log::trace!("time or date is changed: {}", date_time);
                let mut parent_msg = vec![];
                for listener in self.time_change_listener.iter() {
                    let pmsg = listener.emit(self.date_time());
                    parent_msg.push(pmsg);
                }
                log::trace!("sending this to parent: {:?}", parent_msg);
                Effects::with_external(parent_msg)
            }
            Msg::Mount(target_node) => {
                log::debug!("widget is mounted to {:?}", target_node);
                Effects::none()
            }
            Msg::BtnClick => {
                log::trace!("btn is clicked..");
                self.cnt += 1;
                Effects::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![
                class("datetimebox"),
                on_mount(|me| Msg::Mount(me.target_node)),
            ],
            vec![
                input(
                    vec![
                        r#type("date"),
                        class("datetimebox__date"),
                        on_change(|input| {
                            log::trace!("input: {:?}", input);
                            Msg::DateChange(input.value)
                        }),
                        value(&self.date),
                    ],
                    vec![],
                ),
                input(
                    vec![
                        r#type("time"),
                        class("datetimebox__time"),
                        on_change(|input| Msg::TimeChange(input.value)),
                        value(&self.time),
                    ],
                    vec![],
                ),
                input(vec![r#type("text"), value(self.cnt)], vec![]),
                button(
                    vec![on_click(move |_| Msg::BtnClick)],
                    vec![text("Do something")],
                ),
            ],
        )
    }
}
