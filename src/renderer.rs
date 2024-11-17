use minifb::{Key, Window, WindowOptions};
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
    
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
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
    
            // Actualizar y renderizar los planetas
            for planet in planets.iter_mut() {
                planet.update(time);
                self.draw_model_with_texture(
                    &planet.model,
                    planet.position,
                    &planet.texture,
                    5.0,
                );
            }
    
            // Actualizar y renderizar la nave
            nave.update(&self.window, camera);
            self.draw_model_with_texture(
                &nave.model,
                nave.position,
                &nave.texture,
                1.0,
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
    ) {
        for vertex in &model.vertices {
            // Escalamos el vértice
            let scaled_vertex = [
                vertex[0] * scale,
                vertex[1] * scale,
                vertex[2] * scale,
            ];
    
            // Proyectamos el vértice al espacio de la pantalla
            let projected =
                crate::utils::project_vertex(scaled_vertex, self.width, self.height, position);
    
            // Solo renderizamos si está dentro de los límites de la pantalla
            if projected.0 < self.width && projected.1 < self.height {
                let x = (projected.0 as f32 / self.width as f32 * texture.width() as f32) as u32;
                let y = (projected.1 as f32 / self.height as f32 * texture.height() as f32) as u32;
    
                // Garantizamos que las coordenadas estén en rango
                let pixel = texture.get_pixel(x % texture.width(), y % texture.height());
                let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
                self.buffer[projected.1 * self.width + projected.0] = color;
            }
        }
    }
    
    
    
}
