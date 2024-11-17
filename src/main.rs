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

    let mut nave = Nave::new(
        "assets/models/nave.obj",
        "assets/textures/nave_texture.png",
    );

    let mut planets = vec![
        Planet::new("Mercurio", "assets/models/sphere.obj", "assets/textures/mercury.png", 100.0, 0.15, 50.0),
        Planet::new("Venus", "assets/models/sphere.obj", "assets/textures/venus.png", 200.0, 0.14, 50.0),
        Planet::new("Tierra", "assets/models/sphere.obj", "assets/textures/earth.png", 300.0, 0.13, 50.0),
        Planet::new("Marte", "assets/models/sphere.obj", "assets/textures/mars.png", 400.0, 0.12, 50.0),
    ];
    
    
    let mut camera = Camera::new();

    renderer.run(&mut planets, &mut nave, &mut camera);
}
