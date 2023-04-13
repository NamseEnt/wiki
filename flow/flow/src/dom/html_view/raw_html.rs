use crate::*;

pub fn raw_html(html: impl ToString) -> Element {
    render(RawHtmlView {
        html: html.to_string(),
    })
}

#[derive(Clone, PartialEq, Debug)]
pub struct RawHtmlView {
    pub html: String,
}

impl Render for RawHtmlView {
    fn render(self: Box<Self>) -> Element {
        html_to_element(&self.html)
    }

    fn on_mount(&self) {
        crate::log!("RawHtmlView::mount");
    }

    fn on_unmount(&self) {
        crate::log!("RawHtmlView::on_unmount");
    }
}

fn html_to_element(html: &str) -> Element {
    let dom = scraper::Html::parse_fragment(&html);

    let root = dom.root_element();

    dom_element_to_element(root)
}

fn dom_element_to_element(element: scraper::ElementRef) -> Element {
    let children = element
        .children()
        .filter_map(|child| match child.value() {
            scraper::node::Node::Element(_) => Some(dom_element_to_element(
                scraper::ElementRef::wrap(child).unwrap(),
            )),
            scraper::node::Node::Text(text) => Some(text.text.to_string().into_element()),
            _ => None,
        })
        .collect::<Vec<Element>>();

    match element.value().name() {
        "html" => crate::render(children),
        "h1" => h1((), children),
        "li" => li((), children),
        "p" => p((), children),
        "ul" => ul((), children),
        _ => panic!("Unknown element: {}", element.value().name()),
    }
}
