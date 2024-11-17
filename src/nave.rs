use crate::model::Model;
use minifb::Key;

pub struct Nave {
    pub model: Model,
    pub position: [f32; 3],
    pub speed: f32,
}

impl Nave {
    pub fn new(model_path: &str) -> Self {
        Self {
            model: Model::load(model_path),
            position: [0.0, 0.0, 50.0],
            speed: 2.0,
        }
    }

    pub fn update(&mut self, window: &minifb::Window) {
        if window.is_key_down(Key::W) {
            self.position[1] -= self.speed;
        }
        if window.is_key_down(Key::S) {
            self.position[1] += self.speed;
        }
        if window.is_key_down(Key::A) {
            self.position[0] -= self.speed;
        }
        if window.is_key_down(Key::D) {
            self.position[0] += self.speed;
        }
        if window.is_key_down(Key::Up) {
            self.position[2] -= self.speed;
        }
        if window.is_key_down(Key::Down) {
            self.position[2] += self.speed;
        }
    }
}
