use egui::Ui;

use crate::{
    git_app::GitApp, git_app_renders::render_git_app, walk_app::render_random_walk, WalkApp,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WebApp {
    selected: Apps,
    walk_app: WalkApp,
    git_app: GitApp,
    dropped_files: Vec<egui::DroppedFile>,
}

impl Default for WebApp {
    fn default() -> Self {
        Self {
            selected: Apps::Welcome,
            walk_app: WalkApp::default(),
            git_app: GitApp::default(),
            dropped_files: Default::default(),
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
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_grid").show(ctx, |ui| {
            ui.add_space(3.0);
            render_selection(self, ui);
            ui.add_space(3.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            render_selected(self, ui);
            render_powered_by(ui);
            egui::warn_if_debug_build(ui);
        });
    }
}

// example for file drag and drop
//impl WebApp {
//  fn ui_file_drag_and_drop(&mut self, ctx: &egui::Context) {
// use egui::*;
// use std::fmt::Write as _;

//  if !ctx.in

// Preview hovering files:
//if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
//  let text = ctx.input(|i| {
//     let mut text = "Dropping files:\n".to_owned();
//for file in &i.raw.hovered_files {
//      if let Some(path) = &file.path {
// write!(text, "\n{}", path.display()).ok();
//} else if !file.mime.is_empty() {
//    write!(text, "\n{}", file.mime).ok();
// } else {
//    text += "\n???";
//}
//}
//text
// });

//let painter =
// ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

// let screen_rect = ctx.screen_rect();
//painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
//painter.text(
// screen_rect.center(),
// Align2::CENTER_CENTER,
// text,
// TextStyle::Heading.resolve(&ctx.style()),
// Color32::WHITE,
//);
//}

// Collect dropped files:
//ctx.input(|i| {
//    if !i.raw.dropped_files.is_empty() {
//    self.dropped_files = i.raw.dropped_files.clone();
//}
//});

// Show dropped files (if any):
//if !self.dropped_files.is_empty() {
//  let mut open = true;
// egui::Window::new("Dropped files")
// .open(&mut open)
// .show(ctx, |ui| {
//for file in &self.dropped_files {
//  let mut info = if let Some(path) = &file.path {
//  path.display().to_string()
// } else if !file.name.is_empty() {
//  file.name.clone()
// } else {
//    "???".to_owned()
// };
//if let Some(bytes) = &file.bytes {
//    write!(info, " ({} bytes)", bytes.len()).ok();
//}
//ui.label(info);
//}
// });
//if !open {
//   self.dropped_files.clear();
//}
//  }
// }
//}

fn render_selection(web_app: &mut WebApp, ui: &mut Ui) {
    egui::Grid::new("apps_grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Applications");
            if ui.button("Welcome").clicked() {
                web_app.selected = Apps::Welcome;
            };
            if ui.button("Random Walks").clicked() {
                web_app.selected = Apps::RandomWalks;
            };
            if ui.button("Git Apps").clicked() {
                web_app.selected = Apps::GitApps;
            };
        });

        ui.end_row();
    });
}

fn render_selected(web_app: &mut WebApp, ui: &mut Ui) {
    match web_app.selected {
        Apps::Welcome => {
            ui.heading("Welcome to my webapp");
        }
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
