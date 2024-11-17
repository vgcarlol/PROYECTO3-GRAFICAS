pub struct Camera {
    pub position: [f32; 3], // x, y, z
    pub speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 100.0], // Distancia inicial desde el plano
            speed: 5.0,
        }
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

    pub fn zoom_in(&mut self) {
        self.position[2] -= self.speed;
    }

    pub fn zoom_out(&mut self) {
        self.position[2] += self.speed;
    }
}
