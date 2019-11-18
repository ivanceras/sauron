//#![deny(warnings)]
use sauron::{
    html::{
        attributes,
        attributes::{
            height,
            id,
            width,
        },
    },
    svg::{
        attributes::*,
        *,
    },
    Cmd,
    Component,
    Node,
    Program,
    *,
};
use wasm_bindgen::prelude::*;

extern crate log;

pub enum Msg {
    Click,
}

pub struct App {
    click_count: u32,
}

impl App {
    pub fn new() -> Self {
        App { click_count: 0 }
    }
}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        svg!([viewBox([0, 0, 800, 460]) ],[
            desc!([],[
                text!("Star Trek vs. Star Wars - Book mentions via Google NGRAM. Source: https://books.google.com/ngrams/graph?content=Star+Wars%2C+Star+Trek&year_start=1960&year_end=2008&corpus=15&smoothing=3&share=&direct_url=t1%3B%2CStar%20Wars%3B%2Cc0%3B.t1%3B%2CStar%20Trek%3B%2Cc0")
            ]),
            defs!([],[
                radialGradient!([id("gradient-1")],[
                    stop!([attributes::style("stop-color: rgb(99, 84, 84);")],[

                    ]),
                    stop!([attributes::style("stop-color: rgb(19, 19, 19);")],[

                    ])
                ]),
                pattern!([id("pattern-2"), width("100"), height("100")],[
                    rect!([width("6.9"), height("180.6"), attributes::style("fill: rgb(216, 216, 216);")],[

                    ]),
                    rect!([width("6.9"), height("180.6"), attributes::style("fill: rgb(216, 216, 216); stroke-width: 1;")],[

                    ])
                ]),
               svg::tags::style(vec![id("bx-google-fonts")],vec![
                    text!("@import url(https://fonts.googleapis.com/css?family=Roboto:100,100italic,300,300italic,400,400italic,500,500italic,700,700italic,900,900italic);")
                ]),
                pattern!([id("pattern-3"), href("#pattern-2")],[

                ])
            ]),
            rect!([width("800"), height("460"), attributes::style("fill: url(#gradient-1);")],[

            ]),
            rect!([width("690"), height("360"), attributes::style("fill: url(#pattern-3); fill-opacity: 0.2; stroke: rgb(105, 105, 104);")],[

            ]),
           svg::tags::text(vec![attributes::style("font-size: 16px; font-family: Roboto; fill: rgb(251, 251, 251); word-spacing: 0px;")],vec![
                text!("Star Trek vs. Star Wars - Book mentions via Google NGRAM.")
            ]),
  path!([d("M 84.2 416.8 L 98.8 416.8 L 113.3 416.8 L 127.8 416.8 L 142.3 416.8 L 156.8 416.8 L 171.3 416.8 L 185.8 416.7 L 200.3 416.7 L 214.9 416.6 L 229.4 416.6 L 243.9 416.5 L 258.4 416.3 L 272.9 416 L 287.4 413.5 L 301.9 408.3 L 316.4 399.1 L 330.9 387.3 L 345.4 375.6 L 359.9 362 L 374.4 334.7 L 389 316.7 L 403.5 287.2 L 418 244.2 L 432.5 197.5 L 447 158.1 L 461.5 133.3 L 476 127.1 L 489.9 121.1 L 505.1 127 L 519.6 158.1 L 534.1 196.9 L 548.6 225.8 L 563.1 241.5 L 577.6 246.8 L 592.1 245.3 L 606.6 227.2 L 621.2 209.3 L 635.7 197.6 L 650.2 182.9 L 664.7 172.8 L 679.2 167.1 L 693.7 162.8 L 708.2 172.7 L 722.7 180.7 L 737.3 187 L 751.8 191.9 L 766.3 193 L 780.8 197.1"), attributes::style("stroke: rgb(253, 200, 39); stroke-width: 3; fill: none;"), attr("bx:origin","0.5 0.5")], []),
  path!([d("M 84 417.3 L 98.6 417.3 L 113 417.3 L 127.5 417.2 L 141.9 417.2 L 156.5 413.8 L 170.9 413.2 L 185.4 412.4 L 199.9 411.5 L 214.4 410.6 L 228.8 409.6 L 243.3 408.2 L 257.8 403.9 L 272.3 398.9 L 286.7 385.7 L 301.2 381.8 L 315.7 376.7 L 330.2 365.2 L 344.6 359 L 359.1 359.9 L 373.6 354 L 388.1 355.9 L 402.5 348 L 417 339 L 431.5 339.8 L 446 337.6 L 460.4 332.3 L 474.9 328.3 L 489.4 315.4 L 503.9 303.2 L 518.3 301 L 532.8 277.8 L 547.3 240.6 L 561.8 209.7 L 576.2 194.3 L 590.7 182.6 L 605.2 154.5 L 619.7 134.5 L 634.1 138.4 L 642.1 145.3 L 648.6 150.9 L 663.1 160.2 L 677.6 162.3 L 692 169.5 L 706.5 193.1 L 721 201.7 L 735.5 212.6 L 749.9 218.9 L 764.4 226.1 L 778.9 228.9"), attributes::style("stroke: rgb(33, 125, 245); fill: none; stroke-width: 3;"), attr("bx:origin","0.5 0.5")],[]),

            svg::tags::text(vec![attributes::style("white-space: pre; font-size: 15px; fill: rgb(33, 125, 245); word-spacing: 0px;")],vec![
                text!("Star Trek")
            ]),
            svg::tags::text(vec![attributes::style("white-space: pre; font-size: 15px; fill: rgb(253, 200, 39); word-spacing: 0px;")],vec![
                text!("Star Wars")
            ]),
            g!([class("y-axis")],[
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.00000%")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.00020%")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.00040%")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.00060%")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.00080%")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.000100%")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.000120%")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.000140%")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("0.000160%")
                ])
            ]),
            g!([class("x-axis")],[
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("1960")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("1965")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("1970")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("1975")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("1980")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("1985")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("1990")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("1995")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("2000")
                ]),
                svg::tags::text(vec![attributes::style("text-anchor: middle; fill: rgb(103, 102, 102); font-size: 12px;")],vec![
                    text!("2005")
                ])
            ])
        ])
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => self.click_count += 1,
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    Program::mount_to_body(App::new());
}
