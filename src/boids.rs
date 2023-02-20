use std::collections::HashMap;

use egui::{
    plot::{Line, Plot, PlotPoints},
    vec2, Ui,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BoidApp {
    settings: BoidsSettings,
    #[serde(skip)]
    field: Field,
    #[serde(skip)]
    boid_plot_state: Vec<[f64; 2]>,
}

impl Default for BoidApp {
    fn default() -> Self {
        Self {
            settings: BoidsSettings::default(),
            field: Field {
                state: HashMap::new(),
            },
            boid_plot_state: vec![],
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

#[derive(Clone)]
struct Boid {
    id: u16,
    position: [f64; 2],
}

impl Boid {
    pub fn new(id: u16, position: [f64; 2]) -> Self {
        Self { id, position }
    }
}

pub fn render_boids(app: &mut BoidApp, ui: &mut Ui) {
    let _ = app.field.move_all_once();

    render_plot_via_boid_field(app);

    ui.horizontal(|ui| {
        let sin: PlotPoints = PlotPoints::from(app.boid_plot_state.clone());

        let line = Line::new(sin);
        Plot::new("boids_plot")
            .view_aspect(2.0)
            .min_size(vec2(1000.0, 250.0))
            .show(ui, |plot_ui| plot_ui.line(line));
    });
}

fn render_plot_via_boid_field(app: &mut BoidApp) {
    let to_render: Vec<Boid> = app.field.state.values().cloned().collect();

    let new_render_state: &mut Vec<[f64; 2]> = &mut vec![];

    for boid in to_render {
        let state = render_boid(boid);
        for point in state {
            new_render_state.push(point);
        }
    }

    app.boid_plot_state = new_render_state.to_vec()
}

fn render_boid(boid: Boid) -> Vec<[f64; 2]> {
    let boid_pos = boid.position;

    let to_right = [(boid_pos[0] + 1.0), (boid_pos[1])];

    let to_up = [(boid_pos[0]), (boid_pos[1] + 1.0)];

    let to_left = [(boid_pos[0] - 1.0), (boid_pos[1])];

    let to_down = [(boid_pos[0]), (boid_pos[1] - 1.0)];

    vec![to_right, to_up, boid_pos, to_left, to_down]
}
