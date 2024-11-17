use minifb::{Key, Window, WindowOptions};
use crate::planet::Planet;
use crate::nave::Nave;
use crate::camera::Camera;
use crate::skybox::render_skybox;

pub struct Renderer {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height];
        let window = Window::new(title, width, height, WindowOptions::default())
            .unwrap_or_else(|_| panic!("Error al crear la ventana"));
        Self { window, buffer, width, height }
    }

    pub fn run(&mut self, planets: &mut [Planet], nave: &mut Nave, camera: &mut Camera) {
        let mut time = 0.0;

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.buffer.fill(0);

            // Renderizar el skybox con la posición de la cámara
            render_skybox(&mut self.buffer, self.width, self.height, "assets/textures/skybox.png", camera.position);

            // Renderizar planetas
            for planet in planets.iter_mut() {
                planet.update(time);
                self.draw_model(&planet.model, planet.position, 0x00FF00);
            }

            // Actualizar y renderizar la nave
            nave.update(&self.window);
            camera.update(nave.position); // La cámara sigue la nave
            self.draw_model(&nave.model, nave.position, 0xFF0000);

            // Actualizar la ventana
            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();

            time += 0.01;
        }
    }

    fn draw_model(&mut self, model: &crate::model::Model, position: [f32; 3], color: u32) {
        for vertex in &model.vertices {
            let projected = crate::utils::project_vertex(*vertex, self.width, self.height, position);
            if projected.0 < self.width && projected.1 < self.height {
                self.buffer[projected.1 * self.width + projected.0] = color;
            }
        }
    }
}
