mod render_app;

use crate::app::render_app::{render_powered_by, render_selected, render_selection};
use crate::typefast::TypeFastApp;
use omni_git::GitApp;
use omni_randomwalk::WalkApp;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WebApp {
    selected: Apps,
    walk_app: WalkApp,
    git_app: GitApp,
    typefast_app: TypeFastApp,
}

impl Default for WebApp {
    fn default() -> Self {
        Self {
            selected: Apps::Welcome,
            walk_app: WalkApp::default(),
            git_app: GitApp::default(),
            typefast_app: TypeFastApp::default(),
        }
    }
}

impl WebApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum Apps {
    Welcome,
    RandomWalks,
    GitApps,
    TypeFastApp,
}

impl eframe::App for WebApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_grid").show(ctx, |ui| {
            ui.add_space(3.0);
            render_selection(self, ui);
            ui.add_space(3.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            render_selected(self, ui);
            render_powered_by(ui);
            egui::warn_if_debug_build(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
