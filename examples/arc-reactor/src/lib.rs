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
                width: 120px;
                height: 120px;
                border-radius: 50%;
                background: radial-gradient(circle, #002d34 30%, #0089a9 60%, #00a6ce);
                border: 2px solid rgba(0, 45, 52, 0.3);
                filter: drop-shadow(0 0 3.6px #46fffe);
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
                width: 10px;
                height: 0;
                position: absolute;
                top: -9px;
                left: 41px;
                transform-origin: 20px 70px;
                border: 10px solid transparent;
                border-bottom: 0 solid;
                border-top: 20px solid #46fffe;
                border-radius: 8px;
                filter: drop-shadow(0 0 3.6px #46fffe);
            }

            .reactor .trapezoid-container .trapezoid:after {
                content: "";
                position: absolute;
                width: 16px;
                height: 16px;
                border-radius: 50%;
                background: radial-gradient(circle, #e4ffff 30%, #1ffffd);
                margin: 0;
                bottom: 3px;
                right: -3px;
            }

            .reactor .small-circle-container {
                position: relative;
                width: 100%;
                height: 100%;
            }

            .reactor .small-circle-container .small-circle {
                position: absolute;
                width: 8px;
                height: 8px;
                top: -100px;
                left: 42px;
                transform-origin: 18px 41px;
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
                width: 61px;
                height: 61px;
                border: 2px solid #52fefe;
                background-color: #ffffff;
                border-radius: 50%;
                box-shadow: 0 0 4px 2px #52fefe, 0 0 4px 1.6px #52fefe inset;
            }

            .reactor .circle-center {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 44px;
                height: 44px;
                background-color: #0c4d5b;
                border-radius: 50%;
                box-shadow: 0 0 4px 2px #52fefe, 0 0 4px 1.6px #52fefe;
            }

            .reactor .circle-innner {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 38px;
                height: 38px;
                border: 2px solid #52fefe;
                background-color: #ffffff;
                border-radius: 50%;
                box-shadow: 0 0 4px 2px #52fefe, 0 0 9px 3px #52fefe inset;
            }
            .reactor .circle-innner:hover {
               background-color: #ff0000; 
            }

            "#;
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
              <a href="https://github.com/ivanceras/sauron/tree/master/examples/arc-reactor/src/lib.rs"
                 alt="view code"
                 title="view code">
                <div class="circle-innner"></div>
              </a>
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
