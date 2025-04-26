#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1.0;

        let gravity = 9.8;

        // Calculate x components (no acceleration)
        self.actual_velocity.x = self.init_velocity.x;
        self.actual_position.x = self.init_position.x + self.init_velocity.x * self.time;

        // Calculate y velocity
        let vy = self.init_velocity.y - gravity * self.time;
        self.actual_velocity.y = (vy * 10.0).round() / 10.0;

        // Calculate y position
        let y = self.init_position.y + self.init_velocity.y * self.time - 0.5 * gravity * self.time.powi(2);
        self.actual_position.y = (y * 10.0).round() / 10.0;

        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}