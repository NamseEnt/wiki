use flow::prelude::*;

pub struct WikiAppModel {
    pub title: String,
}

impl ViewModel<WikiAppView> for WikiAppModel {
    fn reduce(self, event: &dyn std::any::Any) -> Self {
        self
    }
    fn as_view(&self) -> WikiAppView {
        WikiAppView {
            title: self.title.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct WikiAppView {
    pub title: String,
}

impl Render for WikiAppView {
    fn render(self: Box<Self>) -> Element {
        h1((), self.title)
    }
    fn on_mount(&self) {
        flow::log!("WikiAppView mounted");
    }
    fn on_unmount(&self) {
        flow::log!("WikiAppView unmounted");
    }
}
