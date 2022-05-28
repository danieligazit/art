use nannou::prelude::*;

pub struct PenroseFlake {
    pub axiom: &'static str,
    pub rule_f: &'static str,
    pub start_length: f32,
    pub draw_length: f32,
    pub rotation: f32,
    pub generations: u64,
    pub steps: usize,
    pub production: &'static str,
    pos: Vec2,
}

const THETA: f32 = PI / 10.0;

impl PenroseFlake {
    pub fn new() -> PenroseFlake {
        PenroseFlake {
            axiom: "F3-F3-F3-F3-F",
            rule_f: "F3-F3-F45-F++F3-F",
            start_length: 450.0,
            draw_length: 0.0,
            rotation: THETA,
            generations: 0,
            steps: 0,
            production: "",
            pos: pt2(0.0, 0.0)
        }
    
    }

    pub fn reset(&mut self) {
        self.production = self.axiom;
        self.draw_length = self.start_length;
        self.generations = 0;
    }

    pub fn view(&mut self, draw: &Draw) {
        //translate(width, height)
        let mut repeats = 1;
        self.steps += 3;

        if self.steps > self.production.len() {
            self.steps = self.production.len();
        }

        for i in 0..self.steps {
            let step = self.production.chars().nth(i).unwrap();

            match step {
                'F' => {
                    for j in 0..repeats {
                        draw.line()
                            .start(self.pos)
                            ;
                            // .end(pt2(self.pos.x, self.))
                        
                    
                        // translate(0, -self.draw_length)
                    }
                    repeats = 1;
                }
                '+' => {
                    for j in 0..repeats {
                        self.rotation += THETA;    
                    }
                    repeats = 1;
                }
                '-' => {
                    for j in 0..repeats {
                        self.rotation -= THETA;    
                    }
                    repeats = 1; 
                }
                _ => {}
            }
        }
        
    }

    pub fn update() {
        
    }
}