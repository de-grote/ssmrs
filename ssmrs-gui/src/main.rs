#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(not(target_arch = "wasm32"), tokio::main)]
async fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "SSMRS",
        native_options,
        Box::new(|cc| Ok(Box::new(ssmrs_gui::SSMRS::new(cc)))),
    )
    .unwrap();
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
async fn start() {
    // Make sure panics are logged using `console.error`.

    use eframe::wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let runner = eframe::WebRunner::new();
    let elem = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("the_canvas_id")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    runner
        .start(
            elem,
            eframe::WebOptions::default(),
            Box::new(|cc| Ok(Box::new(ssmrs_gui::SSMRS::new(cc)))),
        )
        .await
        .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // unused
}
