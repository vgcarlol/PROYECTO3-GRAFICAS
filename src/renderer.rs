use minifb::{Window, WindowOptions};
use crate::planet::Planet;
use crate::nave::Nave;
use crate::skybox::render_skybox;
use crate::camera::Camera;
use image::RgbaImage;

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

    pub fn run(
        &mut self,
        planets: &mut [Planet],
        nave: &mut Nave,
        camera: &mut Camera,
    ) {
        let mut time = 0.0;

        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {
            self.buffer.fill(0);

            // Actualizar la posición de la cámara para seguir a la nave
            camera.update(nave.position);

            // Renderizar el Skybox
            render_skybox(
                &mut self.buffer,
                self.width,
                self.height,
                "assets/textures/skybox.png",
                camera.position,
            );

            // Renderizar planetas
            for planet in planets.iter_mut() {
                planet.update(time);
                self.draw_model_with_texture(
                    &planet.model,
                    planet.position,
                    &planet.texture,
                    1.0,
                    camera.position,
                );
            }

            // Renderizar la nave
            nave.update(&self.window, camera);
            self.draw_model_with_texture(
                &nave.model,
                nave.position,
                &nave.texture,
                1.0,
                camera.position,
            );

            // Actualizar la ventana
            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();

            time += 0.01;
        }
    }

    fn draw_model_with_texture(
        &mut self,
        model: &crate::model::Model,
        position: [f32; 3],
        texture: &RgbaImage,
        scale: f32,
        camera_position: [f32; 3],
    ) {
        for (vertex, uv) in model.vertices.iter().zip(&model.uvs) {
            let scaled_vertex = [
                vertex[0] * scale + position[0],
                vertex[1] * scale + position[1],
                vertex[2] * scale + position[2],
            ];
    
            let projected = crate::utils::project_vertex(
                scaled_vertex,
                self.width,
                self.height,
                camera_position,
            );
    
            if projected.0 < self.width && projected.1 < self.height {
                let u = (uv[0] * texture.width() as f32).clamp(0.0, texture.width() as f32 - 1.0)
                    as u32;
                let v = (uv[1] * texture.height() as f32).clamp(0.0, texture.height() as f32 - 1.0)
                    as u32;
    
                let pixel = texture.get_pixel(u, v);
                let color = ((pixel[0] as u32) << 16)
                    | ((pixel[1] as u32) << 8)
                    | (pixel[2] as u32);
    
                self.buffer[projected.1 * self.width + projected.0] = color;
            }
        }
    }
    
}
