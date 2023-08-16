// svg_clock example ported from:
// https://github.com/utkarshkukreti/draco/tree/master/examples/svg_clock

#![deny(warnings)]
#![deny(clippy::all)]

use js_sys::Date;
use sauron::{html::units::percent, jss, wasm_bindgen::JsCast, *};

#[macro_use]
extern crate log;

pub enum Msg {
    Tick,
}

pub struct Clock {
    date: Date,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            date: Date::new_0(),
        }
    }
}

impl Application<Msg> for Clock {
    // we wire the window set_interval api to trigger an Msg::Tick
    // by dispatching it from the program, through the Cmd interface
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::new(move |mut program| {
            let clock: Closure<dyn FnMut()> = Closure::new(move || {
                program.dispatch(Msg::Tick);
            });

            web_sys::window()
                .expect("no global `window` exists")
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    clock.as_ref().unchecked_ref(),
                    1_000 / 60, // try to Tick at 60fps
                )
                .expect("Unable to start interval");
            clock.forget();
        })
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Tick => {
                info!("Tick Tock");
                self.date = Date::new_0();
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        let circle = node! {
            <circle cx="100" cy="100" r="98" fill="none" stroke="#1a202c"/>
        };

        let hand =
            |rotate: f64, stroke_color: &'static str, stroke_width_value: u32, height: u32| {
                node! {
                    <line
                            x1="100"
                            y1="100"
                            x2=100 - height
                            y2="100"
                            stroke=stroke_color
                            stroke-width=stroke_width_value
                            stroke-linecap="round"
                            transform=format!(
                                "rotate({} 100 100)",
                                (rotate * 10.0).round() / 10.0
                            )
                    />
                }
            };

        let d = &self.date;
        let ms = ((((d.get_hours() * 60 + d.get_minutes()) * 60) + d.get_seconds()) * 1000
            + d.get_milliseconds()) as f64;

        let subsecond_rotate = 90.0 + ((ms / 1000.0) % 1.0) * 360.0;
        let second_rotate = 90.0 + ((ms / 1000.0) % 60.0) * 360.0 / 60.0;
        let minute_rotate = 90.0 + ((ms / 1000.0 / 60.0) % 60.0) * 360.0 / 60.0;
        let hour_rotate = 90.0 + ((ms / 1000.0 / 60.0 / 60.0) % 12.0) * 360.0 / 12.0;

        node! {
            <article>
                <h2>Sauron clock demonstrating svg dom manipulation</h2>
                <a href="https://github.com/ivanceras/sauron/tree/master/examples/svg-clock">
                    code
                </a>
                <div style="display:flex; align-items:center; flex-direction: column">
                    <svg width="400" height="400" view_box="0 0 200 200">
                        { circle }
                        { hand(subsecond_rotate, "#e2e8f0", 10, 90) }
                        { hand(hour_rotate, "#2d3748", 4, 50) }
                        { hand(minute_rotate, "#2d3748", 3, 70) }
                        { hand(second_rotate, "#e53e3e", 2, 90) }
                    </svg>
                </div>
            </article>
        }
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
                margin: 0,
                padding: 0,
                width: percent(100),
                height: percent(100),
            },

            "article": {
                display: "flex",
                flex_direction: "column",
                align_items: "center",
                justify_content: "center",
            }
        }]
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    trace!("starting svg clock..");

    Program::mount_to_body(Clock::default());
}
