use sauron::apply_patches::patch;
use sauron::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    Mount(web_sys::Node),
    BtnClick,
}

#[derive(Clone)]
pub struct DateTimeWidget {
    date: String,
    time: String,
    root_node: Option<web_sys::Node>,
    program: Option<Program<Self, Msg>>,
    cnt: i32,
}

impl DateTimeWidget {
    pub fn new(date: &str, time: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(DateTimeWidget {
            date: date.to_string(),
            time: time.to_string(),
            root_node: None,
            program: None,
            cnt: 0,
        }))
    }

    fn date_time(&self) -> String {
        format!("{} {}", self.date, self.time)
    }

    fn force_increment(&mut self) {
        self.cnt += 1;
        let view = self.view();
        if let Some(program) = self.program.as_mut() {
            log::warn!("manually triggering dom_updater");
            program.dispatch(Msg::TimeChange("11:11".to_string()));
            program.dispatch(Msg::BtnClick)
        }
    }
}

impl Component<Msg> for DateTimeWidget {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::DateChange(date) => {
                log::trace!("date is changed to: {}", date);
                self.date = date;
                let date_time = self.date_time();
                Cmd::none()
            }
            Msg::TimeChange(time) => {
                log::trace!("time is changed to: {}", time);
                let date_time = self.date_time();
                Cmd::none()
            }
            Msg::Mount(target_node) => {
                log::trace!("mounted to: {:?}", target_node);
                self.root_node = Some(target_node);
                if let Some(root_node) = &self.root_node {
                    self.program = Some(Program::new(self.clone(), root_node));
                }
                self.force_increment();
                Cmd::none()
            }
            Msg::BtnClick => {
                log::trace!("btn is clicked..");
                self.cnt += 1;
                let view = self.view();
                if let Some(program) = self.program.as_mut() {
                    log::warn!("manually triggering dom_updater");
                    program.dispatch(Msg::BtnClick);
                }
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let program = self.program.clone();
        div(
            vec![
                class("datetimebox"),
                on_mount(|me| Msg::Mount(me.target_node)),
            ],
            vec![
                input(
                    vec![
                        type_("date"),
                        class("datetimebox__date"),
                        on_change(|input| Msg::DateChange(input.value)),
                        value(&self.date),
                    ],
                    vec![],
                ),
                input(
                    vec![
                        type_("time"),
                        class("datetimebox__time"),
                        on_change(|input| Msg::TimeChange(input.value)),
                        value(&self.time),
                    ],
                    vec![],
                ),
                input(vec![type_("text"), value(self.cnt)], vec![]),
                button(
                    vec![on_click(move |_| {
                        if let Some(program) = &program {
                            program.dispatch(Msg::BtnClick)
                        }
                        Msg::BtnClick
                    })],
                    vec![text("Do something")],
                ),
            ],
        )
    }
}
