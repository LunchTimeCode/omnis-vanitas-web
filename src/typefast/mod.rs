use crate::typefast::render_typefast::Levels;

mod render_typefast;
pub use render_typefast::render_typefast;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TypeFastApp {
    front_text: String,
    input_text: String,
    challenge_text: String,
    times: u32,
    level: Levels,
    score: u32,
}

impl Default for TypeFastApp {
    fn default() -> Self {
        Self {
            front_text: "".into(),
            input_text: "".into(),
            challenge_text: "".into(),
            times: 12,
            level: Levels::Letters,
            score: 0,
        }
    }
}
