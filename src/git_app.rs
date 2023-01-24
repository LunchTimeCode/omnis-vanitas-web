use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GitApp {
    selected_gits_app: GitApps,
}

impl Default for GitApp {
    fn default() -> Self {
        Self {
            selected_gits_app: GitApps::TagDiffWeb,
        }
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GitApps {
    TagDiffWeb,
    TagDiffCommand,
}

pub fn render_git_app(app: &mut GitApp, ui: &mut Ui) {
    egui::Grid::new("git_apps_grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Git Apps");

            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", app.selected_gits_app))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.selected_gits_app,
                        GitApps::TagDiffWeb,
                        "Web Diff",
                    );
                    ui.selectable_value(
                        &mut app.selected_gits_app,
                        GitApps::TagDiffCommand,
                        "Command Diff",
                    );
                });
        });

        ui.end_row();

        ui.vertical(|ui| match app.selected_gits_app {
            GitApps::TagDiffWeb => render_web_diff(ui),
            GitApps::TagDiffCommand => render_tag_diff(ui),
        });

        ui.end_row();
    });
}

fn render_tag_diff(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        actions(ui);
        ui.end_row();
        settings(ui);
        ui.end_row();

        ui.end_row();
    });
}

fn render_web_diff(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        actions(ui);
        ui.end_row();
        settings(ui);
        ui.end_row();

        ui.end_row();
    });
}

fn actions(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.collapsing("Actions", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("create").clicked() {};
                });
            });
        });
    });
}

fn settings(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.collapsing("Settings", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    ui.label("pattern");
                });
            });
        });
    });
}
