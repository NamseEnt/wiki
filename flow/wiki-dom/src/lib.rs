use wasm_bindgen::prelude::*;

#[wasm_bindgen()]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    flow::dom::hydrate::<wiki::WikiAppModel, wiki::WikiAppView>("root").await;
}
