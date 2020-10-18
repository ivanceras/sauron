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
        vec![r#"
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
                width: 60vmin;
                height: 60vmin;
                border-radius: 50%;
                background: radial-gradient(circle, #002d34 30%, #0089a9 60%, #00a6ce);
                border: 1vw solid rgba(0, 45, 52, 0.3);
                filter: drop-shadow(0 0 1.8vmin #46fffe);
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
                animation: spin 10s infinite linear;
            }

            .reactor .trapezoid-container .trapezoid {
                box-sizing: content-box;
                width: 5vmin;
                height: 0;
                position: absolute;
                top: -4.5vmin;
                left: 20.5vmin;
                transform-origin: 10vmin 35vmin;
                border: 5vmin solid transparent;
                border-bottom: 0 solid;
                border-top: 10vmin solid #46fffe;
                border-radius: 4vmin;
                filter: drop-shadow(0 0 1.8vmin #46fffe);
            }

            .reactor .trapezoid-container .trapezoid:after {
                content: "";
                position: absolute;
                width: 8vmin;
                height: 8vmin;
                border-radius: 50%;
                background: radial-gradient(circle, #e4ffff 30%, #1ffffd);
                margin: 0;
                bottom: 1.5vmin;
                right: -1.5vmin;
            }

            .reactor .trapezoid-container .trapezoid-1 {
                transform: rotate(0deg);
            }

            .reactor .trapezoid-container .trapezoid-2 {
                transform: rotate(36deg);
            }

            .reactor .trapezoid-container .trapezoid-3 {
                transform: rotate(72deg);
            }

            .reactor .trapezoid-container .trapezoid-4 {
                transform: rotate(108deg);
            }

            .reactor .trapezoid-container .trapezoid-5 {
                transform: rotate(144deg);
            }

            .reactor .trapezoid-container .trapezoid-6 {
                transform: rotate(180deg);
            }

            .reactor .trapezoid-container .trapezoid-7 {
                transform: rotate(216deg);
            }

            .reactor .trapezoid-container .trapezoid-8 {
                transform: rotate(252deg);
            }

            .reactor .trapezoid-container .trapezoid-9 {
                transform: rotate(288deg);
            }

            .reactor .trapezoid-container .trapezoid-10 {
                transform: rotate(324deg);
            }

            .reactor .small-circle-container {
                position: relative;
                width: 100%;
                height: 100%;
            }

            .reactor .small-circle-container .small-circle {
                position: absolute;
                width: 4vmin;
                height: 4vmin;
                top: -50vmin;
                left: 21vmin;
                transform-origin: 9vmin 20.5vmin;
                border-radius: 50%;
                background: radial-gradient(circle, #eeffff 20%, #21fffe, #009fc1);
            }

            .reactor .small-circle-container .small-circle-1 {
                transform: rotate(0deg);
            }

            .reactor .small-circle-container .small-circle-2 {
                transform: rotate(30deg);
            }

            .reactor .small-circle-container .small-circle-3 {
                transform: rotate(60deg);
            }

            .reactor .small-circle-container .small-circle-4 {
                transform: rotate(90deg);
            }

            .reactor .small-circle-container .small-circle-5 {
                transform: rotate(120deg);
            }

            .reactor .small-circle-container .small-circle-6 {
                transform: rotate(150deg);
            }

            .reactor .small-circle-container .small-circle-7 {
                transform: rotate(180deg);
            }

            .reactor .small-circle-container .small-circle-8 {
                transform: rotate(210deg);
            }

            .reactor .small-circle-container .small-circle-9 {
                transform: rotate(240deg);
            }

            .reactor .small-circle-container .small-circle-10 {
                transform: rotate(270deg);
            }

            .reactor .small-circle-container .small-circle-11 {
                transform: rotate(300deg);
            }

            .reactor .small-circle-container .small-circle-12 {
                transform: rotate(330deg);
            }

            .reactor .circle-outer {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 30.5vmin;
                height: 30.5vmin;
                border: 1px solid #52fefe;
                background-color: #ffffff;
                border-radius: 50%;
                box-shadow: 0px 0px 2px 1px #52fefe, 0px 0px 2vw 0.8vmin #52fefe inset;
            }

            .reactor .circle-center {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 22vmin;
                height: 22vmin;
                background-color: #0c4d5b;
                border-radius: 50%;
                box-shadow: 0px 0px 2px 1px #52fefe, 0px 0px 2vw 0.8vmin #52fefe;
            }

            .reactor .circle-innner {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                margin: auto;
                width: 19vmin;
                height: 19vmin;
                border: 1px solid #52fefe;
                background-color: #ffffff;
                border-radius: 50%;
                box-shadow: 0px 0px 2px 1px #52fefe, 0px 0px 4.5vw 1.5vmin #52fefe inset;
            }"#
        .to_string()]
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <div class="reactor">
              <div class="trapezoid-container">
                <div class="trapezoid trapezoid-1"></div>
                <div class="trapezoid trapezoid-2"></div>
                <div class="trapezoid trapezoid-3"></div>
                <div class="trapezoid trapezoid-4"></div>
                <div class="trapezoid trapezoid-5"></div>
                <div class="trapezoid trapezoid-6"></div>
                <div class="trapezoid trapezoid-7"></div>
                <div class="trapezoid trapezoid-8"></div>
                <div class="trapezoid trapezoid-9"></div>
                <div class="trapezoid trapezoid-10"></div>
              </div>

              <div class="small-circle-container">
                <div class="small-circle small-circle-1"></div>
                <div class="small-circle small-circle-2"></div>
                <div class="small-circle small-circle-3"></div>
                <div class="small-circle small-circle-4"></div>
                <div class="small-circle small-circle-5"></div>
                <div class="small-circle small-circle-6"></div>
                <div class="small-circle small-circle-7"></div>
                <div class="small-circle small-circle-8"></div>
                <div class="small-circle small-circle-9"></div>
                <div class="small-circle small-circle-10"></div>
                <div class="small-circle small-circle-11"></div>
                <div class="small-circle small-circle-12"></div>
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
