use obj::Obj;
use std::path::Path;


pub struct Model {
    pub vertices: Vec<[f32; 3]>, // Coordenadas de los vértices
    pub uvs: Vec<[f32; 2]>,      // Coordenadas UV
    pub faces: Vec<[usize; 3]>,  // Índices de los vértices que forman cada cara
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

        // Extraer las caras (triángulos) del archivo OBJ
        let faces = obj
            .data
            .objects
            .iter()
            .flat_map(|o| &o.groups)
            .flat_map(|g| &g.polys)
            .filter_map(|poly| {
                if poly.0.len() == 3 { // Asegurar que sean triángulos
                    Some([
                        poly.0[0].0, // Índice del primer vértice
                        poly.0[1].0, // Índice del segundo vértice
                        poly.0[2].0, // Índice del tercer vértice
                    ])
                } else {
                    None
                }
            })
            .collect();

        Self { vertices, uvs, faces }
    }
}
