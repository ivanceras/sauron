// svg_clock example ported from:
// https://github.com/utkarshkukreti/draco/tree/master/examples/svg_clock

#![deny(warnings)]
#![deny(clippy::all)]
use console_error_panic_hook;
use js_sys::Date;
use sauron::{html::attributes::style, prelude::*, wasm_bindgen::JsCast};

#[macro_use]
extern crate log;

pub enum Msg {
    Tick,
}

pub struct Clock {
    date: Date,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            date: Date::new_0(),
        }
    }
}

impl Component<Msg> for Clock {
    // we wire the window set_interval api to trigger an Msg::Tick
    // by dispatching it from the program, through the Cmd interface
    fn init(&self) -> Cmd<Self, Msg> {
        Cmd::new(move |program| {
            let clock: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
                program.dispatch(Msg::Tick);
            }));

            web_sys::window()
                .expect("no global `window` exists")
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    clock.as_ref().unchecked_ref(),
                    30,
                )
                .expect("Unable to start interval");
            clock.forget();
        })
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Tick => {
                self.date = Date::new_0();
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        let circle = circle(
            vec![cx(100), cy(100), r(98), fill("none"), stroke("#1a202c")],
            vec![],
        );

        let hand = |rotate: f64,
                    stroke_color: &'static str,
                    stroke_width_value: u32,
                    height: u32| {
            line(
                vec![
                    x1(100),
                    y1(100),
                    x2(100 - height),
                    y2(100),
                    stroke(stroke_color),
                    stroke_width(stroke_width_value),
                    stroke_linecap("round"),
                    transform(format!(
                        "rotate({} 100 100)",
                        (rotate * 10.0).round() / 10.0
                    )),
                ],
                vec![],
            )
        };

        let d = &self.date;
        let ms = ((((d.get_hours() * 60 + d.get_minutes()) * 60)
            + d.get_seconds())
            * 1000
            + d.get_milliseconds()) as f64;

        let subsecond_rotate = 90.0 + ((ms / 1000.0) % 1.0) * 360.0;
        let second_rotate = 90.0 + ((ms / 1000.0) % 60.0) * 360.0 / 60.0;
        let minute_rotate = 90.0 + ((ms / 1000.0 / 60.0) % 60.0) * 360.0 / 60.0;
        let hour_rotate =
            90.0 + ((ms / 1000.0 / 60.0 / 60.0) % 12.0) * 360.0 / 12.0;

        article(
            vec![],
            vec![
                h2(
                    vec![],
                    vec![text("Sauron clock demonstrating svg dom manipulation")]
                ),
                a(
                    vec![href(
                        "https://github.com/ivanceras/sauron/tree/master/examples/svg-clock"
                    )],
                    vec![text("code")]
                ),
                div(
                    vec![style("display","flex"),
                     style("align-items", "center"),
                     style("flex-direction", "column"),
                    ],
                    vec![svg(
                        vec![width(400), height(400), viewBox([0, 0, 200, 200])],
                        vec![
                            circle,
                            hand(subsecond_rotate, "#e2e8f0", 10, 90),
                            hand(hour_rotate, "#2d3748", 4, 50),
                            hand(minute_rotate, "#2d3748", 3, 70),
                            hand(second_rotate, "#e53e3e", 2, 90),
                        ]
                    )]
                )
            ]
        )
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    trace!("starting svg clock..");

    Program::mount_to_body(Clock::new());
}
