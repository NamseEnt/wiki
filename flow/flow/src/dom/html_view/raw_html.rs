use crate::*;

pub fn raw_html(html: impl ToString) -> Element {
    crate::log!("raw_html()");
    let raw_html = RawHtmlView {
        html: html.to_string(),
    };
    Element::Single {
        box_render: Box::new(HtmlNodeView::RawHtml(raw_html)),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RawHtmlView {
    pub html: String,
}

impl Render for RawHtmlView {
    fn render(self: Box<Self>) -> Element {
        Element::Multiple { elements: vec![] }
    }

    fn on_mount(&self) {
        crate::log!("RawHtmlView::mount");
    }

    fn on_unmount(&self) {
        crate::log!("RawHtmlView::on_unmount");
    }
}
