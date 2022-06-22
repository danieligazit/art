use nannou::prelude::*;
use kd_tree::{KdTree, KdPoint};
use crate::particle::{Particle, self};

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

    pub fn get_kd_tree(&self) -> KdTree<[f32; 2]>{
        
        let points:Vec<[f32;2]> = self.particles
            .iter()
            .map(|particle|{[particle.pos.x, particle.pos.y]})
            .collect();
        
        kd_tree::KdTree::build_by_ordered_float(points.to_owned())


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
