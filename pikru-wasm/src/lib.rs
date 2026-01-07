use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const RENDER_OPTIONS_DEF: &'static str = r#"
interface RenderOptions {
    cssVariables?: boolean;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "RenderOptions")]
    pub type RenderOptions;

    #[wasm_bindgen(method, getter)]
    fn cssVariables(this: &RenderOptions) -> Option<bool>;
}

/// Render pikchr markup to SVG.
///
/// Returns the SVG string on success, or an error message on failure.
#[wasm_bindgen]
pub fn render(source: &str, options: Option<RenderOptions>) -> Result<String, String> {
    let opts = pikru::RenderOptions {
        css_variables: options
            .as_ref()
            .and_then(|o| o.cssVariables())
            .unwrap_or(false),
    };
    pikru::pikchr_with_options(source, &opts).map_err(|e| e.to_string())
}
