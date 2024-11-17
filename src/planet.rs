use crate::model::Model;

pub struct Planet {
    pub name: String,
    pub model: Model,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub position: [f32; 3],
}

impl Planet {
    pub fn new(name: &str, model_path: &str, orbit_radius: f32, orbit_speed: f32) -> Self {
        Self {
            name: name.to_string(),
            model: Model::load(model_path),
            orbit_radius,
            orbit_speed,
            position: [orbit_radius, 0.0, 0.0],
        }
    }

    pub fn update(&mut self, time: f32) {
        self.position[0] = self.orbit_radius * (time * self.orbit_speed).cos();
        self.position[2] = self.orbit_radius * (time * self.orbit_speed).sin();
    }
}
