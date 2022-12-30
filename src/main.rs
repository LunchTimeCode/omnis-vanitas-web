#![warn(clippy::all, rust_2018_idioms)]

use omnis_vanitas_web::TemplateApp;

// when compiling to web using trunk.
fn main() {
    use eframe::{Theme, WebGlContextOption, WebOptions};
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = WebOptions {
        follow_system_theme: false,
        default_theme: Theme::Dark,
        webgl_context_option: WebGlContextOption::WebGl1,
    };

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "omni_vanitas_canvas", // hardcode it
            web_options,
            Box::new(|cc| Box::new(TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
