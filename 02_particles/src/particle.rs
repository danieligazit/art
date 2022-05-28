
use nannou::prelude::*;

pub struct Particle {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    life_span: f32,
}

impl Particle {
    pub fn new(l: Point2) -> Self {
        Particle {
            acceleration: vec2(0.0, 0.05),
            velocity: vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0),
            position: l,
            life_span: 255.0,
        }
    }

    // Method to update position
    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.life_span -= 1.0;
    }

    // Method to display
    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .w_h(12.0, 12.0)
            .rgba(0.5, 0.5, 0.5, self.life_span / 255.0)
            // .stroke(rgba(0.0, 0.0, 0.0, self.life_span / 255.0))
            .stroke_weight(0.0);
    }

    // Is the particle still useful?
    pub fn is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}