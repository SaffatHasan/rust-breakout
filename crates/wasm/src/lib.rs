use core::ui::BreakoutApp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Redirect log messages to browser console
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    #[wasm_bindgen]
    pub async fn start(
        &self,
        canvas: web_sys::HtmlCanvasElement,
    ) -> Result<(), wasm_bindgen::JsValue> {
        // Sets up the panic hook so Rust errors show in JS console
        console_error_panic_hook::set_once();

        let web_options = eframe::WebOptions::default();

        // Use self.runner to start the app on the provided canvas element
        self.runner
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(BreakoutApp::new(cc)))),
            )
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to start eframe: {:?}", e)))
    }

    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
    }
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    let handle = WebHandle::new();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("the_canvas_id")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    handle.start(canvas).await?;

    Ok(())
}
