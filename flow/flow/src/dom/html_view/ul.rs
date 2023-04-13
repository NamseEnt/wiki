use crate::*;

pub fn ul(props: impl UlProps, children: impl IntoElement) -> Element {
    crate::log!("ul()");
    let mut ul = UlView {
        style: None,
        on_click: None,
        children: children.into_element(),
    };
    props.add_to(&mut ul);
    Element::Single {
        box_render: Box::new(HtmlNodeView::Ul(ul)),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UlView {
    style: Option<HtmlStyle>,
    on_click: Option<OnClick>,
    children: Element,
}

impl Render for UlView {
    fn render(self: Box<Self>) -> Element {
        self.children
    }

    fn on_mount(&self) {
        crate::log!("UlView::mount");
    }

    fn on_unmount(&self) {
        crate::log!("UlView::on_unmount");
    }
}

pub trait UlProps {
    fn add_to(self, ul: &mut UlView);
}

impl UlProps for () {
    fn add_to(self, _li: &mut UlView) {}
}
impl<T0, T1> UlProps for (T0, T1)
where
    T0: UlProps,
    T1: UlProps,
{
    fn add_to(self, ul: &mut UlView) {
        self.0.add_to(ul);
        self.1.add_to(ul);
    }
}

impl UlProps for HtmlStyle {
    fn add_to(self, ul: &mut UlView) {
        ul.style = Some(self);
    }
}
impl UlProps for OnClick {
    fn add_to(self, ul: &mut UlView) {
        ul.on_click = Some(self);
    }
}
