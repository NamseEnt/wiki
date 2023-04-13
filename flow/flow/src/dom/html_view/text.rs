use crate::*;

impl IntoElement for &str {
    fn into_element(self) -> Element {
        text(self)
    }
}
impl IntoElement for &String {
    fn into_element(self) -> Element {
        text(self)
    }
}
impl IntoElement for String {
    fn into_element(self) -> Element {
        text(self)
    }
}

fn text(text: impl ToString) -> Element {
    Element::Single {
        box_render: Box::new(HtmlNodeView::Text(TextNodeView {
            text: text.to_string(),
        })),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TextNodeView {
    pub text: String,
}

impl Render for TextNodeView {
    fn render(self: Box<Self>) -> Element {
        Element::Multiple { elements: vec![] }
    }

    fn on_mount(&self) {}

    fn on_unmount(&self) {
        crate::log!("TextNode::on_unmount");
    }
}
