// code is derived from: https://codepen.io/AsyrafHussin/pen/odVpmE?__cf_chl_jschl_tk__=b2a5d3c92297b4243deded6d24dd8727e6c0c718-1603031934-0-AXZNUeXia5wQorUjr2SzbbdAE1gNkxgtLS5ElYnE1U7Azaw8xRz0_XjMROZjlIO-pZ4HFU450aebEQvXqEIl8PNYwSH4-Ux7Wpe97dBjRl72Xftyxs8Gs0SecTOw-W87lLArzoxmu_1gGTAd02U9UolrKRXusUoaDNPp1Ue8Cchh5vhO9ayJLPkCOcy1ReT92tizsFyKDP1gKqf5V1k0ZvoYEshzezvF_Cie4qyW154U8bu40DSBjnaf734gykrvq7Ot52EPsejJnCR1w-CxEci2NHHWtR6OPgCYXEQdHk56poUkfCzo6Ml2l6jzGawh1ln-Gkb8JBCabcMh0qz0LvvRdvqvzcsCHwmbMzkCbMpe
#![deny(warnings)]
use sauron::jss;
use sauron::{node, prelude::*, Application, Cmd, Node, Program};

pub enum Msg {
    Click,
}

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }
}

impl Application<Msg> for App {
    fn update(&mut self, _msg: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        let trapezoid_count = 8;
        let small_circle_count = 10;
        node! {
            <div class="reactor">
              <div class="trapezoid-container">
                {for i in 0..trapezoid_count{
                    node!{
                        <div class=format!("trapezoid trapezoid-{}",i+1)
                             style=format!("transform:rotate({}deg)", i * 360 / trapezoid_count)>
                        </div>
                    }
                }}
              </div>

              <div class="small-circle-container">
                {for i in 0..small_circle_count{
                    node!{
                        <div class=format!("small-circle small-circle-{}",i+1)
                             style=format!("transform:rotate({}deg)", i * 360 / small_circle_count)>
                        </div>
                    }
                }}
              </div>
              <div class="circle-outer"></div>
              <div class="circle-center"></div>
              <a href="https://github.com/ivanceras/sauron/tree/master/examples/arc-reactor/src/lib.rs"
                 alt="view code"
                 title="view code">
                <div class="circle-innner"></div>
              </a>
            </div>
        }
    }

    fn style(&self) -> String {
        jss! {
            "body": {
                position: "absolute",
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
                margin: "auto",
                background: "#0c0e0e",
                margin_left: percent(-3),
                overflow: "hidden",
            },

            ".reactor": {
                position: "absolute",
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
                margin: "auto",
                width: px(120),
                height: px(120),
                border_radius: percent(50),
                background: "radial-gradient(circle, #002d34 30%, #0089a9 60%, #00a6ce)",
                border: "2px solid rgba(0, 45, 52, 0.3)",
                filter: "drop-shadow(0 0 3.6px #46fffe)",
            },

            "@keyframes spin": {
              "0%": {
                transform: "rotate(0deg)",
              },
              "100%": {
                transform: "rotate(360deg)",
              },
            },

            ".reactor .trapezoid-container": {
                position: "relative",
                width: percent(100),
                height: percent(100),
                animation: "spin 5s infinite linear",
            },

            ".reactor .trapezoid-container .trapezoid": {
                box_sizing: "content-box",
                width: px(10),
                height: 0,
                position: "absolute",
                top: px(-9),
                left: px(41),
                transform_origin: px([20, 70]),
                border: "10px solid transparent",
                border_bottom: "0 solid",
                border_top: "20px solid #46fffe",
                border_radius: px(8),
                filter: "drop-shadow(0 0 3.6px #46fffe)",
            },

            ".reactor .trapezoid-container .trapezoid:after": {
                content: "\"\"",
                position: "absolute",
                width: px(16),
                height: px(16),
                border_radius: px(50),
                background: "radial-gradient(circle, #e4ffff 30%, #1ffffd)",
                margin: 0,
                bottom: px(3),
                right: px(-3),
            },

            ".reactor .small-circle-container": {
                position: "relative",
                width: percent(100),
                height: percent(100),
            },

            ".reactor .small-circle-container .small-circle": {
                position: "absolute",
                width: px(8),
                height: px(8),
                top: px(-100),
                left: px(42),
                transform_origin: px([18, 41]),
                border_radius: percent(50),
                background: "radial-gradient(circle, #eeffff 20%, #21fffe, #009fc1)",
            },

            ".reactor .circle-outer": {
                position: "absolute",
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
                margin: "auto",
                width: px(61),
                height: px(61),
                border: "2px solid #52fefe",
                background_color: "#ffffff",
                border_radius: percent(50),
                box_shadow: "0 0 4px 2px #52fefe, 0 0 4px 1.6px #52fefe inset",
            },

            ".reactor .circle-center": {
                position: "absolute",
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
                margin: "auto",
                width: px(44),
                height: px(44),
                background_color: "#0c4d5b",
                border_radius: percent(50),
                box_shadow: "0 0 4px 2px #52fefe, 0 0 4px 1.6px #52fefe",
            },

            ".reactor .circle-innner": {
                position: "absolute",
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
                margin: "auto",
                width: px(38),
                height: px(38),
                border: "2px solid #52fefe",
                background_color: "#ffffff",
                border_radius: percent(50),
                box_shadow: "0 0 4px 2px #52fefe, 0 0 9px 3px #52fefe inset",
            },

            ".reactor .circle-innner:hover": {
               background_color: "#ff0000",
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
