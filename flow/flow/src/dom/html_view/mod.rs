mod h1;
mod li;
mod raw_html;
mod text;

use crate::{render, Render};
pub use h1::*;
pub use li::*;
pub use raw_html::*;
pub use text::*;

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum HtmlNodeView {
    Text(TextNodeView),
    H1(H1View),
    Li(LiView),
    RawHtml(RawHtmlView),
}

impl Render for HtmlNodeView {
    fn render(self: Box<Self>) -> crate::Element {
        match *self {
            HtmlNodeView::Text(text) => render(text),
            HtmlNodeView::H1(h1) => render(h1),
            HtmlNodeView::Li(li) => render(li),
            HtmlNodeView::RawHtml(raw_html) => render(raw_html),
        }
    }
}
