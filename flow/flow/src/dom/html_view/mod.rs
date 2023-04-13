mod h1;
mod li;
mod p;
mod raw_html;
mod text;
mod ul;

use crate::{render, Render};
pub use h1::*;
pub use li::*;
pub use p::*;
pub use raw_html::*;
pub use text::*;
pub use ul::*;

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum HtmlNodeView {
    Text(TextNodeView),
    H1(H1View),
    Li(LiView),
    P(PView),
    Ul(UlView),
}

impl Render for HtmlNodeView {
    fn render(self: Box<Self>) -> crate::Element {
        match *self {
            HtmlNodeView::Text(text) => render(text),
            HtmlNodeView::H1(h1) => render(h1),
            HtmlNodeView::Li(li) => render(li),
            HtmlNodeView::P(p) => render(p),
            HtmlNodeView::Ul(ul) => render(ul),
        }
    }
}
