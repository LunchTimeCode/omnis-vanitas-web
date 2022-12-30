use translation_server_client_silen::ready;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct TemplateApp {
    open: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            open: false,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            open: _,
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            if ui.button("open a window").clicked() {
                self.open = true;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("me", "https://github.com/SilenLoc");
                    ui.label(".");
                });
            });
        });


        if self.open {
            egui::Window::new("Window").show(ctx, |ui| {
                if ui.button("Close").clicked() {
                    self.open = false;
                }
            });
        }

    
    }
}

