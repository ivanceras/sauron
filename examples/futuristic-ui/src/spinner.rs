use sauron::{
    html::{
        attributes::{
            class,
            style,
        },
        div,
    },
    prelude::*,
    Node,
};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Spinner<MSG> {
    _phantom: PhantomData<MSG>,
}

impl<MSG> Spinner<MSG> {
    pub fn new() -> Self {
        Spinner {
            _phantom: PhantomData,
        }
    }

    pub fn style(&self) -> Vec<String> {
        vec![r#"

            .spinner {
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                position: absolute;
            }
            .spinner-anim {
                z-index: 1000;
                display: block;
                opacity: 1;
                position: relative;
                min-height: 90px;
                transition: all 250ms ease-out;
            }
            .spinner__circle {
                border-top: 5px solid #26dafd;
                border-bottom: 5px solid #26dafd;
                box-shadow: 0 0 8px #26dafd;
            }
            .snipper__circle1 {
                width: 50px;
                height: 50px;
                animation: spinner-loading-circle1 750ms infinite linear;
                margin-top: -25px;
                margin-left: -25px;
            }
            .spinner__circle-anim {
                top: 50%;
                left: 50%;
                display: block;
                position: absolute;
                transition: all 250ms ease-out;
                border-left: 5px solid transparent;
                border-right: 5px solid transparent;
                border-radius: 50%;
                background-color: transparent;
            }
            .spinner__circle2 {
                width: 30px;
                height: 30px;
                animation: spinner-loading-circle2 750ms infinite linear;
                margin-top: -15px;
                margin-left: -15px;
            }

            @keyframes spinner-loading-circle1 {
              0% {
                transform: rotate(160deg);
                opacity: 0;
              }

              50% {
                transform: rotate(145deg);
                opacity: 1;
              }

              100% {
                transform: rotate(-320deg);
                opacity: 0;
              }
            }

            @keyframes spinner-loading-circle2 {
              0% {
                transform: rotate(0deg);
              }

              100% {
                transform: rotate(360deg);
              }
            }
                "#
        .to_string()]
    }

    pub fn view(&self) -> Node<MSG> {
        div(
            vec![],
            vec![div(
                vec![
                    style("position", "relative"),
                    style("width", px(200)),
                    style("height", px(200)),
                ],
                vec![div(
                    vec![class("spinner spinner-anim")],
                    vec![
                        div(
                            vec![class(
                                "spinner__circle spinner__circle-anim snipper__circle1",
                            )],
                            vec![],
                        ),
                        div(
                            vec![class(
                                "spinner__circle spinner__circle-anim spinner__circle2",
                            )],
                            vec![],
                        ),
                    ],
                )],
            )],
        )
    }
}
