//! provides functions for retrieving data using http network request
use crate::html::attributes::Callback;
use crate::{Cmd, Component, Dispatch, Program};
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
    pub fn fetch_with_text_response_decoder<
        APP,
        MSG,
        FetchCallback,
        ErrorCallback,
    >(
        url: &str,
        fetch_callback: FetchCallback,
        error_callback: ErrorCallback,
    ) -> Cmd<APP, MSG>
    where
        FetchCallback: Fn(String) -> MSG + Clone + 'static,
        ErrorCallback: Fn(TypeError) -> MSG + Clone + 'static,
        APP: Component<MSG> + 'static,
        MSG: 'static,
    {
        let fetch_callback = Callback::from(fetch_callback);

        let response_decoder_with_dispatcher =
            move |(response, program): (Response, Program<APP, MSG>)| {
                let response_promise =
                    response.text().expect("must be a promise text");

                let fetch_callback = fetch_callback.clone();

                let dispatcher: Closure<dyn FnMut(JsValue)> =
                    Closure::once(move |js_value: JsValue| {
                        let response_text = js_value
                            .as_string()
                            .expect("There's no string value");
                        let msg = fetch_callback.emit(response_text);
                        program.dispatch(msg);
                    });

                let _ = response_promise.then(&dispatcher);

                dispatcher.forget();
            };

        let error_callback = move |type_error| error_callback(type_error);
        Self::fetch_with_request_and_response_decoder(
            url,
            None,
            response_decoder_with_dispatcher,
            error_callback,
        )
    }

    /// API for fetching http rest request
    /// error_callback - request failed, in cases where a network is down, server is dead, etc.
    pub fn fetch_with_request_and_response_decoder<APP, MSG, DECODER, ERR>(
        url: &str,
        request_init: Option<RequestInit>,
        response_decoder_with_dispatcher: DECODER,
        error_callback: ERR,
    ) -> Cmd<APP, MSG>
    where
        APP: Component<MSG> + 'static,
        MSG: 'static,
        DECODER: Fn((Response, Program<APP, MSG>)) + 'static,
        ERR: Fn(TypeError) -> MSG + 'static,
    {
        let url_clone = url.to_string();

        let error_callback = Callback::from(error_callback);
        let response_decoder_with_dispatcher_cb =
            Callback::from(response_decoder_with_dispatcher);
        Cmd::new(move |program| {
            let program_clone = program.clone();
            let error_callback = error_callback.clone();

            let window =
                web_sys::window().expect("should a refernce to window");

            let fetch_promise = if let Some(ref request_init) = request_init {
                window.fetch_with_str_and_init(&url_clone, request_init)
            } else {
                window.fetch_with_str(&url_clone)
            };

            let response_decoder_with_dispatcher_cb =
                response_decoder_with_dispatcher_cb.clone();
            let fetch_callback: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let response: Response = js_value.unchecked_into();
                    response_decoder_with_dispatcher_cb
                        .emit((response, program_clone));
                });

            let error_callback_closure: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let type_error: TypeError = js_value.unchecked_into();
                    program.dispatch(error_callback.emit(type_error));
                });

            let _ = fetch_promise
                .then(&fetch_callback)
                .catch(&error_callback_closure);

            fetch_callback.forget();
            error_callback_closure.forget();
        })
    }
}
