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

#[wasm_bindgen]
pub struct Pikru {
    options: pikru::RenderOptions,
}

#[wasm_bindgen]
impl Pikru {
    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<RenderOptions>) -> Pikru {
        let options = pikru::RenderOptions {
            css_variables: options
                .as_ref()
                .and_then(|o| o.cssVariables())
                .unwrap_or(false),
        };
        Pikru { options }
    }

    /// Render pikchr markup to SVG.
    ///
    /// Returns the SVG string on success, or an error message on failure.
    pub fn render(&self, source: &str) -> Result<String, String> {
        pikru::pikchr_with_options(source, &self.options).map_err(|e| e.to_string())
    }
}
