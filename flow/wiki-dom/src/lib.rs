use wasm_bindgen::prelude::*;

#[wasm_bindgen()]
pub async fn start(title: String) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // TODO: hydrate model from server side rendered result
    flow::dom::start_dom("root", wiki::WikiAppModel { title }).await;
}
