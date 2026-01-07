use wasm_bindgen::prelude::*;

/// Render pikchr markup to SVG.
///
/// Returns the SVG string on success, or an error message on failure.
#[wasm_bindgen]
pub fn render(source: &str) -> Result<String, String> {
    pikru::pikchr(source).map_err(|e| e.to_string())
}
