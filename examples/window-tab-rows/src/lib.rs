#![deny(warnings)]
use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    jss,
    prelude::*,
    Application,
    Cmd,
    Node,
    Program,
};
use tab::Tab;

mod field;
mod row;
mod tab;

#[derive(Debug, PartialEq)]
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

#[async_trait(?Send)]
impl Application<Msg> for Window {
    async fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        self.window_activities += 1;
        match msg {
            Msg::WindowClick => Cmd::none(),
            Msg::ActivateTab(index) => {
                self.activate_tab(index);
                Cmd::none()
            }
            Msg::TabMsg(index, tab_msg) => {
                let effects = self.tabs[index].update(tab_msg).await;
                let effects = effects
                    .map_msg(move |follow_up| Msg::TabMsg(index, follow_up));
                Cmd::from(effects)
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [class("window")],
            [
                button(
                    [on_click(|_| Msg::WindowClick)],
                    [text(format!(
                        "Total window activities: {}",
                        self.window_activities
                    ))],
                ),
                div(
                    [class("tab-list-buttons")],
                    self.tabs.iter().enumerate().map(|(index, tab)| {
                        button(
                            [
                                class("tablink"),
                                styles([("background-color", &tab.color)]),
                                on_click(move |_| Msg::ActivateTab(index)),
                            ],
                            [text(&tab.name)],
                        )
                    }),
                ),
                div(
                    [class("tab-list")],
                    self.tabs.iter().enumerate().map(|(index, tab)| {
                        Tab::view(tab)
                            .map_msg(move |tab_msg| Msg::TabMsg(index, tab_msg))
                    }),
                ),
            ],
        )
    }

    fn style(&self) -> String {
        jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(Window::new());
}
