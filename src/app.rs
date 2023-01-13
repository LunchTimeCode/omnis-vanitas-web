use egui::{
    plot::{Line, Plot, PlotPoints},
    vec2, Ui,
};

use crate::{
    random_cordinates::{random_cordinates_one_dim},
    random_cordinates_two_dim,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
///
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WebApp {
    // Example stuff:
    selected: Apps,

    selected_random_walks: RandomWalkApps,
    walk_plot_state: Vec<[f64; 2]>,
    from: f64,
    to: f64,
    steps: u64



}

impl Default for WebApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            selected: Apps::Welcome,
            selected_random_walks: RandomWalkApps::OneDimension,
            walk_plot_state: vec![[0.0, 0.0]],
            from:0.0,
            to: 0.0,
            steps: 0,
        }
    }
}

impl WebApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
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
            from,
            to,
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
                    render_random_walk(ui, &mut self.selected_random_walks, walk_plot_state, from, to, steps)
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
    from: &mut f64, to: &mut f64, steps: &mut u64,
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
            RandomWalkApps::OneDimension => render_random_walk_one_dimension(ui, walk_plot_state, from, to, steps),
            RandomWalkApps::TwoDimensons => render_random_walk_two_dimension(ui, walk_plot_state,from, to, steps),
        });

        ui.end_row();
    });
}

fn render_random_walk_two_dimension(ui: &mut Ui, walk_plot_state: &mut Vec<[f64; 2]>,from: &mut f64, to: &mut f64, steps: &mut u64,) {

    let cordinates = random_cordinates_two_dim(*from, *to, *steps);

    ui.vertical(|ui| {
        ui.add_space(20.0);
        plot_actions(ui, walk_plot_state,cordinates);
        ui.end_row();
        plot_settings(ui, from,  to, steps);
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

fn render_random_walk_one_dimension(ui: &mut Ui, walk_plot_state: &mut Vec<[f64; 2]>,from: &mut f64, to: &mut f64, steps: &mut u64,) {

    let cordinates = random_cordinates_one_dim(*from, *to, *steps);

    ui.vertical(|ui| {
        ui.add_space(20.0);
        plot_actions(ui, walk_plot_state, cordinates);
        ui.end_row();
        plot_settings(ui, from,  to, steps);
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
    cordinates: Vec<[f64; 2]>
) {
    ui.horizontal(|ui| {
        ui.collapsing("Actions", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("random plot").clicked() {
                        *walk_plot_state = cordinates;
                    };
                });
            });
        });
    });
}

fn plot_settings(
    ui: &mut Ui,
    from: &mut f64, to: &mut f64, steps: &mut u64,
) {
    ui.horizontal(|ui| {
        ui.collapsing("Settings", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {


                    ui.add(egui::Slider::new(from, -10000.0..=*to -1.0).text("from"));
                    ui.add(egui::Slider::new(to, -*from -1.0..=-10000.0).text("to"));
                    ui.add(egui::DragValue::new(steps));
                    ui.label("steps")
                });
            });
        });
    });
}
