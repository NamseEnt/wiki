use crate::*;

pub fn p(props: impl PProps, children: impl IntoElement) -> Element {
    crate::log!("p()");
    let mut p = PView {
        style: None,
        on_click: None,
        children: children.into_element(),
    };
    props.add_to(&mut p);
    Element::Single {
        box_render: Box::new(HtmlNodeView::P(p)),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PView {
    style: Option<HtmlStyle>,
    on_click: Option<OnClick>,
    children: Element,
}

impl Render for PView {
    fn render(self: Box<Self>) -> Element {
        self.children
    }

    fn on_mount(&self) {
        crate::log!("PView::mount");
    }

    fn on_unmount(&self) {
        crate::log!("PView::on_unmount");
    }
}

pub trait PProps {
    fn add_to(self, p: &mut PView);
}

impl PProps for () {
    fn add_to(self, _li: &mut PView) {}
}
impl<T0, T1> PProps for (T0, T1)
where
    T0: PProps,
    T1: PProps,
{
    fn add_to(self, p: &mut PView) {
        self.0.add_to(p);
        self.1.add_to(p);
    }
}

impl PProps for HtmlStyle {
    fn add_to(self, p: &mut PView) {
        p.style = Some(self);
    }
}
impl PProps for OnClick {
    fn add_to(self, p: &mut PView) {
        p.on_click = Some(self);
    }
}
