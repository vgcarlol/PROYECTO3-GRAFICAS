use minifb::{Window, WindowOptions};
use crate::planet::Planet;
use crate::nave::Nave;
use crate::skybox::render_skybox;
use crate::camera::Camera;

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

            camera.update(nave.position);
            render_skybox(&mut self.buffer, self.width, self.height, "assets/textures/skybox.png", camera.position);

            for planet in planets.iter_mut() {
                planet.update(time);
                self.draw_model_with_texture(
                    &planet.model,
                    planet.position,
                    &planet.texture,
                    planet.scale, // Usamos la escala definida en el planeta
                    camera.position,
                );
            }
            

            nave.update(&self.window, camera);
            self.draw_model_with_texture(
                &nave.model,
                nave.position,
                &nave.texture,
                nave.scale,
                camera.position,
            );

            self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
            time += 0.016;
        }
    }

    fn fill_triangle(
        &mut self,
        vertices: &[(usize, usize); 3],
        color: u32,
    ) {
        let mut sorted = *vertices;
        sorted.sort_by(|a, b| a.1.cmp(&b.1));

        let (x0, y0) = sorted[0];
        let (x1, y1) = sorted[1];
        let (x2, y2) = sorted[2];

        for y in y0..=y2 {
            let x_start = if y <= y1 {
                Self::lerp(x0, x1, y, y0, y1)
            } else {
                Self::lerp(x1, x2, y, y1, y2)
            };
            let x_end = Self::lerp(x0, x2, y, y0, y2);

            for x in x_start.min(x_end)..=x_start.max(x_end) {
                if x < self.width && y < self.height {
                    let index = y * self.width + x;
                    if index < self.buffer.len() {
                        self.buffer[index] = color;
                    }
                }
            }
        }
    }

    fn lerp(x0: usize, x1: usize, y: usize, y0: usize, y1: usize) -> usize {
        if y1 == y0 {
            x0
        } else {
            let x0 = x0 as isize;
            let x1 = x1 as isize;
            let y = y as isize;
            let y0 = y0 as isize;
            let y1 = y1 as isize;

            let result = x0 + (x1 - x0) * (y - y0) / (y1 - y0);
            result.max(0) as usize
        }
    }

    fn draw_model_with_texture(
        &mut self,
        model: &crate::model::Model,
        position: [f32; 3],
        texture: &image::RgbaImage,
        scale: f32,
        camera_position: [f32; 3],
    ) {
        for face in model.faces.iter() {
            if face.iter().any(|&idx| idx >= model.vertices.len() || idx >= model.uvs.len()) {
                continue;
            }
    
            let v0 = model.vertices[face[0]];
            let v1 = model.vertices[face[1]];
            let v2 = model.vertices[face[2]];
    
            let vertices = [v0, v1, v2].map(|v| [
                v[0] * scale + position[0],
                v[1] * scale + position[1],
                v[2] * scale + position[2],
            ]);
    
            let projected = vertices.map(|v| crate::utils::project_vertex(v, self.width, self.height, camera_position));
    
            if projected.iter().all(|&(x, y)| x < self.width && y < self.height) {
                let uvs = face.iter().map(|&idx| model.uvs[idx]).collect::<Vec<_>>();
                let color = self.get_texture_color(&uvs, texture);
    
                self.fill_triangle(&projected, color);
            }
        }
    }
    

    fn get_texture_color(&self, uvs: &Vec<[f32; 2]>, texture: &image::RgbaImage) -> u32 {
        if uvs.is_empty() {
            return 0xFFFFFF;
        }

        let u = (uvs[0][0] * texture.width() as f32).clamp(0.0, texture.width() as f32 - 1.0) as u32;
        let v = (uvs[0][1] * texture.height() as f32).clamp(0.0, texture.height() as f32 - 1.0) as u32;

        if u >= texture.width() || v >= texture.height() {
            return 0xFFFFFF;
        }

        let pixel = texture.get_pixel(u, v);
        ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32)
    }
}
