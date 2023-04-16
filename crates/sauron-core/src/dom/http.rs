//! provides functions for retrieving data using http network request
use js_sys::TypeError;
use std::fmt::Debug;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, Response};

/// Provides functions for doing http network request
#[derive(Copy, Clone, Debug)]
pub struct Http;

impl Http {
    /// fetch text document from the url and decode the result with the supplied
    pub async fn fetch_with_text_response_decoder(url: &str) -> Result<String, TypeError> {
        let response = Self::fetch_with_request_and_response_decoder(url, None).await?;

        let response_promise = response.text().expect("must be a promise text");

        let response_text = JsFuture::from(response_promise)
            .await
            .expect("must not error")
            .as_string()
            .expect("must be a text");

        Ok(response_text)
    }

    /// API for fetching http rest request
    pub async fn fetch_with_request_and_response_decoder(
        url: &str,
        request_init: Option<RequestInit>,
    ) -> Result<Response, TypeError> {
        let window = web_sys::window().expect("should a refernce to window");

        let fetch_promise = if let Some(ref request_init) = request_init {
            window.fetch_with_str_and_init(url, request_init)
        } else {
            window.fetch_with_str(url)
        };

        match JsFuture::from(fetch_promise).await {
            Ok(result) => {
                let response: Response = result.unchecked_into();
                Ok(response)
            }
            Err(err) => {
                let type_error: TypeError = err.unchecked_into();
                Err(type_error)
            }
        }
    }
}
