#![deny(warnings)]
use sauron::*;
use wasm_bindgen_test::*;
use sauron::dom::async_delay;

wasm_bindgen_test_configure!(run_in_browser);

// Verify that our DomUpdater's patch method works.
// We test a simple case here, since diff_patch.rs is responsible for testing more complex
// diffing and patching.
#[wasm_bindgen_test]
async fn test_delays() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let t1 = sauron::now();
    log::debug!("t1: {}", t1);
    async_delay(5000).await;
    let t2 = sauron::now();
    log::debug!("t2: {}", t2);

    let elapsed = t2 - t1;
    log::debug!("elapsed: {}", elapsed);
    assert!(elapsed >= 5000.0);
}
