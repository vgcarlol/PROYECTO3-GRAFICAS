mod camera;
mod planet;
mod renderer;
mod model;
mod nave;
mod skybox;
mod utils;

use crate::renderer::Renderer;
use crate::planet::Planet;
use crate::nave::Nave;
use crate::camera::Camera;

fn main() {
    let mut renderer = Renderer::new("Simulación del Sistema Solar", 800, 600);

    // Configurar los planetas con texturas
    let mut planets = vec![
        Planet::new("Mercurio", "assets/models/sphere.obj", "assets/textures/mercury.png", 10.0, 0.02),
        Planet::new("Venus", "assets/models/sphere.obj", "assets/textures/venus.png", 20.0, 0.015),
        Planet::new("Tierra", "assets/models/sphere.obj", "assets/textures/earth.png", 30.0, 0.01),
        Planet::new("Marte", "assets/models/sphere.obj", "assets/textures/mars.png", 40.0, 0.008),
    ];

    // Configurar la nave
    let mut nave = Nave::new("assets/models/nave.obj");

    // Configurar la cámara
    let mut camera = Camera::new();

    // Ejecutar el renderizador
    renderer.run(&mut planets, &mut nave, &mut camera);
}
