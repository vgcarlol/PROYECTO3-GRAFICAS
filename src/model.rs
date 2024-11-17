use obj::Obj;
use std::path::Path;

pub struct Model {
    pub vertices: Vec<[f32; 3]>, // Coordenadas de los vértices
    pub uvs: Vec<[f32; 2]>,      // Coordenadas UV
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

        let uvs = obj
            .data
            .texture
            .iter()
            .map(|uv| [uv[0], uv[1]])
            .collect();

        Self { vertices, uvs }
    }
}
