use flow::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    flow::dom::start_dom("root", WikiAppModel {}, |WikiAppModel {}| WikiAppView {}).await;
}

struct WikiAppModel {}

impl Reduce for WikiAppModel {
    fn reduce(self, event: &dyn std::any::Any) -> Self {
        Self {}
    }
}

#[derive(PartialEq, Clone)]
struct WikiAppView {}

impl Render for WikiAppView {
    fn render(self: Box<Self>) -> Element {
        h1((), "남세엔터 위키:대문")
    }
    fn on_mount(&self) {
        flow::log!("WikiAppView mounted");
    }
    fn on_unmount(&self) {
        flow::log!("WikiAppView unmounted");
    }
}
