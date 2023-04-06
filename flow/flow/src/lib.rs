pub mod dom;
pub mod prelude;
mod reduce;
mod render;
mod render_tree;
mod start;

pub use dom::*;
pub use reduce::*;
pub use render::*;
pub use start::*;

pub fn default<T: Default>() -> T {
    T::default()
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&format_args!($($arg)*).to_string().into());
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        web_sys::console::error_1(&format_args!($($arg)*).to_string().into());
    };
}
