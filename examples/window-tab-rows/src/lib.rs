#![deny(warnings)]
use sauron::{
    html::{attributes::*, events::*, *},
    prelude::*,
    Application, Cmd, Node, Program,
};
use tab::Tab;

mod field;
mod row;
mod tab;

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    WindowClick,
    ActivateTab(usize),
    TabMsg(usize, tab::Msg),
}

pub struct Window {
    window_activities: u32,
    tabs: Vec<Tab>,
    active_tab: usize,
}

impl Window {
    pub fn new() -> Self {
        let mut window = Window {
            window_activities: 0,
            tabs: vec![
                Tab::new("First tab", "peachpuff"),
                Tab::new("Second tab", "lightyellow"),
                Tab::new("Third tab", "lightblue"),
                Tab::new("Fourth tab", "papayawhip"),
            ],
            active_tab: 0,
        };
        window.update_active_tab();
        window
    }

    fn update_active_tab(&mut self) {
        let active_tab = self.active_tab;
        self.tabs
            .iter_mut()
            .enumerate()
            .map(|(index, tab)| {
                if index == active_tab {
                    tab.show()
                } else {
                    tab.hide()
                }
            })
            .collect()
    }

    fn activate_tab(&mut self, index: usize) {
        self.active_tab = index;
        self.update_active_tab();
    }
}

impl Application<Msg> for Window {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        self.window_activities += 1;
        match msg {
            Msg::WindowClick => Cmd::none(),
            Msg::ActivateTab(index) => {
                self.activate_tab(index);
                Cmd::none()
            }
            Msg::TabMsg(index, tab_msg) => {
                let follow_ups = self.tabs[index].update(tab_msg);
                //TODO: maybe we can use partial_function here: https://docs.rs/partial_function
                Cmd::batch_msg(
                    follow_ups
                        .into_iter()
                        .map(|follow_up| Msg::TabMsg(index, follow_up))
                        .collect(),
                )
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![class("window")],
            vec![
                button(
                    vec![on_click(|_| Msg::WindowClick)],
                    vec![text(format!(
                        "Total window activities: {}",
                        self.window_activities
                    ))],
                ),
                div(
                    vec![class("tab-list-buttons")],
                    self.tabs
                        .iter()
                        .enumerate()
                        .map(|(index, tab)| {
                            button(
                                vec![
                                    class("tablink"),
                                    styles([("background-color", &tab.color)]),
                                    on_click(move |_| Msg::ActivateTab(index)),
                                ],
                                vec![text(&tab.name)],
                            )
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
                div(
                    vec![class("tab-list")],
                    self.tabs
                        .iter()
                        .enumerate()
                        .map(|(index, tab)| {
                            Tab::view(tab).map_msg(move |tab_msg| {
                                Msg::TabMsg(index, tab_msg)
                            })
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
            ],
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(Window::new());
}
