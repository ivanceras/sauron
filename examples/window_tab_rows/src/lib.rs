#![deny(warnings)]
use sauron::{
    html::{attributes::*, events::*, *},
    Cmd, Component, Node, Program,
};
use tab::Tab;
use wasm_bindgen::prelude::*;

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

impl Component<Msg> for Window {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        self.window_activities += 1;
        match msg {
            Msg::WindowClick => Cmd::none(),
            Msg::ActivateTab(index) => {
                self.activate_tab(index);
                Cmd::none()
            }
            Msg::TabMsg(index, tab_msg) => {
                self.tabs[index].update(tab_msg);
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![class("window")],
            vec![
                button(
                    vec![onclick(|_| Msg::WindowClick)],
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
                                    onclick(move |_| Msg::ActivateTab(index)),
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
    Program::mount_to_body(Window::new());
}
