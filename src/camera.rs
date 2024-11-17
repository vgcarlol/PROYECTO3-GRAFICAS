pub struct Camera {
    pub position: [f32; 3], // x, y, z
    pub speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 50.0], // Posición inicial de la cámara
            speed: 2.0,
        }
    }

    pub fn update(&mut self, target: [f32; 3]) {
        // La cámara sigue la posición del objetivo (nave)
        self.position[0] = target[0];
        self.position[1] = target[1];
        self.position[2] = target[2] + 50.0; // Aumentar distancia para perspectiva
    }

    pub fn move_up(&mut self) {
        self.position[1] += self.speed;
    }

    pub fn move_down(&mut self) {
        self.position[1] -= self.speed;
    }

    pub fn move_left(&mut self) {
        self.position[0] -= self.speed;
    }

    pub fn move_right(&mut self) {
        self.position[0] += self.speed;
    }
}
