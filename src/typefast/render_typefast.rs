use crate::typefast::TypeFastApp;
use egui::{RichText, Ui};
use rand::Rng;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Levels {
    Letters,
    Words,
    Sentence,
}

pub fn render_typefast(app: &mut TypeFastApp, ui: &mut Ui) {
    render_selection(app, ui);
    match app.level {
        Levels::Letters => render_letters(app, ui),
        Levels::Words => render_words(ui),
        Levels::Sentence => render_sentence(ui),
    }
}

pub fn render_letters(app: &mut TypeFastApp, ui: &mut Ui) {
    level_settings(ui, &mut app.times);
    render_text(app, ui);
}

pub fn render_words(ui: &mut Ui) {
    ui.label("words");
}

pub fn render_sentence(ui: &mut Ui) {
    ui.label("sentence");
}

pub fn render_text(app: &mut TypeFastApp, ui: &mut Ui) {
    if app.challenge_text.is_empty() {
        app.challenge_text = random_letters(app.times).clone()
    }

    // clear after win
    if app.input_text.eq(&app.challenge_text) {
        app.input_text.clear();
        app.challenge_text.clear();
        app.score = app.score + 1
    }

    let challenge_text = RichText::new(&app.challenge_text.to_string()).size(90.0);
    ui.heading(challenge_text);
    let input_text = RichText::new(&app.input_text.to_string()).size(90.0);
    ui.heading(input_text);
    ui.text_edit_multiline(&mut app.input_text);

    let score = RichText::new(&app.score.to_string()).size(90.0);
    ui.heading(score);
}

fn level_settings(ui: &mut Ui, number: &mut u32) {
    ui.horizontal(|ui| {
        ui.collapsing("Settings", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(number));
                });
            });
        });
    });
}

pub fn render_selection(app: &mut TypeFastApp, ui: &mut Ui) {
    egui::Grid::new("apps_grid_tf").show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui
                .add(egui::SelectableLabel::new(
                    app.level == Levels::Letters,
                    "Letters",
                ))
                .clicked()
            {
                app.level = Levels::Letters;
            }

            if ui
                .add(egui::SelectableLabel::new(
                    app.level == Levels::Words,
                    "Words",
                ))
                .clicked()
            {
                app.level = Levels::Words;
            }

            if ui
                .add(egui::SelectableLabel::new(
                    app.level == Levels::Sentence,
                    "Sentence",
                ))
                .clicked()
            {
                app.level = Levels::Sentence;
            }
        });

        ui.end_row();
    });
}

fn random_letters(max: u32) -> String {
    let mut rng = rand::thread_rng();
    let mut s: String = "".into();
    for _i in 0..max {
        let letter: char = rng.gen_range(b'A'..b'Z') as char;
        let manipulated = big_small_space(letter);
        s.push(manipulated)
    }
    s.trim().to_string()
}

fn big_small_space(letter: char) -> char {
    let mut rng = rand::thread_rng();
    let lr = rng.gen_range(0..3);
    match lr {
        0 => letter,
        1 => {
            let chars: Vec<char> = letter.to_lowercase().to_string().chars().collect();
            chars[0]
        }
        2 => ' ',
        _ => letter,
    }
}
