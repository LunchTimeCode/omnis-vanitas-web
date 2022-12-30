#![warn(clippy::all, rust_2018_idioms)]


fn main() {

    use eframe::{Theme, WebGlContextOption};
    use futures::executor;
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions {
        follow_system_theme: false,
        default_theme: Theme::Dark,

        webgl_context_option: WebGlContextOption::BestFirst,
    };

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "omni_vanitas_canvas", // hardcode it
            web_options,
            Box::new(|_| Box::new(TemplateApp::new())),
        )
        .await
        .expect("failed to start eframe");
    });
}
