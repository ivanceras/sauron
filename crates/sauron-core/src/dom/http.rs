//! provides functions for retrieving data using http network request
use crate::dom::Callback;
use crate::{Application, Cmd, Dispatch, Program};
use js_sys::TypeError;
use std::fmt::Debug;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::RequestInit;
use web_sys::Response;

/// Provides functions for doing http network request
#[derive(Copy, Clone, Debug)]
pub struct Http;

impl Http {
    /// fetch text document from the url and decode the result with the supplied
    /// response_text_decoder function
    pub fn fetch_with_text_response_decoder<APP, MSG, SUCCESS, ERROR>(
        url: &str,
        fetch_cb: SUCCESS,
        error_cb: ERROR,
    ) -> Cmd<APP, MSG>
    where
        SUCCESS: Fn(String) -> MSG + Clone + 'static,
        ERROR: Fn(TypeError) -> MSG + Clone + 'static,
        APP: Application<MSG> + 'static,
        MSG: 'static,
    {
        let fetch_cb = Callback::from(fetch_cb);

        let decoder_dispatcher =
            move |(response, program): (Response, Program<APP, MSG>)| {
                let response_promise =
                    response.text().expect("must be a promise text");

                let fetch_cb = fetch_cb.clone();

                let dispatcher: Closure<dyn FnMut(JsValue)> =
                    Closure::once(move |js_value: JsValue| {
                        let response_text = js_value
                            .as_string()
                            .expect("There's no string value");
                        let msg = fetch_cb.emit(response_text);
                        program.dispatch(msg);
                    });

                let _ = response_promise.then(&dispatcher);

                dispatcher.forget();
            };

        let error_cb = move |type_error| error_cb(type_error);
        Self::fetch_with_request_and_response_decoder(
            url,
            None,
            decoder_dispatcher,
            error_cb,
        )
    }

    /// API for fetching http rest request
    /// error_cb - request failed, in cases where a network is down, server is dead, etc.
    pub fn fetch_with_request_and_response_decoder<APP, MSG, DECODER, ERROR>(
        url: &str,
        request_init: Option<RequestInit>,
        decoder_dispatcher: DECODER,
        error_cb: ERROR,
    ) -> Cmd<APP, MSG>
    where
        APP: Application<MSG> + 'static,
        MSG: 'static,
        DECODER: Fn((Response, Program<APP, MSG>)) + 'static,
        ERROR: Fn(TypeError) -> MSG + 'static,
    {
        let url_clone = url.to_string();

        let error_cb = Callback::from(error_cb);
        let decoder_dispatcher_cb = Callback::from(decoder_dispatcher);
        Cmd::new(move |program| {
            let program_clone = program.clone();
            let error_cb = error_cb.clone();

            let window =
                web_sys::window().expect("should a refernce to window");

            let fetch_promise = if let Some(ref request_init) = request_init {
                window.fetch_with_str_and_init(&url_clone, request_init)
            } else {
                window.fetch_with_str(&url_clone)
            };

            let decoder_dispatcher_cb = decoder_dispatcher_cb.clone();
            let fetch_cb_closure: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let response: Response = js_value.unchecked_into();
                    decoder_dispatcher_cb.emit((response, program_clone));
                });

            let error_cb_closure: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let type_error: TypeError = js_value.unchecked_into();
                    program.dispatch(error_cb.emit(type_error));
                });

            let _ = fetch_promise
                .then(&fetch_cb_closure)
                .catch(&error_cb_closure);

            fetch_cb_closure.forget();
            error_cb_closure.forget();
        })
    }
}
