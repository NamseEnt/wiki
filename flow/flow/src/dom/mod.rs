mod html_view;
mod html_virtual_node;
#[cfg(feature = "dom")]
mod hydrate;
#[cfg(feature = "dom-ssr")]
pub mod server_side_render;
#[cfg(feature = "dom")]
mod start;
pub mod style;

use crate::Closured;
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

#[derive(Clone, PartialEq, Debug)]
pub struct OnClick {
    closure: Closured<()>,
}

pub fn on_click<Capture, Return: 'static>(
    capture: Capture,
    func: fn(&(), &Capture) -> Option<Return>,
) -> OnClick
where
    Capture: std::any::Any + Clone + PartialEq + std::fmt::Debug,
{
    OnClick {
        closure: crate::closure(capture, func).to_closured(),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Href {
    url: String,
}

pub fn href(url: impl ToString) -> Href {
    Href {
        url: url.to_string(),
    }
}
