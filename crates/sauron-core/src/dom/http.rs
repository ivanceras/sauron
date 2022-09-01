//! provides functions for retrieving data using http network request
use crate::dom::Callback;
use crate::{Application, Cmd, Dispatch, Program};
use js_sys::TypeError;
use std::fmt::Debug;
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestInit;
use web_sys::Response;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

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

                let fetch_cb = fetch_cb.clone();
                let response_promise =
                    response.text().expect("must be a promise text");

                let response_fut = JsFuture::from(response_promise);

                spawn_local(async move{
                    let response_text =
                        response_fut.await
                            .expect("must not error")
                            .as_string()
                            .expect("must be a text");

                    program.dispatch(fetch_cb.emit(response_text));
                });
            };

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
            let window =
                web_sys::window().expect("should a refernce to window");

            let fetch_promise = if let Some(ref request_init) = request_init {
                window.fetch_with_str_and_init(&url_clone, request_init)
            } else {
                window.fetch_with_str(&url_clone)
            };

            let fetch_fut = JsFuture::from(fetch_promise);

            spawn_local(async move{
                match fetch_fut.await{
                    Ok(result) => {
                        let response: Response = result.unchecked_into();
                        decoder_dispatcher_cb.emit((response, program));
                    }
                    Err(err) => {
                        let type_error: TypeError = err.unchecked_into();
                        program.dispatch(error_cb.emit(type_error));
                    }
                }
            });

        })
    }
}
