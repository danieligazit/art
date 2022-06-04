use nannou::prelude::*;
use palette::rgb::Srgb;
use std::cmp;
pub struct PenroseFlake {
    pub axiom: String,
    pub rule_f: String,
    pub start_length: f32,
    pub draw_length: f32,
    pub generations: u64,
    pub steps: usize,
    pub repeats: u32,
    pub step: u32,
    pub production: String,
    pub buf: Vec<Vec2>,
    pub pos: Vec2,
    pub theta: f32,
    pub base_theta: f32,
    pub color: Srgb<u8>,
}

const THETA: f32 = PI / 10.0;

impl PenroseFlake {
    pub fn new(pos: Vec2, color: Srgb<u8>, axiom: &str, rule: &str, base_theta: f32) -> PenroseFlake {
        PenroseFlake {
            axiom: axiom.into(),
            rule_f: rule.into(),
            start_length: 500.0,
            draw_length: 0.0,
            generations: 0,
            steps: 0,
            production: "".into(),
            buf: Vec::new(),
            pos: pos,
            repeats: 1,
            step: 0,
            theta: 0.0,
            base_theta: base_theta,
            color: color,
        }
        
    }

    pub fn simulate(&mut self, gen: u64){
        while self.generations < gen {
            self.production = self.production.replace("F", &self.rule_f);
            self.draw_length *= 0.4;
            self.generations += 1;
        }
    }

    pub fn reset(&mut self) {
        self.steps = cmp::max(3, self.production.len());
        self.production = (&self.axiom).into();
        self.draw_length = self.start_length;
        self.generations = 0;

    }

    pub fn update(&mut self) {
        
        self.pos = match self.buf.last(){
            None => self.pos,
            Some(pos) => *pos
        };

        self.buf.clear();

        self.step += 1;

        if self.step >= self.production.len().try_into().unwrap() {
            self.step = 0;
            self.theta = 0.0;
        }
        let step = self.production.chars().nth(self.step as usize).unwrap();

        match step {
            'F' => {
                for _ in 0..self.repeats {
                    let point = pt2(
                        self.pos.x - (self.draw_length * self.theta.sin()),  
                        self.pos.y - (self.draw_length * self.theta.cos())
                    );
                    self.buf.push(point);
                }
                self.repeats = 1;
            }
            '+' => {
                for _ in 0..self.repeats {
                    self.theta += self.base_theta;    
                }
                self.repeats = 1;
            }
            '-' => {
                for _ in 0..self.repeats {
                    self.theta -= self.base_theta;    
                }
                self.repeats = 1; 
            }
            '0'..='9' => {
                self.repeats += step as u32 - 48;
            }
            _ => {}
        }

    }

    fn update_once(&mut self) {
        self.step += 1;

        let step = self.production.chars().nth(self.step as usize).unwrap();

        match step {
            'F' => {
                for _ in 0..self.repeats {
                    let point = pt2(
                        self.pos.x - (self.draw_length * self.theta.sin()),  
                        self.pos.y - (self.draw_length * self.theta.cos())
                    );
                    self.buf.push(point);
                }
                self.repeats = 1;
            }
            '+' => {
                for _ in 0..self.repeats {
                    self.theta += THETA;    
                }
                self.repeats = 1;
            }
            '-' => {
                for _ in 0..self.repeats {
                    self.theta -= THETA;    
                }
                self.repeats = 1; 
            }
            '0'..='9' => {
                self.repeats += step as u32 - 48;
            }
            _ => {}
        }
    }

    pub fn view(&self, draw: &Draw) {
        let mut pos = self.pos;
        for point in &self.buf{
            draw.line()
                .start(pos)
                .end(*point)
                .weight(0.4)
                .color(self.color)
                ;
            pos = *point;
        }
                    
    }

   
}