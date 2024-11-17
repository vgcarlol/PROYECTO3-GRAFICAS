use obj::Obj;
use std::path::Path;

pub struct Model {
    pub vertices: Vec<[f32; 3]>, // Coordenadas en 3D
}

impl Model {
    pub fn load(path: &str) -> Self {
        let obj = Obj::load(Path::new(path)).expect("Error al cargar el modelo .obj");

        let vertices = obj
            .data
            .position
            .iter()
            .map(|v| [v[0], v[1], v[2]])
            .collect();

        Self { vertices }
    }
}
