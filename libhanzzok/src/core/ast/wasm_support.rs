use super::HanzzokAstNode;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct HanzzokAstNodeWrapper {
    pub(crate) handle: HanzzokAstNode,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl HanzzokAstNodeWrapper {}
