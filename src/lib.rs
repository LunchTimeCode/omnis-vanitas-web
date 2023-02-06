#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::WebApp;
mod walk_app;
pub use walk_app::WalkApp;
mod git_app;
pub use git_app::GitApp;
mod git_app_renders;
pub use git_app_renders::render_git_app;
mod random_cordinates;
pub use random_cordinates::random_cordinates_one_dim;
pub use random_cordinates::random_cordinates_two_dim;
