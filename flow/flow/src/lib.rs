pub mod dom;
mod model;
pub mod prelude;
mod render;
mod render_tree;
mod start;

pub use anyhow;
pub use anyhow::Result;
pub use dom::*;
pub use model::*;
pub use render::*;
pub use start::*;

pub fn default<T: Default>() -> T {
    T::default()
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        #[cfg(feature = "dom")]
        web_sys::console::log_1(&format_args!($($arg)*).to_string().into());
        #[cfg(feature = "dom-ssr")]
        println!($($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "dom")]
        web_sys::console::error_1(&format_args!($($arg)*).to_string().into());
        #[cfg(feature = "dom-ssr")]
        println!($($arg)*);
    };
}
