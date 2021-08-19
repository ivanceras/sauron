use sauron::apply_patches::patch;
use sauron::prelude::*;
use sauron::Callback;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

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
    time_change_listener: Vec<Callback<PMSG>>,
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

    fn force_increment(&mut self) {
        self.cnt += 1;
    }

    pub fn on_date_time_change<F>(&mut self, f: F)
    where
        F: Fn(Event) -> PMSG + 'static,
    {
        self.time_change_listener.push(Callback::from(f));
    }
}

impl<PMSG> Widget<Msg, PMSG> for DateTimeWidget<PMSG>
where
    PMSG: Clone + Debug + 'static,
{
    fn update(&mut self, msg: Msg) -> (Vec<Msg>, Vec<PMSG>) {
        match msg {
            Msg::DateChange(date) => {
                log::trace!("date is changed to: {}", date);
                (vec![Msg::TimeOrDateModified(self.date_time())], vec![])
            }
            Msg::TimeChange(time) => {
                log::trace!("time is changed to: {}", time);
                (vec![Msg::TimeOrDateModified(self.date_time())], vec![])
            }
            Msg::TimeOrDateModified(date_time) => {
                log::trace!("time or date is changed: {}", date_time);
                let mut parent_msg = vec![];
                for listener in self.time_change_listener.iter() {
                    let event = web_sys::Event::new("time_change")
                        .expect("must construct custom event");
                    let event: Event = event.into();
                    let pmsg = listener.emit(event);
                    parent_msg.push(pmsg);
                }
                log::trace!("sending this to parent: {:?}", parent_msg);
                (vec![], parent_msg)
            }
            Msg::Mount(target_node) => {
                /*
                //log::debug!("Mounting attempt...");
                if !self.mounted {
                    log::debug!("replacing the original target");
                    let mut self_clone = self.clone();
                    self_clone.mounted = true;
                    self_clone.date = "2020-02-02".to_string();
                    self_clone.time = "22:22".to_string();
                    Program::replace_mount(self_clone, &target_node);
                    self.mounted = true;
                }
                */
                (vec![], vec![])
            }
            Msg::BtnClick => {
                log::trace!("btn is clicked..");
                self.cnt += 1;
                (vec![], vec![])
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
                        on_change(|input| Msg::DateChange(input.value)),
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
