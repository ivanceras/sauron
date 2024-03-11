#![deny(warnings)]
use js_sys::Date;
use sauron::prelude::*;

pub enum Msg {
    Click,
    DoubleClick,
    Clock,
    ChangeName(String),
    ChangeBiography(String),
    ChangeThought(String),
}

pub struct App {
    click_count: u32,
    double_clicks: u32,
    date: Date,
    name: String,
    biography: String,
    thought: Option<String>,
}

impl App {
    pub fn new(click_count: u32) -> App {
        App {
            click_count,
            double_clicks: 0,
            date: Date::new_0(),
            name: String::new(),
            biography: String::new(),
            thought: None,
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::new(|mut program| {
            let program2 = program.clone();
            let clock: Closure<dyn FnMut()> = Closure::new(move || {
                program.dispatch(Msg::Clock);
            });
            window()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    clock.as_ref().unchecked_ref(),
                    1000,
                )
                .expect("Unable to start interval");
            program2.closures.borrow_mut().push(clock);
        })
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => {
                self.click_count += 1;
                log::info!("click count: {}", self.click_count);
            }
            Msg::DoubleClick => {
                self.double_clicks += 1;
            }
            Msg::Clock => {
                self.date = Date::new_0();
            }
            Msg::ChangeName(name) => {
                self.name = name;
            }
            Msg::ChangeBiography(bio) => {
                self.biography = bio;
            }
            Msg::ChangeThought(thought) => {
                if !thought.is_empty() {
                    self.thought = Some(thought);
                } else {
                    self.thought = None;
                }
            }
        }
        Cmd::none()
    }


    fn view(&self) -> Node<Msg> {
        node! {
            <div class="some-class" id="some-id" {attr("data-id", 1)} {style!{"font-family": "monospace"}}>
                <div id="current-time">{text!("Today is {}",self.date.to_locale_string("en-GB", &JsValue::undefined()))}</div>
                <div>
                        "Your name is: "
                        <input type="text"
                                on_input=|event: InputEvent| Msg::ChangeName(event.value())
                                placeholder="John Smith"
                                data-attr=format!("Hello{}", &self.name)
                        />
                        <button on_click=|event: MouseEvent| {
                                trace!("Clicked at ({},{})", event.x(), event.y());
                                Msg::Click}>
                            Click me!
                        </button>
                        <button on_dblclick=|event: MouseEvent| {
                                trace!("Double clicked at ({},{})", event.x(), event.y());
                                Msg::DoubleClick}>
                            {text!("DoubleClicks {}", self.double_clicks)}
                        </button>
                </div>
                <p>
                    {text!("Hello {}!", self.name)}
                    {if self.click_count > 0 {
                        text!(
                            ", You've clicked on that button for {} time{}",
                            self.click_count,
                            if self.click_count > 1 { "s" } else { "" }
                        )
                    } else {
                        text("here..")
                    }}
                </p>
                <div>
                        <p>Tell us something about yourself</p>
                        <div>
                            <textarea rows=10 cols=80
                                    on_input=|event: InputEvent| {
                                        Msg::ChangeBiography(event.value())
                                    }
                                    placeholder="I'm a..."
                            />
                        </div>
                        <p>{text!("{}", self.biography)}</p>
                </div>
                <div>
                        "What are you thinking right now?"
                        <input type="text"
                                on_change=|event: InputEvent| Msg::ChangeThought(event.value())
                                placeholder="Elephants..."
                        />
                        {if let Some(thought) = &self.thought {
                            text(format!("Hmmn {}... Interesting.", thought))
                        } else {
                            node!{<span></span>}
                        }}
                </div>
            </div>
        }
    }


    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }]
    }
}
