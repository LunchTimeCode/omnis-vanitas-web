use egui::Ui;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WelcomeApp {
    selected: WebPage,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum WebPage {
    Me,
    CV,
    Blog,
}

impl Default for WelcomeApp {
    fn default() -> Self {
        Self {
            selected: WebPage::Me,
        }
    }
}

pub fn render_welcome(welcome_app: &mut WelcomeApp, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.horizontal_top(|ui| {
            if ui
                .add(egui::SelectableLabel::new(
                    welcome_app.selected == WebPage::Me,
                    "Me",
                ))
                .clicked()
            {
                welcome_app.selected = WebPage::Me;
            }

            if ui
                .add(egui::SelectableLabel::new(
                    welcome_app.selected == WebPage::CV,
                    "CV",
                ))
                .clicked()
            {
                welcome_app.selected = WebPage::CV;
            }

            if ui
                .add(egui::SelectableLabel::new(
                    welcome_app.selected == WebPage::Blog,
                    "Blog",
                ))
                .clicked()
            {
                welcome_app.selected = WebPage::Blog;
            }
            ui.end_row();
        });

        ui.horizontal_centered(|ui| {
            match welcome_app.selected {
                WebPage::Me => render_me(welcome_app, ui),
                WebPage::CV => render_cv(welcome_app, ui),
                WebPage::Blog => render_blog(welcome_app, ui),
            }
            ui.end_row();
        });
    });
}

pub fn render_me(_: &mut WelcomeApp, ui: &mut Ui) {
    ui.label("me");
}

pub fn render_cv(_: &mut WelcomeApp, ui: &mut Ui) {
    ui.label("cv");
}

pub fn render_blog(_: &mut WelcomeApp, ui: &mut Ui) {
    ui.label("blog");
}
