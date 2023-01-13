#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::WebApp;
mod random_cordinates;
pub use random_cordinates::random_cordinates;
