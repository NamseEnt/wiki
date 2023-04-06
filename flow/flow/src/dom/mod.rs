mod any_clone_partial_eq;
mod html_node;
#[cfg(feature = "dom-ssr")]
pub mod server_side_render;
#[cfg(feature = "dom")]
mod start;
pub mod style;

use any_clone_partial_eq::*;
pub use html_node::*;
#[cfg(feature = "dom")]
pub use start::*;
use std::fmt::Debug;
pub use style::*;

pub struct OnClick {
    event: Box<dyn AnyClonePartialEq>,
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
