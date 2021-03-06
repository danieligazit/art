use nannou::prelude::*;
use crate::particle::Particle;

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        let particles: Vec<Particle> = Vec::new();
        ParticleSystem { particles }
    }

    pub fn add_particle(&mut self, pos: Point2) {
        self.particles.push(Particle::new(pos));
    }

    pub fn delete_particles(&mut self){
        self.particles = Vec::new();
    }
    
    pub fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].update();
            if self.particles[i].is_consumed() {
                self.particles.remove(i);
            }
        }
    }

    pub fn apply_velocity<F: Fn(&Point2) -> Vec2>(&mut self, func: F){
        self.particles
            .iter_mut()
            .for_each(|particle|{
                let speed = func(&particle.next_pos);
                particle.next_pos += speed;
            })
    }

    pub fn apply_position<F: Fn(&mut Particle) -> Point2>(&mut self, func: F){
        self.particles.iter_mut().for_each(|particle|{
            particle.next_pos = func(particle);
        });
    }


    pub fn draw(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}
