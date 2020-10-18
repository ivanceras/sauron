// code is derived from: https://codepen.io/AsyrafHussin/pen/odVpmE?__cf_chl_jschl_tk__=b2a5d3c92297b4243deded6d24dd8727e6c0c718-1603031934-0-AXZNUeXia5wQorUjr2SzbbdAE1gNkxgtLS5ElYnE1U7Azaw8xRz0_XjMROZjlIO-pZ4HFU450aebEQvXqEIl8PNYwSH4-Ux7Wpe97dBjRl72Xftyxs8Gs0SecTOw-W87lLArzoxmu_1gGTAd02U9UolrKRXusUoaDNPp1Ue8Cchh5vhO9ayJLPkCOcy1ReT92tizsFyKDP1gKqf5V1k0ZvoYEshzezvF_Cie4qyW154U8bu40DSBjnaf734gykrvq7Ot52EPsejJnCR1w-CxEci2NHHWtR6OPgCYXEQdHk56poUkfCzo6Ml2l6jzGawh1ln-Gkb8JBCabcMh0qz0LvvRdvqvzcsCHwmbMzkCbMpe
#![deny(warnings)]
use sauron::{
    node,
    prelude::*,
    Cmd,
    Component,
    Node,
    Program,
};

pub enum Msg {
    Click,
}

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }
}

impl Component<Msg> for App {
    fn style(&self) -> Vec<String> {
        let css = r#"
            body {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                background: #0c0e0e;
                margin-left: -3%;
                overflow: hidden;
            }

            .reactor {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 60px;
                height: 60px;
                border-radius: 50%;
                background: radial-gradient(circle, #002d34 30%, #0089a9 60%, #00a6ce);
                border: 1px solid rgba(0, 45, 52, 0.3);
                filter: drop-shadow(0 0 1.8px #46fffe);
            }

            @keyframes spin {
              0% {
                transform: rotate(0deg);
              }
              100% {
                transform: rotate(360deg);
              }
            }

            .reactor .trapezoid-container {
                position: relative;
                width: 100%;
                height: 100%;
                animation: spin 5s infinite linear;
            }

            .reactor .trapezoid-container .trapezoid {
                box-sizing: content-box;
                width: 5px;
                height: 0;
                position: absolute;
                top: -4.5px;
                left: 20.5px;
                transform-origin: 10px 35px;
                border: 5px solid transparent;
                border-bottom: 0 solid;
                border-top: 10px solid #46fffe;
                border-radius: 4px;
                filter: drop-shadow(0 0 1.8px #46fffe);
            }

            .reactor .trapezoid-container .trapezoid:after {
                content: "";
                position: absolute;
                width: 8px;
                height: 8px;
                border-radius: 50%;
                background: radial-gradient(circle, #e4ffff 30%, #1ffffd);
                margin: 0;
                bottom: 1.5px;
                right: -1.5px;
            }

            .reactor .small-circle-container {
                position: relative;
                width: 100%;
                height: 100%;
            }

            .reactor .small-circle-container .small-circle {
                position: absolute;
                width: 4px;
                height: 4px;
                top: -50px;
                left: 21px;
                transform-origin: 9px 20.5px;
                border-radius: 50%;
                background: radial-gradient(circle, #eeffff 20%, #21fffe, #009fc1);
            }

            .reactor .circle-outer {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 30.5px;
                height: 30.5px;
                border: 1px solid #52fefe;
                background-color: #ffffff;
                border-radius: 50%;
                box-shadow: 0px 0px 2px 1px #52fefe, 0px 0px 2px 0.8px #52fefe inset;
            }

            .reactor .circle-center {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 22px;
                height: 22px;
                background-color: #0c4d5b;
                border-radius: 50%;
                box-shadow: 0px 0px 2px 1px #52fefe, 0px 0px 2px 0.8px #52fefe;
            }

            .reactor .circle-innner {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 19px;
                height: 19px;
                border: 1px solid #52fefe;
                background-color: #ffffff;
                border-radius: 50%;
                box-shadow: 0px 0px 2px 1px #52fefe, 0px 0px 4.5px 1.5px #52fefe inset;
            }"#;
        vec![css.to_string()]
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
              <div class="circle-innner"></div>
            </div>
        }
    }

    fn update(&mut self, _msg: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
