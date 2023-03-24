use crate::app::Apps;
use crate::typefast::render_typefast;
use crate::WebApp;
use egui::Ui;
use omni_git::render_git_app;
use omni_randomwalk::render_random_walk;

pub fn render_selection(web_app: &mut WebApp, ui: &mut Ui) {
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

            if ui
                .add(egui::SelectableLabel::new(
                    web_app.selected == Apps::TypeFastApp,
                    "TypeFast App",
                ))
                .clicked()
            {
                web_app.selected = Apps::TypeFastApp;
            }
        });

        ui.end_row();
    });
}

pub fn render_selected(web_app: &mut WebApp, ui: &mut Ui) {
    match web_app.selected {
        Apps::Welcome => render_me(ui),
        Apps::RandomWalks => render_random_walk(&mut web_app.walk_app, ui),
        Apps::GitApps => render_git_app(&mut web_app.git_app, ui),
        Apps::TypeFastApp => render_typefast(&mut web_app.typefast_app, ui),
    }
}

pub fn render_powered_by(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("powered by ");
        ui.hyperlink_to("Silen", "https://github.com/SilenLoc/omnis-vanitas-web");
        ui.label(".");
    });
}

pub fn render_me(ui: &mut Ui) {
    ui.label("Welcome to the apps I made :)");
}
