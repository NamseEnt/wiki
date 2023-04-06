use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    flow::dom::start_dom("root", wiki::WikiAppModel {}, |wiki::WikiAppModel {}| {
        wiki::WikiAppView {}
    })
    .await;
}
