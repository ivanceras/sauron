// svg_clock example ported from:
// https://github.com/utkarshkukreti/draco/tree/master/examples/svg_clock
use js_sys::Date;
use sauron::{
    html::attributes::*,
    svg::attributes::*,
    Cmd,
    Node,
    *,
};

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
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Tick => {
                self.date = Date::new_0();
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        let circle = circle!(
            [cx(100), cy(100), r(98), fill("none"), stroke("#1a202c")],
            []
        );

        let line = |rotate: f64,
                    stroke_color: &'static str,
                    stroke_width_value: u32,
                    height: u32| {
            svg::tags::line(
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

        div!(
            [style(
                "display: flex; align-items: center; flex-direction: column;"
            )],
            [svg!(
                [width(400), height(400), viewBox([0, 0, 200, 200])],
                [
                    circle,
                    line(subsecond_rotate, "#e2e8f0", 10, 90),
                    line(hour_rotate, "#2d3748", 4, 50),
                    line(minute_rotate, "#2d3748", 3, 70),
                    line(second_rotate, "#e53e3e", 2, 90),
                ]
            )]
        )
    }
}
