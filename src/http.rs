use crate::Dispatch;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure,
                   JsCast,
                   JsValue};
use web_sys::Response;

pub struct Http;

impl Http {
    pub fn fetch_with_text_response_decoder<DE, CB, OUT, DSP, MSG>(program: &Rc<DSP>,
                                                                   url: &str,
                                                                   response_text_decoder: DE,
                                                                   cb: CB)
        where CB: Fn(Result<OUT, JsValue>) -> MSG + Clone + 'static,
              DE: Fn(String) -> OUT + 'static,
              OUT: 'static,
              DSP: Dispatch<MSG> + 'static
    {
        let cb_clone = cb.clone();
        let response_decoder = move |js_value: JsValue| {
            let response_text =
                js_value.as_string().expect("There's no string value");
            let msg_value = response_text_decoder(response_text);
            cb(Ok(msg_value))
        };
        let fail_cb = move |js_value| cb_clone(Err(js_value));
        Self::fetch_with_response_decoder(program,
                                          url,
                                          response_decoder,
                                          fail_cb);
    }

    /// API for fetching http rest request
    pub fn fetch_with_response_decoder<F, ERR, DSP, MSG>(program: &Rc<DSP>,
                                                         url: &str,
                                                         response_decoder: F,
                                                         fail_cb: ERR)
        where F: Fn(JsValue) -> MSG + 'static,
              ERR: Fn(JsValue) -> MSG + Clone + 'static,
              DSP: Dispatch<MSG> + 'static
    {
        let program_clone = Rc::clone(program);

        let decoder_and_dispatcher = move |js_value: JsValue| {
            let msg = response_decoder(js_value);
            program_clone.dispatch(msg);
        };

        let program_clone_status_err = Rc::clone(program);
        let fail_status_cb = fail_cb.clone();

        let promise = crate::window().fetch_with_str(url);
        let cb: Closure<FnMut(JsValue)> =
            Closure::once(move |js_value: JsValue| {
                let response: &Response = js_value.as_ref().unchecked_ref();
                let status = response.status();
                crate::log!("status: {}", status);
                if status == 200 {
                    let response_promise =
                        response.text().expect("expecting a text");
                    let decoder_and_dispatcher_cb: Closure<FnMut(JsValue)> =
                        Closure::once(decoder_and_dispatcher);
                    response_promise.then(&decoder_and_dispatcher_cb);
                    decoder_and_dispatcher_cb.forget();
                } else {
                    program_clone_status_err.dispatch(fail_status_cb(js_value));
                }
            });

        let program_clone_response_error = Rc::clone(program);
        let fail_closure: Closure<FnMut(JsValue)> =
            Closure::once(move |js_value: JsValue| {
                crate::log!("failed to get a response: {:#?}", js_value);
                program_clone_response_error.dispatch(fail_cb(js_value));
            });

        promise.then(&cb).catch(&fail_closure);

        cb.forget();
        fail_closure.forget();
    }
}
