use crate::flow::prelude::*;
pub use flow::prelude as flow;
use serde::*;

#[derive(Serialize, Deserialize)]
pub struct WikiAppModel {
    pub title: String,
    pub markdown_content: String,
}

impl ViewModel<WikiAppView> for WikiAppModel {
    fn reduce(self, _event: &dyn std::any::Any) -> Self {
        self
    }
    fn as_view(&self) -> WikiAppView {
        WikiAppView {
            title: self.title.clone(),
            markdown_content: self.markdown_content.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct WikiAppView {
    pub title: String,
    pub markdown_content: String,
}

impl Render for WikiAppView {
    fn render(self: Box<Self>) -> Element {
        render((
            nav((), ()),
            section(
                (),
                (
                    h1((), self.title),
                    ContentBody {
                        markdown_content: self.markdown_content,
                    },
                ),
            ),
        ))
    }
    fn on_mount(&self) {
        flow::log!("WikiAppView mounted");
    }
    fn on_unmount(&self) {
        flow::log!("WikiAppView unmounted");
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ContentBody {
    pub markdown_content: String,
}

impl Render for ContentBody {
    fn render(self: Box<Self>) -> Element {
        let html = self.markdown_to_html(&self.markdown_content);
        raw_html(html)
    }
    fn on_mount(&self) {
        flow::log!("ContentBody mounted");
    }
    fn on_unmount(&self) {
        flow::log!("ContentBody unmounted");
    }
}

impl ContentBody {
    fn markdown_to_html(&self, markdown: &str) -> String {
        let arena = comrak::Arena::new();
        let root = comrak::parse_document(&arena, markdown, &comrak::ComrakOptions::default());
        let mut html = vec![];
        comrak::format_html(root, &comrak::ComrakOptions::default(), &mut html).unwrap();
        String::from_utf8(html).unwrap()
    }
}
