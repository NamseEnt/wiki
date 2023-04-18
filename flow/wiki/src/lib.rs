use crate::flow::prelude::*;
pub use flow::prelude as flow;
use serde::*;

#[derive(Serialize, Deserialize)]
pub struct WikiAppModel {
    pub title: String,
    pub markdown_content: String,
    pub search_bar: SearchBarModel,
}

impl WikiAppModel {
    pub fn new(title: String, markdown_content: String) -> Self {
        Self {
            title,
            markdown_content,
            search_bar: SearchBarModel {
                input: "".to_string(),
            },
        }
    }
}

impl ViewModel<WikiAppView> for WikiAppModel {
    fn reduce(self, event: &dyn std::any::Any) -> Self {
        Self {
            title: self.title,
            markdown_content: self.markdown_content,
            search_bar: self.search_bar.reduce(event),
        }
    }
    fn as_view(&self) -> WikiAppView {
        WikiAppView {
            title: self.title.clone(),
            markdown_content: self.markdown_content.clone(),
            search_bar: self.search_bar.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SearchBarModel {
    pub input: String,
}

impl Reduce for SearchBarModel {
    fn reduce(self, event: &dyn std::any::Any) -> Self {
        if let Some(event) = event.downcast_ref::<SearchBarEvent>() {
            match event {
                SearchBarEvent::InputChanged(input) => {
                    flow::log!("SearchBarEvent::InputChanged, input: {}", input);
                    Self {
                        input: input.clone(),
                    }
                }
            }
        } else {
            self
        }
    }
}

pub enum SearchBarEvent {
    InputChanged(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct WikiAppView {
    pub title: String,
    pub markdown_content: String,
    pub search_bar: SearchBarModel,
}

impl Render for WikiAppView {
    fn render(self: Box<Self>) -> Element {
        render((
            nav(
                (),
                SearchBarView {
                    model: self.search_bar,
                },
            ),
            section(
                (),
                (
                    h1((), self.title),
                    ContentBodyView {
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
pub struct ContentBodyView {
    pub markdown_content: String,
}

impl Render for ContentBodyView {
    fn render(self: Box<Self>) -> Element {
        let html = self.markdown_to_html(&self.markdown_content);
        raw_html(html)
    }
    fn on_mount(&self) {
        flow::log!("ContentBodyView mounted");
    }
    fn on_unmount(&self) {
        flow::log!("ContentBodyView unmounted");
    }
}

impl ContentBodyView {
    fn markdown_to_html(&self, markdown: &str) -> String {
        let arena = comrak::Arena::new();
        let root = comrak::parse_document(&arena, markdown, &comrak::ComrakOptions::default());
        let mut html = vec![];
        comrak::format_html(root, &comrak::ComrakOptions::default(), &mut html).unwrap();
        String::from_utf8(html).unwrap()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct SearchBarView {
    pub model: SearchBarModel,
}

impl Render for SearchBarView {
    fn render(self: Box<Self>) -> Element {
        flow::log!("SearchBarView.render, input: {}", self.model.input);

        render((
            text_input(
                (),
                self.model.input,
                closure((), |value: &String, _capture| {
                    flow::log!("input changed: {}", value);
                    Some(SearchBarEvent::InputChanged(value.to_string()))
                }),
            ),
            button((), "검색"),
        ))
    }
}
