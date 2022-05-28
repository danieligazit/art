// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-4: Cantor Set
// Renders a simple fractal, the Cantor Set
mod particle;
mod particle_system;

use nannou::prelude::*;
use particle_system::ParticleSystem;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    ps: ParticleSystem,
}

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

fn model(app: &App) -> Model {
    app.new_window().size(WIDTH, HEIGHT).view(view).build().unwrap();
    let (_w, h) = app.window_rect().w_h();
    let ps = ParticleSystem::new();
    Model { ps }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.ps.add_particle(pt2(random_range(-1.0 * WIDTH as f32, WIDTH as f32), random_range(-1.0 * HEIGHT as f32, HEIGHT as f32)));
    m.ps.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    m.ps.draw(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}