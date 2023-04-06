use super::*;
use crate::Render;

pub fn h1(props: impl H1Props, children: impl IntoElement) -> Element {
    crate::log!("h1");
    let mut h1 = H1View {
        style: None,
        on_click: None,
        children: children.into_element(),
    };
    props.add_to(&mut h1);
    Element::Single {
        box_render: Box::new(h1),
    }
}

#[derive(Clone, PartialEq)]
pub struct H1View {
    style: Option<HtmlStyle>,
    on_click: Option<OnClick>,
    children: Element,
}

impl Render for H1View {
    fn render(self: Box<Self>) -> Element {
        self.children
    }

    fn on_mount(&self) {}

    fn on_unmount(&self) {
        crate::log!("H1View::on_unmount");
    }
}

pub trait H1Props {
    fn add_to(self, h1: &mut H1View);
}

impl H1Props for () {
    fn add_to(self, _li: &mut H1View) {}
}
impl<T0, T1> H1Props for (T0, T1)
where
    T0: H1Props,
    T1: H1Props,
{
    fn add_to(self, h1: &mut H1View) {
        self.0.add_to(h1);
        self.1.add_to(h1);
    }
}

impl H1Props for HtmlStyle {
    fn add_to(self, h1: &mut H1View) {
        h1.style = Some(self);
    }
}
impl H1Props for OnClick {
    fn add_to(self, h1: &mut H1View) {
        h1.on_click = Some(self);
    }
}
