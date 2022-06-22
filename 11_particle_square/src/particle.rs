use nannou::prelude::geom::polygon::triangles;
use nannou::prelude::*;

use std::iter;

#[derive(Debug)]
pub struct Particle {
    pub pos: Point2,
    pub next_pos: Point2,
    tail: Vec<Point2>,
    pub tail_length: usize,
    achieved_adulthood: bool,
    pub life_span: f32,
}

impl Particle {
    pub fn new(l: Point2) -> Self {
        Particle {
            tail: Vec::new(),
            pos: l,
            next_pos: l,
            tail_length: 0,
            achieved_adulthood: false,
            life_span: 1000.0,
        }
    }

    pub fn apply_position<F: Fn(&Particle) -> Point2>(&mut self, func: F){
        self.next_pos = func(&self);
    }

    // Method to update position
    pub fn update(&mut self) {
        
        if !self.is_dead(){
            self.pos = self.next_pos;
            
            if self.tail_length != 0{
                
                self.tail.push(self.pos);
            }
        }
        
        self.life_span -= 1.0;
        

        if (self.achieved_adulthood || self.is_dead()) && self.tail.len() > 0 {
            self.tail.remove(0);
        }

        if self.tail.len() >= self.tail_length {
            self.achieved_adulthood = true;
        }
    }

    // Method to display
    pub fn display(&self, draw: &Draw) {
        // let mm: Vec<usize> = self.tail.iter().enumerate().map(|(i, _)| i).collect();
        // println!("{:?}", mm);

        let points = self.tail
            .iter()
            .enumerate()
            .map(|(i, point)| {
                
                // let new = i as f32 * 10.0;
                // let color= rgb(255.0 - new , 255.0 - new, 255.0 - new);
                // println!("{:?}", color);
                (*point, WHITE)
            })
            .chain(iter::once((self.pos, WHITE)));
        
        if self.tail.len() == 0 {
            draw.ellipse()
                .radius(4.0)
                .xy(self.pos)
                .color(BLACK)
                ;
        }

        draw
            .polyline()
            .weight(0.2)
            .join_round()
            .points_colored(points)
            
            // .color(WHITE)
            ;
    }


    // Is the particle still useful?
    pub fn is_dead(&self) -> bool {
        return self.life_span < 0.0;
    }

    pub fn is_consumed(&self) -> bool {
        return self.is_dead() && self.tail.len() == 0;
    }
}