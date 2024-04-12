use crate::views::{data_view, DataView};
use sauron::Window;
use sauron::{
    html::{attributes::*, events::*, *},
    jss, Application, Cmd, Component, Node,
};

#[derive(Debug, PartialEq)]
pub enum Msg {
    DataViewMsg(data_view::Msg),
    MouseMove(i32, i32),
    EndResize(i32, i32),
    StartResize(Grip, i32, i32),
}

/// provides a resizable wrapper for the DataView
pub struct App {
    data_view: DataView,
    active_resize: Option<Grip>,
    width: i32,
    height: i32,
    start_x: i32,
    start_y: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Grip {
    Right,
    Bottom,
    BottomRight,
}

impl App {
    pub fn new(data_view: DataView, width: i32, height: i32) -> Self {
        App {
            data_view,
            active_resize: None,
            width,
            height,
            start_x: 0,
            start_y: 0,
        }
    }
}

impl Application for App {
    type MSG = Msg;

    /// Setup the resize wrapper to listen to the mouseup
    /// and mousemove event of the Window
    /// to have a continuity and ensure that the mousemouve
    /// event is always captured.
    /// Unliked when listen by the this view container, which the mouse
    /// can be outside of this view, which causes the mousmove event
    /// not being triggered
    fn init(&mut self) -> Cmd<Msg> {
        Cmd::batch([
            Window::on_mouseup(|event| Msg::EndResize(event.client_x(), event.client_y())),
            Window::on_mousemove(|event| Msg::MouseMove(event.client_x(), event.client_y())),
            Cmd::from(self.data_view.init().map_msg(Msg::DataViewMsg)),
        ])
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::DataViewMsg(data_view_msg) => {
                let effects = self.data_view.update(data_view_msg);
                Cmd::from(effects.map_msg(Msg::DataViewMsg))
            }
            Msg::EndResize(client_x, client_y) => {
                self.active_resize = None;
                let effects = self
                    .data_view
                    .update(data_view::Msg::ColumnEndResize(client_x, client_y));
                Cmd::from(effects.map_msg(Msg::DataViewMsg))
            }
            Msg::MouseMove(client_x, client_y) => {
                if let Some(active_resize) = &self.active_resize {
                    match active_resize {
                        Grip::BottomRight => {
                            let delta_x = client_x - self.start_x;
                            let delta_y = client_y - self.start_y;
                            self.width += delta_x;
                            self.height += delta_y;
                            self.start_x = client_x;
                            self.start_y = client_y;
                        }
                        Grip::Right => {
                            let delta_x = client_x - self.start_x;
                            self.width += delta_x;
                            self.start_x = client_x;
                        }
                        Grip::Bottom => {
                            let delta_y = client_y - self.start_y;
                            self.height += delta_y;
                            self.start_y = client_y;
                        }
                    }
                    self.data_view.set_allocated_size(self.width, self.height);
                }
                let effects = self
                    .data_view
                    .update(data_view::Msg::MouseMove(client_x, client_y));
                Cmd::from(effects.map_msg(Msg::DataViewMsg))
            }
            Msg::StartResize(grip, client_x, client_y) => {
                self.active_resize = Some(grip);
                self.start_x = client_x;
                self.start_y = client_y;
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        main(
            [class("resize_wrapper grid")],
            [
                self.data_view.view().map_msg(Msg::DataViewMsg),
                div(
                    [
                        class("resize_wrapper__resize_grip resize_wrapper__resize_grip--right"),
                        on_mousedown(|event| {
                            Msg::StartResize(Grip::Right, event.client_x(), event.client_y())
                        }),
                    ],
                    [],
                ),
                div(
                    [
                        class("resize_wrapper__resize_grip resize_wrapper__resize_grip--bottom"),
                        on_mousedown(|event| {
                            Msg::StartResize(Grip::Bottom, event.client_x(), event.client_y())
                        }),
                    ],
                    [],
                ),
                div(
                    [
                        class(
                            "resize_wrapper__resize_grip resize_wrapper__resize_grip--bottom_right",
                        ),
                        on_mousedown(|event| {
                            Msg::StartResize(Grip::BottomRight, event.client_x(), event.client_y())
                        }),
                    ],
                    [],
                ),
                a(
                    [href(
                        "https://github.com/ivanceras/sauron/tree/master/examples/data-viewer/",
                    )],
                    [text("code")],
                ),
            ],
        )
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }]
    }
}
