//#![deny(warnings)]
use frame::Frame;
use fui_button::FuiButton;
use header::Header;
use paragraph::Paragraph;
use sauron::html::attributes::{class, id, style};
use sauron::html::events::on_click;
use sauron::html::{div, text};
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};
use spinner::Spinner;
use web_sys::HtmlAudioElement;

mod frame;
mod fui_button;
mod header;
mod paragraph;
mod spinner;

pub enum Msg {
    ReAnimateFrame,
    ReAnimateHeader,
    FrameMsg(frame::Msg),
    HeaderMsg(header::Msg),
    FuiButtonMsg(Box<fui_button::Msg<Self>>),
    SimpleFuiButtonMsg(Box<fui_button::Msg<Self>>),
    SkewedFuiButtonMsg(Box<fui_button::Msg<Self>>),
    SimpleSkewedFuiButtonMsg(Box<fui_button::Msg<Self>>),
    GreenFuiButtonMsg(Box<fui_button::Msg<Self>>),
    ParagraphMsg(Box<paragraph::Msg<Self>>),
    ReanimateParagraph,
    ReanimateAll,
    NoOp,
}

pub struct App {
    show: bool,
    header: Header,
    frame: Frame,
    fui_button: FuiButton<Msg>,
    simple_fui_button: FuiButton<Msg>,
    skewed_fui_button: FuiButton<Msg>,
    simple_skewed_fui_button: FuiButton<Msg>,
    green_fui_button: FuiButton<Msg>,
    spinner: Spinner<Msg>,
    paragraph: Paragraph<Msg>,
}

impl App {
    pub fn new() -> Self {
        let mut fui_button = FuiButton::<Msg>::new_with_label("Reanimate All");
        fui_button.add_event_listeners(vec![on_click(|_| Msg::ReanimateAll)]);

        let mut simple_fui_button = FuiButton::<Msg>::new_with_label("Simple");
        simple_fui_button.has_corners(false);

        let mut skewed_fui_button =
            FuiButton::<Msg>::new_with_label("Skewed button");
        skewed_fui_button.skewed(true);
        skewed_fui_button.has_corners(true);

        let mut simple_skewed_fui_button =
            FuiButton::<Msg>::new_with_label("Skewed simple");
        simple_skewed_fui_button.skewed(true);
        simple_skewed_fui_button.has_corners(false);

        let mut green_fui_button =
            FuiButton::<Msg>::new_with_label("Cyberpunk");
        green_fui_button.use_green(true);

        let paragraph_content = "This is an experimental demo showcasing usage of sauron[0] Component lifecycle to work alongside
        css transition, animation and timed DOM manipulation. This is also an exploration on how to add theming to the web framework.

        Sauron is a light-weight web framework designed to have you write least amount of code possible.

        [0]: https://github.com/ivanceras/sauron
        ";

        App {
            show: true,
            frame: Frame::new_with_content("Retro Futuristic UI in rust"),
            header: Header::new_with_content("Header"),
            fui_button,
            simple_fui_button,
            skewed_fui_button,
            simple_skewed_fui_button,
            green_fui_button,
            spinner: Spinner::new(),
            paragraph: Paragraph::new_with_content(paragraph_content),
        }
    }

    fn reanimate_all() -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            program.dispatch(Msg::ParagraphMsg(Box::new(
                paragraph::Msg::AnimateIn,
            )));
            program.dispatch(Msg::ReAnimateFrame);
            program.dispatch(Msg::HeaderMsg(header::Msg::TriggerAnimation));
        })
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd<Self, Msg> {
        Self::reanimate_all()
    }

    fn style(&self) -> Vec<String> {
        vec![r#"
        .container {
            color: #26dafd;
            font-size: 21px;
            line-height: 1.5;
            font-family: "Titillium Web", "sans-serif";
            margin: auto;
        }
        .container ::selection {
            color: #021114;
            text-shadow: none;
            background-color: #26dafd;
        }

        .futuristic-buttons {
            display: flex;
        }
        "#
        .to_string()]
        .into_iter()
        .chain(self.header.style().into_iter())
        .chain(self.frame.style().into_iter())
        .chain(self.fui_button.style().into_iter())
        .chain(self.spinner.style().into_iter())
        .chain(self.paragraph.style().into_iter())
        .collect()
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![],
            vec![
                div(
                    vec![class("container")],
                    vec![
                        button(
                            vec![
                                on_click(|_| Msg::ReAnimateHeader),
                                style("margin", "20px"),
                                style("display", "block"),
                            ],
                            vec![text("Animate Header")],
                        ),
                        self.header.view().map_msg(Msg::HeaderMsg),
                        button(
                            vec![
                                on_click(|_| Msg::ReAnimateFrame),
                                style("margin", "20px"),
                                style("display", "block"),
                            ],
                            vec![text("Animate Frame")],
                        ),
                        self.frame
                            .view()
                            .map_msg(|frame_msg| Msg::FrameMsg(frame_msg)),
                        div(
                            vec![
                                style("width", px(100)),
                                style("height", px(20)),
                            ],
                            vec![],
                        ),
                        div(vec![class("futuristic-buttons")], vec![
                            self.fui_button.view().map_msg(|fbtn_msg| {
                                Msg::FuiButtonMsg(Box::new(fbtn_msg))
                            }),
                            self.simple_fui_button.view().map_msg(|fbtn_msg| {
                                Msg::SimpleFuiButtonMsg(Box::new(fbtn_msg))
                            }),
                            self.green_fui_button.view().map_msg(|fbtn_msg| {
                                Msg::GreenFuiButtonMsg(Box::new(fbtn_msg))
                            }),
                            self.skewed_fui_button.view().map_msg(|fbtn_msg| {
                                Msg::SkewedFuiButtonMsg(Box::new(fbtn_msg))
                            }),
                            self.simple_skewed_fui_button.view().map_msg(|fbtn_msg| {
                                Msg::SimpleSkewedFuiButtonMsg(Box::new(fbtn_msg))
                            }),
                        ]),
                        button(
                            vec![
                                on_click(|_| Msg::ReanimateParagraph),
                                style("margin", "20px"),
                                style("display", "block"),
                            ],
                            vec![text("Animate paragraph")],
                        ),
                        p(
                            vec![styles([
                                ("position", "relative"),
                                ("display", "inline-block"),
                            ])],
                            vec![self.paragraph.view()],
                        ),
                        self.spinner.view(),
                        button(
                            vec![
                                on_click(|_| Msg::ReanimateAll),
                                style("margin", "20px"),
                                style("display", "block"),
                            ],
                            vec![text("Reanimate All")],
                        ),
                    ],
                ),
                footer(
                    vec![],
                    vec![a(
                        vec![href("https://github.com/ivanceras/sauron/tree/master/examples/futuristic-ui/")],
                        vec![text("code")],
                    )],
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ReAnimateFrame => {
                self.frame.update(frame::Msg::TriggerAnimation)
            }
            Msg::ReAnimateHeader => {
                self.header.update(header::Msg::TriggerAnimation)
            }
            Msg::FrameMsg(frame_msg) => self.frame.update(frame_msg),
            Msg::HeaderMsg(header_msg) => self.header.update(header_msg),
            Msg::FuiButtonMsg(fui_btn_msg) => {
                self.fui_button.update(*fui_btn_msg)
            }
            Msg::SimpleFuiButtonMsg(fui_btn_msg) => {
                self.simple_fui_button.update(*fui_btn_msg)
            }
            Msg::SkewedFuiButtonMsg(fui_btn_msg) => {
                self.skewed_fui_button.update(*fui_btn_msg)
            }
            Msg::SimpleSkewedFuiButtonMsg(fui_btn_msg) => {
                self.simple_skewed_fui_button.update(*fui_btn_msg)
            }
            Msg::GreenFuiButtonMsg(fui_btn_msg) => {
                self.green_fui_button.update(*fui_btn_msg)
            }
            Msg::ParagraphMsg(word_msg) => {
                log::trace!("animating paragraph..");
                self.paragraph.update(*word_msg)
            }
            Msg::ReanimateParagraph => {
                self.paragraph.update(paragraph::Msg::AnimateIn)
            }
            Msg::ReanimateAll => {
                log::debug!("Reanimating...");
                Self::reanimate_all()
            }
            Msg::NoOp => Cmd::none(),
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
