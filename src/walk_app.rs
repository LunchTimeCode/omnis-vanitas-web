use egui::{
    plot::{Line, Plot, PlotPoints},
    vec2, Ui,
};

use crate::{random_cordinates_one_dim, random_cordinates_two_dim};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WalkApp {
    selected_random_walks: RandomWalkApps,

    #[serde(skip)]
    walk_plot_state: Vec<[f64; 2]>,

    steps: u64,
}

impl Default for WalkApp {
    fn default() -> Self {
        Self {
            selected_random_walks: RandomWalkApps::OneDimension,
            walk_plot_state: vec![[0.0, 0.0]],
            steps: 0,
        }
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RandomWalkApps {
    OneDimension,
    TwoDimensons,
}

pub fn render_random_walk(walk_app: &mut WalkApp, ui: &mut Ui) {
    egui::Grid::new("random_walk grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Random Walks");

            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", walk_app.selected_random_walks))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut walk_app.selected_random_walks,
                        RandomWalkApps::OneDimension,
                        "One dimensions",
                    );
                    ui.selectable_value(
                        &mut walk_app.selected_random_walks,
                        RandomWalkApps::TwoDimensons,
                        "Two dimensions",
                    );
                });
        });

        ui.end_row();

        ui.vertical(|ui| match walk_app.selected_random_walks {
            RandomWalkApps::OneDimension => render_random_walk_one_dimension(
                ui,
                &mut walk_app.walk_plot_state,
                &mut walk_app.steps,
            ),
            RandomWalkApps::TwoDimensons => render_random_walk_two_dimension(
                ui,
                &mut walk_app.walk_plot_state,
                &mut walk_app.steps,
            ),
        });

        ui.end_row();
    });
}

fn render_random_walk_one_dimension(
    ui: &mut Ui,
    walk_plot_state: &mut Vec<[f64; 2]>,
    steps: &mut u64,
) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        plot_actions(ui, walk_plot_state, steps, 1);
        ui.end_row();
        plot_settings(ui, steps);
        ui.end_row();

        ui.horizontal(|ui| {
            let sin: PlotPoints = PlotPoints::from(walk_plot_state.clone());

            let line = Line::new(sin);
            Plot::new("my_plot")
                .view_aspect(2.0)
                .min_size(vec2(1000.0, 250.0))
                .show(ui, |plot_ui| plot_ui.line(line));
        });
        ui.end_row();
    });
}

fn render_random_walk_two_dimension(
    ui: &mut Ui,
    walk_plot_state: &mut Vec<[f64; 2]>,
    steps: &mut u64,
) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        plot_actions(ui, walk_plot_state, steps, 2);
        ui.end_row();
        plot_settings(ui, steps);
        ui.end_row();
        ui.horizontal(|ui| {
            let sin: PlotPoints = PlotPoints::from(walk_plot_state.clone());

            let line = Line::new(sin);
            Plot::new("my_plot")
                .view_aspect(2.0)
                .min_size(vec2(1000.0, 250.0))
                .show(ui, |plot_ui| plot_ui.line(line));
        });
        ui.end_row();
    });
}

fn plot_actions(
    ui: &mut Ui,
    walk_plot_state: &mut Vec<[f64; 2]>,
    steps: &mut u64,
    dimensions: u128,
) {
    ui.horizontal(|ui| {
        ui.collapsing("Actions", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("walk").clicked() {
                        let cords = match dimensions {
                            1 => random_cordinates_one_dim(*steps),
                            2 => random_cordinates_two_dim(*steps),
                            _ => vec![],
                        };

                        *walk_plot_state = cords;
                    };
                });
            });
        });
    });
}

fn plot_settings(ui: &mut Ui, steps: &mut u64) {
    ui.horizontal(|ui| {
        ui.collapsing("Settings", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(steps));
                    ui.label("steps")
                });
            });
        });
    });
}
