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
            position: [0.0, 0.0, 550.0], // Frente a la cámara
            speed: 2.0,
            scale: 50.0, // Ajustamos la escala inicial de la nave
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

        // Aseguramos que la nave esté en el rango visible
        self.position[0] = self.position[0].clamp(-500.0, 500.0);
        self.position[2] = self.position[2].clamp(-500.0, 500.0);

        // Actualizar la posición de la cámara para seguir a la nave
        camera.update(self.position);
    }
}
