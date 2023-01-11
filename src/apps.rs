
    use crate::WebApp;

        pub fn update_apps(&mut app: WebApp, ctx: &egui::Context)-> WebApp {

            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Side Panel");

                ui.add(egui::Slider::new(&mut app.value, 0.0..=10.0).text("value"));
                if ui.button("Increment").clicked() {
                    app.value += 1.0;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("powered by ");
                        ui.hyperlink_to("Silen", "https://github.com/SilenLoc/omnis-vanitas-web");
                        ui.label(".");
                    });
                });
            });
            app 
        }

