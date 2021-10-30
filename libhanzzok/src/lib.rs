pub use v_htmlescape::escape;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn w_escape(s: String) -> String {
    escape(s.as_ref()).to_string()
}

pub mod api;
pub mod codegen;
pub mod core;
pub mod syntax;
