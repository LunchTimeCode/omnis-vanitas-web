use std::collections::HashMap;

use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BoidApp {
    settings: BoidsSettings,
    #[serde(skip)]
    field: Field,
}

impl Default for BoidApp {
    fn default() -> Self {
        Self {
            settings: BoidsSettings::default(),
            field: Field {
                state: HashMap::new(),
            },
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
struct BoidsSettings {
    amount: u32,
}

struct Field {
    state: HashMap<u16, Boid>,
}

impl Field {
    pub fn move_all_once(&mut self) -> Result<String, String> {
        let ids: Vec<u16> = self.state.keys().cloned().into_iter().collect();
        for id in ids {
            let _ = self.move_boid(&id);
        }
        Ok("moved all boids".to_owned())
    }

    pub fn move_boid(&mut self, id: &u16) -> Result<String, String> {
        let boid_to_move = self
            .state
            .get_mut(id)
            .map_or(Err("could not find boid"), Ok)?;
        let id = boid_to_move.id;

        //apply rules and move boid depending on the others, adding a boid etc.
        boid_to_move.position = [1.0, 1.0];
        let _ = self.add_boid(1, [1.0, 1.0]);

        Ok(format!["moved boid {}", id])
    }

    pub fn add_boid(&mut self, id: u16, at: [f64; 2]) -> Result<String, String> {
        self.state
            .insert(id, Boid::new(id, at))
            .map_or(Err("could not position boid".to_owned()), |_b| {
                Ok("placed boid".to_owned())
            })
    }
}

struct Boid {
    id: u16,
    position: [f64; 2],
}

impl Boid {
    pub fn new(id: u16, position: [f64; 2]) -> Self {
        Self { id, position }
    }
}

pub fn render_boids(app: &mut BoidApp, _ui: &mut Ui) {
    let _ = app.field.move_all_once();
}
