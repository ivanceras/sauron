//! provides functions for retrieving data using http network request
use crate::html::attributes::Callback;
use crate::{Cmd, Component, Dispatch};
use js_sys::TypeError;
use std::fmt::Debug;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::Headers;
use web_sys::RequestInit;
use web_sys::Response;

/// Provides functions for doing http network request
#[derive(Copy, Clone, Debug)]
pub struct Http;

impl Http {
    /// fetch text document from the url and decode the result with the supplied
    /// response_text_decoder function
    pub fn fetch_with_text_response_decoder<APP, MSG, CB, ERRCB>(
        url: &str,
        cb: CB,
        err_cb: ERRCB,
    ) -> Cmd<APP, MSG>
    where
        CB: Fn(String) -> MSG + Clone + 'static,
        ERRCB: Fn(TypeError) -> MSG + Clone + 'static,
        APP: Component<MSG> + 'static,
        MSG: 'static,
    {
        let cb = Callback::from(cb);
        let text_response_decoder =
            move |(_status_code, response_text, _headers): (
                u16,
                String,
                Headers,
            )| { cb.emit(response_text) };
        let err_cb = move |type_error| err_cb(type_error);
        Self::fetch_with_response_decoder(url, text_response_decoder, err_cb)
    }

    /// API for fetching http rest request
    pub fn fetch_with_response_decoder<APP, MSG, DECODE, ERR>(
        url: &str,
        text_response_decoder: DECODE,
        err_cb: ERR,
    ) -> Cmd<APP, MSG>
    where
        APP: Component<MSG> + 'static,
        MSG: 'static,
        DECODE: Fn((u16, String, Headers)) -> MSG + 'static,
        ERR: Fn(TypeError) -> MSG + 'static,
    {
        Self::fetch_with_request_and_response_decoder(
            url,
            None,
            text_response_decoder,
            err_cb,
        )
    }

    /// API for fetching http rest request
    /// err_cb - request failed, in cases where a network is down, server is dead, etc.
    pub fn fetch_with_request_and_response_decoder<APP, MSG, DECODE, ERR>(
        url: &str,
        request_init: Option<RequestInit>,
        text_response_decoder: DECODE,
        err_cb: ERR,
    ) -> Cmd<APP, MSG>
    where
        APP: Component<MSG> + 'static,
        MSG: 'static,
        DECODE: Fn((u16, String, Headers)) -> MSG + 'static,
        ERR: Fn(TypeError) -> MSG + 'static,
    {
        let url_clone = url.to_string();
        let text_response_decoder = Callback::from(text_response_decoder);

        let err_cb = Callback::from(err_cb);
        Cmd::new(move |program| {
            let program_clone = program.clone();

            let text_response_decoder = text_response_decoder.clone();

            let err_cb = err_cb.clone();

            let window =
                web_sys::window().expect("should a refernce to window");

            let promise = if let Some(ref request_init) = request_init {
                window.fetch_with_str_and_init(&url_clone, request_init)
            } else {
                window.fetch_with_str(&url_clone)
            };

            let cb: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let response: Response = js_value.unchecked_into();
                    let status = response.status();
                    let response_promise =
                        response.text().expect("must be a promise text");
                    let decoder_and_dispatcher_cb: Closure<dyn FnMut(JsValue)> =
                        Closure::once(move |js_value: JsValue| {
                            let response_text = js_value
                                .as_string()
                                .expect("There's no string value");
                            let msg = text_response_decoder.emit((
                                status,
                                response_text,
                                response.headers(),
                            ));
                            program_clone.dispatch(msg);
                        });

                    let _ = response_promise.then(&decoder_and_dispatcher_cb);

                    decoder_and_dispatcher_cb.forget();
                });

            let err_closure: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let type_error: TypeError = js_value.unchecked_into();
                    program.dispatch(err_cb.emit(type_error));
                });

            let _ = promise.then(&cb).catch(&err_closure);

            cb.forget();
            err_closure.forget();
        })
    }
}
