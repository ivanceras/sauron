use crate::views::{data_view, DataView};

use sauron::{
    html::{attributes::class, events::*, *},
    Cmd, Component, Node, Window, *,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    DataViewMsg(data_view::Msg),
    MouseMove(i32, i32),
    EndResize(i32, i32),
    StartResize(Grip, i32, i32),
}

/// provides a resizable wrapper for the DataView
pub struct ResizeWrapper {
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

impl ResizeWrapper {
    pub fn new(data_view: DataView, width: i32, height: i32) -> Self {
        ResizeWrapper {
            data_view,
            active_resize: None,
            width,
            height,
            start_x: 0,
            start_y: 0,
        }
    }
}

impl Component<Msg> for ResizeWrapper {
    /// Setup the resize wrapper to listen to the mouseup
    /// and mousemove event of the Window
    /// to have a continuity and ensure that the mousemouve
    /// event is always captured.
    /// Unliked when listen by the this view container, which the mouse
    /// can be outside of this view, which causes the mousmove event
    /// not being triggered
    fn init(&self) -> Cmd<Self, Msg> {
        let mut cmds = vec![Window::add_event_listeners(vec![
            onmouseup(|event| {
                Msg::EndResize(
                    event.coordinate.client_x,
                    event.coordinate.client_y,
                )
            }),
            onmousemove(|event| {
                Msg::MouseMove(
                    event.coordinate.client_x,
                    event.coordinate.client_y,
                )
            }),
        ])];
        cmds.push(DataView::init());
        Cmd::batch(cmds)
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::DataViewMsg(data_view_msg) => {
                self.data_view.update(data_view_msg);
                Cmd::none()
            }
            Msg::EndResize(_client_x, _client_y) => {
                self.active_resize = None;
                Cmd::none()
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
                Cmd::none()
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
            vec![class("resize_wrapper grid")],
            vec![
                self.data_view.view().map_msg(Msg::DataViewMsg),
                div(
                    vec![
                        class(
                            "resize_wrapper__resize_grip resize_wrapper__resize_grip--right",
                        ),
                        onmousedown(|event| {
                            Msg::StartResize(
                                Grip::Right,
                                event.coordinate.client_x,
                                event.coordinate.client_y,
                            )
                        }),
                    ],
                    vec![],
                ),
                div(
                    vec![
                        class(
                            "resize_wrapper__resize_grip resize_wrapper__resize_grip--bottom",
                        ),
                        onmousedown(|event| {
                            Msg::StartResize(
                                Grip::Bottom,
                                event.coordinate.client_x,
                                event.coordinate.client_y,
                            )
                        }),
                    ],
                    vec![],
                ),
                div(
                    vec![
                        class(
                            "resize_wrapper__resize_grip resize_wrapper__resize_grip--bottom_right",
                        ),
                        onmousedown(|event| {
                            Msg::StartResize(
                                Grip::BottomRight,
                                event.coordinate.client_x,
                                event.coordinate.client_y,
                            )
                        }),
                    ],
                    vec![],
                ),
                a!(
                    [html::attributes::href(
                        "https://github.com/ivanceras/sauron/tree/master/examples/data-viewer/"
                    )],
                    [text!("code")]
                ),
            ],
        )
    }
}
