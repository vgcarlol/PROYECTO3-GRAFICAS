use crate::model::Model;
use image::open;
use image::RgbaImage;
use minifb::Key;
use crate::camera::Camera;

pub struct Nave {
    pub model: Model,
    pub texture: RgbaImage,
    pub position: [f32; 3],
    pub speed: f32,
    pub scale: f32,
}

impl Nave {
    pub fn new(model_path: &str, texture_path: &str) -> Self {
        let texture = open(texture_path)
            .expect("Error al cargar la textura de la nave")
            .to_rgba8();

        Self {
            model: Model::load(model_path),
            texture,
            position: [0.0, 50.0, 0.0], // Posicionada en el plano eclíptico
            speed: 2.0,
            scale: 50.0, // Escala ajustada para ser proporcional
        }
    }

    pub fn update(&mut self, window: &minifb::Window, camera: &mut Camera) {
        let speed_multiplier = 1.0;

        if window.is_key_down(Key::W) {
            self.position[2] -= self.speed * speed_multiplier;
        }
        if window.is_key_down(Key::S) {
            self.position[2] += self.speed * speed_multiplier;
        }
        if window.is_key_down(Key::A) {
            self.position[0] -= self.speed * speed_multiplier;
        }
        if window.is_key_down(Key::D) {
            self.position[0] += self.speed * speed_multiplier;
        }

        self.position[1] = 50.0; // Asegurarse de mantener la nave en el plano eclíptico

        camera.update(self.position);
    }
}
