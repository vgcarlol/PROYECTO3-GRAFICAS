use crate::model::Model;
use image::open;
use image::RgbaImage;

pub struct Planet {
    pub name: String,
    pub model: Model,
    pub texture: RgbaImage,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub position: [f32; 3],
}

impl Planet {
    pub fn new(
        name: &str,
        model_path: &str,
        texture_path: &str,
        orbit_radius: f32,
        orbit_speed: f32,
    ) -> Self {
        let texture = open(texture_path)
            .expect(&format!("Error al cargar la textura del planeta: {}", texture_path))
            .to_rgba8();

        Self {
            name: name.to_string(),
            model: Model::load(model_path),
            texture,
            orbit_radius,
            orbit_speed,
            position: [orbit_radius, 0.0, 0.0], // Ajustar posición inicial
        }
    }

    pub fn update(&mut self, time: f32) {
        self.position[0] = self.orbit_radius * (time * self.orbit_speed).cos();
        self.position[2] = self.orbit_radius * (time * self.orbit_speed).sin();
        self.position[1] = 0.0; // Mantener en el plano horizontal
    }
}
