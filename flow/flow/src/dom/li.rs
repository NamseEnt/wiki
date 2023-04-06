use super::*;
use crate::Render;

pub fn li(props: impl LiProps, children: impl IntoElement) -> Element {
    crate::log!("li");
    let mut li = LiView {
        style: None,
        on_click: None,
        children: children.into_element(),
    };
    props.add_to(&mut li);
    Element::Single {
        box_render: Box::new(li),
    }
}

#[derive(Clone, PartialEq)]
pub struct LiView {
    style: Option<HtmlStyle>,
    on_click: Option<OnClick>,
    children: Element,
}

impl Render for LiView {
    fn render(self: Box<Self>) -> Element {
        self.children
    }

    fn on_mount(&self) {}

    fn on_unmount(&self) {
        crate::log!("LiView::on_unmount");
    }
}

pub trait LiProps {
    fn add_to(self, li: &mut LiView);
}

impl LiProps for () {
    fn add_to(self, _li: &mut LiView) {}
}
impl<T0, T1> LiProps for (T0, T1)
where
    T0: LiProps,
    T1: LiProps,
{
    fn add_to(self, li: &mut LiView) {
        self.0.add_to(li);
        self.1.add_to(li);
    }
}

impl LiProps for HtmlStyle {
    fn add_to(self, li: &mut LiView) {
        li.style = Some(self);
    }
}
impl LiProps for OnClick {
    fn add_to(self, li: &mut LiView) {
        li.on_click = Some(self);
    }
}
