#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::WebApp;

mod welcome;
pub use welcome::render_welcome;
pub use welcome::WelcomeApp;
