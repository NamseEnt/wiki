mod element;
mod into_element;

pub use element::Element;
pub use into_element::IntoElement;
use std::any::Any;
use std::fmt::Debug;

pub trait Render: AnyEqual + CloneBox + Debug {
    #[deprecated(note = "Please do not use this method.")]
    fn render(self: Box<Self>) -> Element;
    fn on_mount(&self) {}
    fn on_unmount(&self) {}
}

pub trait AnyEqual {
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, _: &dyn Render) -> bool;
}

impl<S: 'static + PartialEq> AnyEqual for S {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Render) -> bool {
        other
            .as_any()
            .downcast_ref::<S>()
            .map_or(false, |a| self == a)
    }
}

pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn Render>;
}

impl<S: 'static + Clone + Render> CloneBox for S {
    fn clone_box(&self) -> Box<dyn Render> {
        Box::new(Clone::clone(self))
    }
}

pub fn render(into_element: impl IntoElement) -> Element {
    into_element.into_element()
}
