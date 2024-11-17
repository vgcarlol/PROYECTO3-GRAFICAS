pub struct Camera {
    pub position: [f32; 3], // x, y, z
    pub speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 1000.0],
            speed: 2.0,
        }
    }

    pub fn update(&mut self, target: [f32; 3]) {
        let offset_z = -450.0; // Mantener la distancia adecuada
        let offset_y = 100.0;  // Altura relativa
        let follow_speed = 0.15; // Suavidad del movimiento

        self.position[0] += (target[0] - self.position[0]) * follow_speed;
        self.position[1] += ((target[1] + offset_y) - self.position[1]) * follow_speed;
        self.position[2] += ((target[2] + offset_z) - self.position[2]) * follow_speed;
    }
}
