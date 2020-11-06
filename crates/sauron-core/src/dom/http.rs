//! provides functions for retrieving data using http network request
use crate::{mt_dom::Callback, Cmd, Component, Dispatch};
use js_sys::TypeError;
use std::fmt::Debug;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::Headers;
use web_sys::Response;

/// Provides functions for doing http network request
#[derive(Copy, Clone, Debug)]
pub struct Http;

impl Http {
    /// fetch text document from the url and decode the result with the supplied
    /// response_text_decoder function
    pub fn fetch_with_text_response_decoder<APP, MSG, DE, CB, ERRCB, OUT>(
        url: &str,
        response_text_decoder: DE,
        cb: CB,
        err_cb: ERRCB,
    ) -> Cmd<APP, MSG>
    where
        CB: Fn(Result<OUT, Response>) -> MSG + Clone + 'static,
        ERRCB: Fn(TypeError) -> MSG + Clone + 'static,
        DE: Fn(String) -> OUT + Clone + 'static,
        OUT: 'static,
        APP: Component<MSG> + 'static,
        MSG: 'static,
    {
        let response_text_decoder = Callback::from(response_text_decoder);
        let cb = Callback::from(cb);
        let cb_clone = cb.clone();
        let success_text_decoder =
            move |(response_text, _headers): (String, Headers)| {
                let msg_value = response_text_decoder.emit(response_text);
                cb.emit(Ok(msg_value))
            };
        let fail_cb = move |resp| cb_clone.emit(Err(resp));
        let err_cb = move |type_error| err_cb(type_error);
        Self::fetch_with_response_decoder(
            url,
            success_text_decoder,
            fail_cb,
            err_cb,
        )
    }

    /// API for fetching http rest request
    pub fn fetch_with_response_decoder<APP, MSG, SUCCEED, FAIL, ERR>(
        url: &str,
        success_text_decoder: SUCCEED,
        fail_cb: FAIL,
        err_cb: ERR,
    ) -> Cmd<APP, MSG>
    where
        APP: Component<MSG> + 'static,
        MSG: 'static,
        SUCCEED: Fn((String, Headers)) -> MSG + 'static,
        FAIL: Fn(Response) -> MSG + 'static,
        ERR: Fn(TypeError) -> MSG + 'static,
    {
        let url_clone = url.to_string();
        let success_text_decoder = Callback::from(success_text_decoder);

        let fail_cb = Callback::from(fail_cb);
        let err_cb = Callback::from(err_cb);

        let cmd: Cmd<APP, MSG> = Cmd::new(move |program| {
            let program_clone = program.clone();

            let success_text_decoder = success_text_decoder.clone();

            let fail_cb = fail_cb.clone();
            let err_cb = err_cb.clone();

            let program_clone_status_err = program.clone();

            let promise = crate::window().fetch_with_str(&url_clone);
            let cb: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let fail_cb = fail_cb.clone();
                    let response: Response = js_value.unchecked_into();
                    let status = response.status();
                    if status == 200 {
                        let response_promise =
                            response.text().expect("must be a promise text");
                        let decoder_and_dispatcher_cb: Closure<
                            dyn FnMut(JsValue),
                        > = Closure::once(move |js_value: JsValue| {
                            let response_text = js_value
                                .as_string()
                                .expect("There's no string value");
                            let msg = success_text_decoder
                                .emit((response_text, response.headers()));
                            program_clone.dispatch(msg);
                        });

                        let _ =
                            response_promise.then(&decoder_and_dispatcher_cb);

                        decoder_and_dispatcher_cb.forget();
                    } else {
                        program_clone_status_err
                            .dispatch(fail_cb.emit(response));
                    }
                });

            let err_closure: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let type_error: TypeError = js_value.unchecked_into();
                    program.dispatch(err_cb.emit(type_error));
                });

            let _ = promise.then(&cb).catch(&err_closure);

            cb.forget();
            err_closure.forget();
        });

        cmd
    }
}
