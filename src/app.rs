use egui::{
    plot::{Line, Plot, PlotPoints},
    vec2, Ui,
};

use crate::{random_cordinates::random_cordinates_one_dim, random_cordinates_two_dim};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WebApp {
    selected: Apps,

    selected_random_walks: RandomWalkApps,

    #[serde(skip)]
    walk_plot_state: Vec<[f64; 2]>,

    steps: u64,
}

impl Default for WebApp {
    fn default() -> Self {
        Self {
            selected: Apps::Welcome,
            selected_random_walks: RandomWalkApps::OneDimension,
            walk_plot_state: vec![[0.0, 0.0]],
            steps: 0,
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
    Second,
    Third,
}

impl eframe::App for WebApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            selected,
            selected_random_walks: _,
            walk_plot_state,
            steps,
        } = self;

        egui::TopBottomPanel::top("top_grid").show(ctx, |ui| {
            ui.add_space(3.0);
            egui::Grid::new("apps_grid").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Applications");
                    if ui.button("Welcome").clicked() {
                        *selected = Apps::Welcome;
                    };
                    if ui.button("Random Walks").clicked() {
                        *selected = Apps::RandomWalks;
                    };
                });

                ui.end_row();
            });

            ui.add_space(3.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);

            match selected {
                Apps::Welcome => {
                    ui.heading("Welcome to my webapp");
                }
                Apps::RandomWalks => {
                    render_random_walk(ui, &mut self.selected_random_walks, walk_plot_state, steps)
                }
                Apps::Second => place_holder(ui),
                Apps::Third => place_holder(ui),
            }

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("powered by ");
                ui.hyperlink_to("Silen", "https://github.com/SilenLoc/omnis-vanitas-web");
                ui.label(".");
            });

            egui::warn_if_debug_build(ui);
        });
    }
}

fn place_holder(ui: &mut Ui) {
    ui.heading("Place Holder");
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum RandomWalkApps {
    OneDimension,
    TwoDimensons,
}

fn render_random_walk(
    ui: &mut Ui,
    selected_random_walks: &mut RandomWalkApps,
    walk_plot_state: &mut Vec<[f64; 2]>,
    steps: &mut u64,
) {
    egui::Grid::new("random_walk grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Random Walks");

            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", selected_random_walks))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        selected_random_walks,
                        RandomWalkApps::OneDimension,
                        "One dimensions",
                    );
                    ui.selectable_value(
                        selected_random_walks,
                        RandomWalkApps::TwoDimensons,
                        "Two dimensions",
                    );
                });
        });

        ui.end_row();

        ui.vertical(|ui| match selected_random_walks {
            RandomWalkApps::OneDimension => {
                render_random_walk_one_dimension(ui, walk_plot_state, steps)
            }
            RandomWalkApps::TwoDimensons => {
                render_random_walk_two_dimension(ui, walk_plot_state, steps)
            }
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
