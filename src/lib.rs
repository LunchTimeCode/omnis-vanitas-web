#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::WebApp;
mod git_app;
pub use git_app::GitApp;
mod git_app_renders;
pub use git_app_renders::render_git_app;
mod boids;
pub use boids::render_boids;
pub use boids::BoidApp;
