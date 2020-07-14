//! provides functions for retrieving data using http network request
use crate::{
    prelude::Callback,
    Cmd,
    Component,
    Dispatch,
};
use wasm_bindgen::{
    closure::Closure,
    JsCast,
    JsValue,
};
use web_sys::Response;

/// Provides functions for doing http network request
#[derive(Copy, Clone, Debug)]
pub struct Http;

impl Http {
    /// fetch text document from the url and decode the result with the supplied
    /// response_text_decoder function
    pub fn fetch_with_text_response_decoder<DE, CB, OUT, APP, MSG>(
        url: &str,
        response_text_decoder: DE,
        cb: CB,
    ) -> Cmd<APP, MSG>
    where
        CB: Fn(Result<OUT, JsValue>) -> MSG + Clone + 'static,
        DE: Fn(String) -> OUT + Clone + 'static,
        OUT: 'static,
        APP: Component<MSG> + 'static,
        MSG: 'static,
    {
        let response_text_decoder = Callback::from(response_text_decoder);
        let cb = Callback::from(cb);
        let cb_clone = cb.clone();
        let response_decoder = move |js_value: JsValue| {
            let response_text =
                js_value.as_string().expect("There's no string value");
            let msg_value = response_text_decoder.emit(response_text);
            cb.emit(Ok(msg_value))
        };
        let fail_cb = move |js_value| cb_clone.emit(Err(js_value));
        Self::fetch_with_response_decoder(url, response_decoder, fail_cb)
    }

    /// API for fetching http rest request
    pub fn fetch_with_response_decoder<F, ERR, APP, MSG>(
        url: &str,
        response_decoder: F,
        fail_cb: ERR,
    ) -> Cmd<APP, MSG>
    where
        F: Fn(JsValue) -> MSG + 'static,
        ERR: Fn(JsValue) -> MSG + 'static,
        APP: Component<MSG> + 'static,
        MSG: 'static,
    {
        let url_clone = url.to_string();
        let response_decoder = Callback::from(response_decoder);

        let fail_cb = Callback::from(fail_cb);

        let cmd: Cmd<APP, MSG> = Cmd::new(move |program| {
            let program_clone = program.clone();

            let response_decoder = response_decoder.clone();

            let decoder_and_dispatcher = move |js_value: JsValue| {
                let msg = response_decoder.emit(js_value);
                program_clone.dispatch(msg);
            };

            let fail_cb = fail_cb.clone();
            let fail_cb2 = fail_cb.clone();

            let program_clone_status_err = program.clone();

            let promise = crate::window().fetch_with_str(&url_clone);
            let cb: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let fail_cb = fail_cb.clone();
                    let response: &Response = js_value.as_ref().unchecked_ref();
                    let status = response.status();
                    if status == 200 {
                        let response_promise = response.text();
                        if let Ok(response_promise) = response_promise {
                            let decoder_and_dispatcher_cb: Closure<
                                dyn FnMut(JsValue),
                            > = Closure::once(decoder_and_dispatcher);

                            let _ = response_promise
                                .then(&decoder_and_dispatcher_cb);

                            decoder_and_dispatcher_cb.forget();
                        } else {
                            panic!("Expecting a string");
                        }
                    } else {
                        program_clone_status_err
                            .dispatch(fail_cb.emit(js_value));
                    }
                });

            let program_clone_response_error = program;
            let fail_closure: Closure<dyn FnMut(JsValue)> =
                Closure::once(move |js_value: JsValue| {
                    let fail_cb = fail_cb2.clone();
                    program_clone_response_error
                        .dispatch(fail_cb.emit(js_value));
                });

            let _ = promise.then(&cb).catch(&fail_closure);

            cb.forget();
            fail_closure.forget();
        });

        cmd
    }
}
