#![deny(warnings)]
#![deny(clippy::all)]
use log::trace;
use sauron::{
    dom::{delay, TimeoutCallbackHandle},
    html::attributes::*,
    html::events::*,
    html::*,
    jss, text, wasm_bindgen, Application, Cmd, Node, Program,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use wasm_bindgen_futures::spawn_local;
use futures::channel::mpsc;

pub enum Msg {
    Click,
    NoOp,
    CancelPrevious,
}

#[derive(Default)]
pub struct App {
    current_handle: Rc<RefCell<Option<TimeoutCallbackHandle>>>,
    executed: Rc<AtomicBool>,
}

impl Application for App {
    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        sauron::html::main(
            [],
            [
                h2([], [text("Delay example")]),
                h4(
                    [],
                    [text!(
                        "Is executed: {}",
                        self.executed.load(Ordering::Relaxed)
                    )],
                ),
                div(
                    [],
                    [
                        input(
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
                        button(
                            [on_click(|_| Msg::CancelPrevious)],
                            [text("Cancel previous")],
                        ),
                        button([on_click(|_| Msg::NoOp)], [text("Noping..")]),
                    ],
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Click => {
                spawn_local(some_async_function());
                Cmd::none()
            }
            Msg::CancelPrevious => {
                let current_handle = Rc::clone(&self.current_handle);
                let executed = Rc::clone(&self.executed);
                //Cmd::new(|program| Self::execute_delayed(program, current_handle, executed))
                log::info!("in execute delayed...");
                if let Some(current_handle) = current_handle.borrow_mut().take() {
                    log::info!("We cancelled {:?}", current_handle);
                    drop(current_handle);
                }
                let (mut tx, rx) = mpsc::unbounded();
                let handle = sauron::dom::request_timeout_callback(
                    move || {
                        log::info!("I'm executing after 5 seconds");
                        executed.store(true, Ordering::Relaxed);
                        tx.start_send(Msg::NoOp).unwrap();
                    },
                    5000,
                )
                .expect("must have a handle");

                *current_handle.borrow_mut() = Some(handle);
                Cmd::sub(rx, sauron::Closure::new(|_:sauron::web_sys::Event|{
                    panic!("This is not called!");
                }))
            }
            Msg::NoOp => Cmd::none(),
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

async fn some_async_function() {
    let t1 = sauron::now();
    log::debug!("t1: {}", t1);
    delay(1000).await;
    let t2 = sauron::now();
    log::debug!("t2: {}", t2);
    log::debug!("elapsed: {}", t2 - t1);
    delay(5000).await;
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
