use eframe::App;
use egui::Ui;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
///
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WebApp {
    // Example stuff:
    label: String,
    selected: Apps,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    pub value: f32,
}

impl Default for WebApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            selected: Apps::Welcome,
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
            label,
            selected,
            value,
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("");

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Apps");
                });
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(selected, Apps::Welcome, "Welcome");
                        ui.selectable_value(selected, Apps::RandomWalks, "Random Walks");
                        ui.selectable_value(selected, Apps::Second, "Second");
                        ui.selectable_value(selected, Apps::Third, "Third");
                    });
                ui.end_row();
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("Silen", "https://github.com/SilenLoc/omnis-vanitas-web");
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match selected {
                Apps::Welcome => {
                    ui.heading("Welcome to my webapp");
                }
                Apps::RandomWalks => {
                    ui.heading("Random Walks");
                }
                Apps::Second => place_holder(ui),
                Apps::Third => place_holder(ui),
            }

            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
            });
        }
    }
}

fn place_holder(ui: &mut Ui) {
    ui.heading("Place Holder");
}
