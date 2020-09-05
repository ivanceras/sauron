//#![deny(warnings)]
#![recursion_limit = "256"]
use animate_list::AnimateList;
use frame::Frame;
use fui_button::FuiButton;
use header::Header;
use paragraph::Paragraph;
use sauron::{
    html::{
        attributes::{
            class,
            style,
        },
        div,
        events::on_click,
        text,
    },
    prelude::*,
    Cmd,
    Component,
    Node,
    Program,
};
use spinner::Spinner;

mod animate_list;
mod frame;
mod fui_button;
mod header;
mod paragraph;
mod spinner;

#[derive(Clone, Debug)]
pub enum Msg {
    ReAnimateFrame,
    ReAnimateHeader,
    ReAnimateParagraph,
    ReAnimateList,
    FrameMsg(frame::Msg),
    HeaderMsg(header::Msg),
    ParagraphMsg(paragraph::Msg),
    FuiButtonMsg(Box<fui_button::Msg<Self>>),
    SimpleFuiButtonMsg(Box<fui_button::Msg<Self>>),
    SkewedFuiButtonMsg(Box<fui_button::Msg<Self>>),
    SimpleSkewedFuiButtonMsg(Box<fui_button::Msg<Self>>),
    GreenFuiButtonMsg(Box<fui_button::Msg<Self>>),
    DisabledFuiButtonMsg(Box<fui_button::Msg<Self>>),
    AnimateListMsg(Box<animate_list::Msg>),
    ReanimateAll,
    NoOp,
}

pub struct App {
    header: Header,
    frame: Frame,
    paragraph: Paragraph<Msg>,
    fui_button: FuiButton<Msg>,
    simple_fui_button: FuiButton<Msg>,
    skewed_fui_button: FuiButton<Msg>,
    simple_skewed_fui_button: FuiButton<Msg>,
    green_fui_button: FuiButton<Msg>,
    disabled_fui_button: FuiButton<Msg>,
    spinner: Spinner<Msg>,
    animate_list: AnimateList<Msg>,
}

impl App {
    pub fn new() -> Self {
        let mut fui_button = FuiButton::<Msg>::new_with_label("Reanimate All");
        fui_button.add_event_listeners(vec![on_click(|_| Msg::ReanimateAll)]);

        let mut simple_fui_button =
            FuiButton::<Msg>::new_with_label("Reanimate Paragraph");
        simple_fui_button.has_corners(false);
        simple_fui_button
            .add_event_listeners(vec![on_click(|_| Msg::ReAnimateParagraph)]);

        let mut skewed_fui_button =
            FuiButton::<Msg>::new_with_label("Skewed button");
        skewed_fui_button.skewed(true);
        skewed_fui_button.has_corners(true);

        let mut simple_skewed_fui_button =
            FuiButton::<Msg>::new_with_label("Skewed simple");
        simple_skewed_fui_button.skewed(true);
        simple_skewed_fui_button.has_corners(false);

        let mut green_fui_button = FuiButton::<Msg>::new_with_label("Sci-fi");
        green_fui_button.use_green(true);

        let mut disabled_fui_button =
            FuiButton::<Msg>::new_with_label("Disabled");
        disabled_fui_button.disabled(true);
        let paragraph_content = "This is an experimental demo showcasing usage of [Sauron](https://github.com/ivanceras/sauron)
                    Component lifecycle to work alongside
                    css transition, animation and timed DOM manipulation. This is also an exploration on how to add theming to the web framework.
                    Sauron is a light-weight web framework designed to have you write least amount of code possible.";

        App {
            frame: Frame::new_with_content("Retro Futuristic UI in rust"),
            header: Header::new_with_content("Header"),
            paragraph: Paragraph::new_with_markdown(paragraph_content),
            fui_button,
            simple_fui_button,
            skewed_fui_button,
            simple_skewed_fui_button,
            green_fui_button,
            disabled_fui_button,
            spinner: Spinner::new(),
            animate_list: AnimateList::new_with_content(
                Self::animate_list_content(),
            ),
        }
    }

    fn animate_list_content() -> Node<Msg> {
        let long_txt = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam scelerisque purus faucibus urna venenatis, a elementum diam laoreet. Fusce eget enim justo. Pellentesque cursus metus elit, ut porttitor eros iaculis sit amet. Quisque varius felis id turpis iaculis, et viverra enim pulvinar. Curabitur vel lacus interdum, molestie purus ut, pretium nibh. Mauris commodo dolor magna, eget dignissim mauris semper vitae. Ut viverra nec ex quis semper. Sed sit amet tincidunt mauris. Mauris in imperdiet ipsum. Praesent pretium tortor ut felis posuere, sed lacinia nunc pretium. Morbi et felis nec neque accumsan tincidunt. In hac habitasse platea dictumst. Nulla sit amet elit sed purus posuere placerat ut quis metus. Etiam mattis interdum dui at ornare. Nunc sit amet venenatis lorem, sed eleifend mauris. Pellentesque eros sem, fermentum vel lacus at, congue rhoncus elit. ";
        div(
            vec![],
            vec![
                button(vec![on_click(|_|Msg::ReAnimateFrame)], vec![text("Animate Frame")]),
                p(vec![], vec![
                    text("This is an experimental demo showcasing usage of sauron[0] Component lifecycle to work alongside
                    css transition, animation and timed DOM manipulation. This is also an exploration on how to add theming to the web framework.
                    Sauron is a light-weight web framework designed to have you write least amount of code possible."),
                    a(vec![href("https://github.com/ivanceras/sauron")], vec![text("Link here")]),
                    img(vec![styles([("width","600px"),("height", "auto"),("display","block")]),src("img/space.jpg")], vec![]),
                ]),
                li(vec![], vec![text(long_txt.clone())]),
                li(vec![], vec![text("List 2")]),
                ul(
                    vec![],
                    vec![
                        li(vec![], vec![text("SubList 3")]),
                        li(vec![], vec![text("Not too long txt here... trying to see if it is correctly animated")]),
                    ],
                ),
                div(vec![],vec![
                    table(vec![],vec![
                        thead(vec![],vec![
                            tr(vec![],vec![
                                th(vec![],vec![text("Prop name")]),
                                th(vec![],vec![text("Type")]),
                                th(vec![],vec![text("Default")]),
                                th(vec![],vec![text("Description")]),
                            ]),
                        ]),
                        tbody(vec![],vec![
                            tr(vec![],vec![
                                td(vec![],vec![text("name")]),
                                td(vec![],vec![text("string")]),
                                td(vec![],vec![text("''")]),
                                td(vec![],vec![text("The base name of the component")]),
                            ]),
                            tr(vec![],vec![
                                td(vec![],vec![text("age")]),
                                td(vec![],vec![text("number")]),
                                td(vec![],vec![text("0")]),
                                td(vec![],vec![text("The age of the component")]),
                            ]),
                            tr(vec![],vec![
                                td(vec![],vec![text("married")]),
                                td(vec![],vec![text("bool")]),
                                td(vec![],vec![text("false")]),
                                td(vec![],vec![text("If the component is married")]),
                            ]),
                        ]),
                    ]),
                ])
            ],
        )
    }

    fn reanimate_all() -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            program.dispatch(Msg::ReAnimateFrame);
            program.dispatch(Msg::ReAnimateHeader);
            program.dispatch(Msg::ReAnimateParagraph);
            program.dispatch(Msg::ReAnimateList);
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
        .chain(self.animate_list.style().into_iter())
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
                            self.disabled_fui_button.view().map_msg(|fbtn_msg| {
                                Msg::DisabledFuiButtonMsg(Box::new(fbtn_msg))
                            }),
                            self.skewed_fui_button.view().map_msg(|fbtn_msg| {
                                Msg::SkewedFuiButtonMsg(Box::new(fbtn_msg))
                            }),
                            self.simple_skewed_fui_button.view().map_msg(|fbtn_msg| {
                                Msg::SimpleSkewedFuiButtonMsg(Box::new(fbtn_msg))
                            }),
                        ]),
                        //self.paragraph.view(),
                        button(
                            vec![
                                on_click(|_| Msg::ReAnimateList),
                                style("margin", "20px"),
                                style("display", "block"),
                            ],
                            vec![text("Animate List")],
                        ),
                        p(
                            vec![],
                            vec![self.animate_list.view()],
                        ),
                        button(
                            vec![
                                on_click(|_| Msg::ReAnimateList),
                                style("margin", "20px"),
                                style("display", "block"),
                            ],
                            vec![text("Animate List")],
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
            Msg::ReAnimateHeader => {
                if let Some(header_msg) =
                    self.header.update(header::Msg::AnimateIn)
                {
                    Cmd::new(move |program| {
                        program.dispatch(Msg::HeaderMsg(header_msg.clone()))
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::HeaderMsg(header_msg) => {
                if let Some(header_msg) = self.header.update(header_msg) {
                    Cmd::new(move |program| {
                        program.dispatch(Msg::HeaderMsg(header_msg.clone()))
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::ReAnimateFrame => {
                if let Some(frame_msg) =
                    self.frame.update(frame::Msg::AnimateIn)
                {
                    Cmd::new(move |program| {
                        program.dispatch(Msg::FrameMsg(frame_msg.clone()))
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::FrameMsg(frame_msg) => {
                if let Some(frame_msg) = self.frame.update(frame_msg) {
                    Cmd::new(move |program| {
                        program.dispatch(Msg::FrameMsg(frame_msg.clone()))
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::FuiButtonMsg(fui_btn_msg) => {
                if let Some(pmsg) = self.fui_button.update(*fui_btn_msg) {
                    Cmd::new(move |program| program.dispatch(pmsg.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::SimpleFuiButtonMsg(fui_btn_msg) => {
                if let Some(pmsg) = self.simple_fui_button.update(*fui_btn_msg)
                {
                    Cmd::new(move |program| program.dispatch(pmsg.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::SkewedFuiButtonMsg(fui_btn_msg) => {
                if let Some(pmsg) = self.skewed_fui_button.update(*fui_btn_msg)
                {
                    Cmd::new(move |program| program.dispatch(pmsg.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::SimpleSkewedFuiButtonMsg(fui_btn_msg) => {
                if let Some(pmsg) =
                    self.simple_skewed_fui_button.update(*fui_btn_msg)
                {
                    Cmd::new(move |program| program.dispatch(pmsg.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::GreenFuiButtonMsg(fui_btn_msg) => {
                if let Some(pmsg) = self.green_fui_button.update(*fui_btn_msg) {
                    Cmd::new(move |program| program.dispatch(pmsg.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::DisabledFuiButtonMsg(fui_btn_msg) => {
                if let Some(pmsg) =
                    self.disabled_fui_button.update(*fui_btn_msg)
                {
                    Cmd::new(move |program| program.dispatch(pmsg.clone()))
                } else {
                    Cmd::none()
                }
            }
            Msg::AnimateListMsg(animate_list_msg) => {
                if let Some(animate_list_msg) =
                    self.animate_list.update(*animate_list_msg)
                {
                    Cmd::new(move |program| {
                        program.dispatch(Msg::AnimateListMsg(Box::new(
                            animate_list_msg.clone(),
                        )))
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::ReAnimateList => {
                if let Some(animate_list_msg) =
                    self.animate_list.update(animate_list::Msg::AnimateIn)
                {
                    Cmd::new(move |program| {
                        program.dispatch(Msg::AnimateListMsg(Box::new(
                            animate_list_msg.clone(),
                        )))
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::ParagraphMsg(para_msg) => {
                if let Some(para_msg) = self.paragraph.update(para_msg) {
                    Cmd::new(move |program| {
                        program.dispatch(Msg::ParagraphMsg(para_msg.clone()));
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::ReAnimateParagraph => {
                if let Some(para_msg) =
                    self.paragraph.update(paragraph::Msg::AnimateIn)
                {
                    Cmd::new(move |program| {
                        program.dispatch(Msg::ParagraphMsg(para_msg.clone()));
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::ReanimateAll => Self::reanimate_all(),
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
