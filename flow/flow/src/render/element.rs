use super::*;

#[derive(Debug)]
pub enum Element {
    Single { box_render: Box<dyn Render> },
    Multiple { elements: Vec<Element> },
}

impl Element {
    pub(crate) fn on_mount(&self) {
        match self {
            Element::Single { box_render } => box_render.on_mount(),
            Element::Multiple { elements } => {
                for element in elements {
                    element.on_mount();
                }
            }
        }
    }
    #[allow(dead_code)]
    pub(crate) fn on_unmount(&self) {
        match self {
            Element::Single { box_render } => box_render.on_unmount(),
            Element::Multiple { elements } => {
                for element in elements {
                    element.on_unmount();
                }
            }
        }
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        match self {
            Element::Single { box_render } => Element::Single {
                box_render: box_render.clone_box(),
            },
            Element::Multiple { elements } => Element::Multiple {
                elements: elements.clone(),
            },
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Element::Single { box_render }, Element::Single { box_render: other }) => {
                box_render.equals(other.as_ref())
            }
            (Element::Multiple { elements }, Element::Multiple { elements: other }) => {
                elements == other
            }
            _ => false,
        }
    }
}

impl Element {
    pub fn event(self, _build: impl FnOnce(&mut EventBuilder)) -> Element {
        render(())
    }
}

pub struct EventBuilder {}

impl EventBuilder {
    pub fn on_click_fn<Event: std::any::Any>(
        &mut self,
        _handler: impl Fn(ClickEvent) -> Option<Event> + 'static,
    ) -> Self {
        todo!()
    }
    pub fn on_click<Event: std::any::Any>(&mut self, _event: Event) -> Self {
        todo!()
    }
}

pub struct ClickEvent {}
