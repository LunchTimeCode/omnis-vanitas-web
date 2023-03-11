use crate::{render_welcome, WelcomeApp};
use egui::Ui;
use omni_git::{render_git_app, GitApp};
use omni_randomwalk::{render_random_walk, WalkApp};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WebApp {
    selected: Apps,
    welcome_app: WelcomeApp,
    walk_app: WalkApp,
    git_app: GitApp,
}

impl Default for WebApp {
    fn default() -> Self {
        Self {
            selected: Apps::Welcome,
            welcome_app: WelcomeApp::default(),
            walk_app: WalkApp::default(),
            git_app: GitApp::default(),
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

fn render_selection(web_app: &mut WebApp, ui: &mut Ui) {
    egui::Grid::new("apps_grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui
                .add(egui::SelectableLabel::new(
                    web_app.selected == Apps::Welcome,
                    "Welcome",
                ))
                .clicked()
            {
                web_app.selected = Apps::Welcome;
            }

            if ui
                .add(egui::SelectableLabel::new(
                    web_app.selected == Apps::RandomWalks,
                    "Random Walks",
                ))
                .clicked()
            {
                web_app.selected = Apps::RandomWalks;
            }

            if ui
                .add(egui::SelectableLabel::new(
                    web_app.selected == Apps::GitApps,
                    "Git Apps",
                ))
                .clicked()
            {
                web_app.selected = Apps::GitApps;
            }
        });

        ui.end_row();
    });
}

fn render_selected(web_app: &mut WebApp, ui: &mut Ui) {
    match web_app.selected {
        Apps::Welcome => render_welcome(&mut web_app.welcome_app, ui),
        Apps::RandomWalks => render_random_walk(&mut web_app.walk_app, ui),
        Apps::GitApps => render_git_app(&mut web_app.git_app, ui),
    }
}

fn render_powered_by(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("powered by ");
        ui.hyperlink_to("Silen", "https://github.com/SilenLoc/omnis-vanitas-web");
        ui.label(".");
    });
}
