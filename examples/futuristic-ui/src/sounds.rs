use sauron::wasm_bindgen::JsCast;
use web_sys::{
    HtmlAudioElement,
    HtmlElement,
};

/// play sound in request animation frame
pub fn play(audio: &HtmlAudioElement) {
    let audio = audio.clone();
    crate::execute_in_request_animation_frame(move || {
        let _ = audio.play().expect("must play");
    });
}

/// check if the audio element is already in the document and return it
/// otherwise, create a new audio element and attach it to the document
pub fn preload(sound_url: &str) -> HtmlAudioElement {
    if let Some(existing) = sauron::document().get_element_by_id(sound_url) {
        log::trace!("existing: {:?}", existing);
        let audio: HtmlAudioElement = existing.unchecked_into();
        audio
    } else {
        create_append_audio(sound_url)
    }
}

/// the html audio element is created and appended to the body
fn create_append_audio(sound_url: &str) -> HtmlAudioElement {
    let audio =
        HtmlAudioElement::new_with_src(&sound_url).expect("must not fail");
    audio.set_attribute("id", sound_url).expect("must set id");

    let audio_element: HtmlElement = audio.clone().into();
    let audio_node: web_sys::Node = audio_element.into();
    sauron::body()
        .append_child(&audio_node)
        .expect("must be appended to the body");
    audio
}
