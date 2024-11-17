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

fn main() {
    let mut renderer = Renderer::new("Simulación del Sistema Solar", 800, 600);

    // Configurar los planetas
    let mut planets = vec![
        Planet::new("Mercurio", "assets/models/sphere.obj", 50.0, 0.02),
        Planet::new("Venus", "assets/models/sphere.obj", 100.0, 0.015),
        Planet::new("Tierra", "assets/models/sphere.obj", 150.0, 0.01),
        Planet::new("Marte", "assets/models/sphere.obj", 200.0, 0.008),
    ];

    // Configurar la nave
    let mut nave = Nave::new("assets/models/nave.obj");

    // Ejecutar el renderizador
    renderer.run(&mut planets, &mut nave);
}
