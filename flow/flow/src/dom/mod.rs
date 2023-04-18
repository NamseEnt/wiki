mod html_view;
mod html_virtual_node;
#[cfg(feature = "dom")]
mod hydrate;
#[cfg(feature = "dom-ssr")]
pub mod server_side_render;
#[cfg(feature = "dom")]
mod start;
pub mod style;

pub use html_view::*;
pub use html_virtual_node::*;
#[cfg(feature = "dom")]
pub use hydrate::*;
#[cfg(feature = "dom")]
pub use start::*;
use std::fmt::Debug;
pub use style::*;

#[cfg(any(feature = "dom", feature = "dom-ssr"))]
const INITIAL_STATE: &str = "__INITIAL_STATE__";

pub struct OnClick {
    event: Box<dyn crate::AnyClonePartialEq>,
}

impl Clone for OnClick {
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone_box(),
        }
    }
}
impl PartialEq for OnClick {
    fn eq(&self, other: &Self) -> bool {
        self.event.equals(other.event.as_ref())
    }
}
impl Debug for OnClick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.event.debug(f)
    }
}

pub fn on_click(event: impl std::any::Any + Clone + PartialEq + Debug) -> OnClick {
    OnClick {
        event: Box::new(event),
    }
}
