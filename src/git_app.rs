use eframe::epaint::ahash::HashMap;
use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GitApp {
    selected_gits_app: GitApps,
    #[serde(skip)]
    choosen: ChoosenTagSymbols,
    #[serde(skip)]
    choosen_other: ChoosenTagSymbols,
}

impl Default for GitApp {
    fn default() -> Self {
        Self {
            selected_gits_app: GitApps::TagDiffWeb,
            choosen: ChoosenTagSymbols::default(),
            choosen_other: ChoosenTagSymbols::default(),
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
            GitApps::TagDiffWeb => render_web_diff(app, ui),
            GitApps::TagDiffCommand => render_tag_diff(app, ui),
        });

        ui.end_row();
    });
}

fn render_tag_diff(app: &mut GitApp, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        actions(app, ui);
        ui.end_row();
        settings(app, ui);
        ui.label(&app.choosen.get_choosen_symbol_chain());
        ui.end_row();
    });
}

fn render_web_diff(app: &mut GitApp, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        actions(app, ui);
        ui.end_row();
        settings(app, ui);
        ui.end_row();

        ui.end_row();
    });
}

fn actions(app: &mut GitApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.collapsing("Actions", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("create").clicked() {
                        app.choosen.choose(TagSymbol::new(1, "prefix", "something"));
                        app.choosen.choose(TagSymbol::new(2, "delitmiter", "/"));
                        app.choosen
                            .choose(TagSymbol::new(3, "version", "someversion"));

                        app.choosen_other
                            .choose(TagSymbol::new(1, "prefix2", "something"));
                        app.choosen_other
                            .choose(TagSymbol::new(2, "delitmiter22", "/"));
                        app.choosen_other
                            .choose(TagSymbol::new(3, "version2", "someversion"));
                    };
                });
            });
        });
    });
}

fn settings(app: &mut GitApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.collapsing("Settings", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                        let mut cloned = app.choosen.get_choosen_symbols();
                        for choosen in &mut cloned {
                            ui.text_edit_singleline(&mut choosen.symbol);
                            ui.add_space(2.0);
                        }
                        for new in cloned.clone() {
                            app.choosen.choose(new)
                        }

                        let mut cloned_other = app.choosen_other.get_choosen_symbols();
                        for choosen_other in &mut cloned_other {
                            ui.text_edit_singleline(&mut choosen_other.symbol);
                            ui.add_space(2.0);
                        }
                        for new in cloned_other.clone() {
                            app.choosen_other.choose(new)
                        }
                
                });
            });
        })
    });
}

#[derive(Clone, PartialEq)]
pub struct TagSymbol {
    order: u16,
    name: String,
    symbol: String,
}

impl TagSymbol {
    fn new(order: u16, name: &str, symbol: &str) -> TagSymbol {
        TagSymbol {
            order,
            name: name.to_owned(),
            symbol: symbol.to_owned(),
        }
    }
}

#[derive(Default)]
pub struct ChoosenTagSymbols {
    pub symbols: HashMap<String, TagSymbol>,
}

impl ChoosenTagSymbols {
    fn choose(&mut self, choose: TagSymbol) {
        self.symbols.insert(choose.name.clone(), choose);
    }

    fn get_choosen_symbols(&mut self) -> Vec<TagSymbol> {
        let mut values: Vec<TagSymbol> = self.symbols.values().cloned().collect();
        values.sort_by(|a, b| a.order.cmp(&b.order));
        values.into_iter().collect()
    }

    fn get_choosen_symbol_chain(&self) -> String {
        let mut values: Vec<TagSymbol> = self.symbols.values().cloned().collect();
        values.sort_by(|a, b| a.order.cmp(&b.order));
        let strings: Vec<String> = values.into_iter().map(|tuple| tuple.symbol).collect();
        strings.concat()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_symbols_in_correct_order() {
        let mut chooser = ChoosenTagSymbols::default();
        chooser.choose(TagSymbol::new(1, "prefix", "something"));
        chooser.choose(TagSymbol::new(2, "delitmiter", "/"));
        chooser.choose(TagSymbol::new(3, "version", "someversion"));

        let tag = chooser.get_choosen_symbol_chain();
        assert_eq!(tag, "something/version/")
    }
}
