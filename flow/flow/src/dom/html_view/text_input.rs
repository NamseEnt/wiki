use crate::*;

pub fn text_input<
    ChangedEvent: std::any::Any,
    ChangedCapture: std::any::Any + Clone + PartialEq + std::fmt::Debug,
>(
    props: impl Props,
    value: impl ToString,
    on_changed: Closure<String, ChangedCapture, ChangedEvent>,
) -> Element {
    let value = value.to_string();

    crate::log!("text_input(), value: {value}");

    let mut text_input: View = View {
        style: None,
        value,
        on_changed: on_changed.to_closured(),
    };
    props.add_to(&mut text_input);
    Element::Single {
        box_render: Box::new(HtmlNodeView::TextInput(text_input)),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct View {
    style: Option<HtmlStyle>,
    pub(crate) value: String,
    pub(crate) on_changed: Closured<String>, // 바뀌면 이걸로 call 해줘
}

impl Render for View {
    fn render(self: Box<Self>) -> Element {
        crate::log!("text_input::View::render. value: {}", self.value);
        Element::Multiple { elements: vec![] }
    }

    fn on_mount(&self) {
        crate::log!("text_input::View::mount");
    }

    fn on_unmount(&self) {
        crate::log!("text_input::View::on_unmount");
    }
}

pub trait Props {
    fn add_to(self, text_input: &mut View);
}

impl Props for () {
    fn add_to(self, _text_input: &mut View) {}
}
impl<T0, T1> Props for (T0, T1)
where
    T0: Props,
    T1: Props,
{
    fn add_to(self, text_input: &mut View) {
        self.0.add_to(text_input);
        self.1.add_to(text_input);
    }
}

impl Props for HtmlStyle {
    fn add_to(self, text_input: &mut View) {
        text_input.style = Some(self);
    }
}
