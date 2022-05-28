use nannou::prelude::*;
use crate::particle::Particle;

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        let particles: Vec<Particle> = Vec::new();
        ParticleSystem { particles }
    }

    pub fn add_particle(&mut self, pos: Point2) {
        self.particles.push(Particle::new(pos));
    }

    pub fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].update();
            if self.particles[i].is_dead() {
                self.particles.remove(i);
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}
