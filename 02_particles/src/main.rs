// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-4: Cantor Set
// Renders a simple fractal, the Cantor Set
mod particle;
mod particle_system;

use nannou::{prelude::*, noise::Perlin, noise::NoiseFn};
use particle_system::ParticleSystem;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    ps: ParticleSystem,
    noise: Perlin,
}

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

fn model(app: &App) -> Model {
    app.new_window().size(WIDTH, HEIGHT).view(view).build().unwrap();
    let (_w, h) = app.window_rect().w_h();
    let ps = ParticleSystem::new();
    let noise = Perlin::new();
    Model { ps , noise}
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.ps.apply_position(|particle| {
        pt2(particle.next_pos.x + 5.0 + random_range(-2.0, 2.0), particle.next_pos.y + 5.0 +random_range(-1.0, 1.0))
    });

    m.ps.apply_position(|particle| {
        particle.next_pos + vec2(
            m.noise.get([particle.next_pos.x as f64, particle.next_pos.y  as f64, 0.0]) as f32,
            m.noise.get([particle.next_pos.x as f64, particle.next_pos.y as f64, 20.0]) as f32,
        )
    });

    m.ps.add_particle(pt2(random_range(-1.0 * WIDTH as f32, WIDTH as f32), random_range(-1.0 * HEIGHT as f32, HEIGHT as f32)));
    m.ps.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
    println!("her");
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE); 
    }
    // Begin drawing
    
    draw.background().color(WHITE);

    m.ps.draw(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}