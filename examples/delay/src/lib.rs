#![deny(warnings)]
#![deny(clippy::all)]
use log::trace;
use sauron::{
    html::{
        attributes::{
            class,
            r#type,
            value,
        },
        div,
        events::on_click,
        h2,h4,
        input,
        text,
    },
    jss,
    prelude::*,
    Application,
    Cmd,
    Node,
    Program,
};
use wasm_bindgen_futures::spawn_local;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::rc::Rc;

pub enum Msg {
    Click,
    NoOp,
    CancelPrevious,
}

#[derive(Default)]
pub struct App {
    current_handle: Option<i32>,
    executed: Rc<AtomicBool>,
}

impl App{
    fn execute_delayed(&mut self){
        log::info!("in execute delayed...");
        if let Some(current_handle) = self.current_handle{
            sauron::dom::clear_timeout_with_handle(current_handle);
            log::info!("We cancelled {}", current_handle);
        }

        log::info!("We are scheduling a new one..");
        let executed = self.executed.clone();
        let handle = sauron::dom::delay_exec(move||{
            log::info!("I'm executing after 5 seconds");
            executed.store(true, Ordering::Relaxed);
        }, 5000).expect("must have a handle");
        self.current_handle = Some(handle);
    }
}

#[async_trait(?Send)]
impl Application<Msg> for App {
    fn view(&self) -> Node<Msg> {
        sauron::html::main(
            [],
            [
                h2([], [text("Delay example")]),
                h4([], [text!("Is executed: {}", self.executed.load(Ordering::Relaxed))]),
                div(
                    [],
                    [input(
                        [
                            class("client"),
                            r#type("button"),
                            value("Click me!"),
                            on_click(|_| {
                                trace!("Button is clicked");
                                Msg::Click
                            }),
                        ],
                        [],
                    ),
                    button([
                        on_click(|_|{Msg::CancelPrevious})
                        ], 
                        [text("Cancel previous")]
                    ),
                    button([
                        on_click(|_|{Msg::NoOp})
                        ], 
                        [text("Noping..")]
                    ),
                    ],
                ),
            ],
        )
    }

    async fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => spawn_local(some_async_function()),
            Msg::CancelPrevious => self.execute_delayed(),
            Msg::NoOp => (),
        }
        Cmd::none()
    }

    fn style(&self) -> String {
        jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }
    }
}


async fn some_async_function() {
    let t1 = sauron::now();
    log::debug!("t1: {}", t1);
    async_delay(1000).await;
    let t2 = sauron::now();
    log::debug!("t2: {}", t2);
    log::debug!("elapsed: {}", t2 - t1);
    async_delay(5000).await;
    let t3 = sauron::now();
    log::debug!("t3: {}", t3);
    log::debug!("elapsed: {}", t3 - t2);
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}
