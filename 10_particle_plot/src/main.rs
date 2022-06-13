// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-4: Cantor Set
// Renders a simple fractal, the Cantor Set
mod particle;
mod utils;
mod particle_system;

use utils::FRange;
use nannou::{prelude::*, noise::Perlin, noise::NoiseFn};
use particle_system::ParticleSystem;
use std::{collections::HashMap};

use nalgebra;
use std::hash::{Hasher, Hash};

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

const WIDTH: u32 = 1080; // 2160
const HEIGHT: u32 = 1920; // 3840

fn model(app: &App) -> Model {
    app.new_window().size(WIDTH, HEIGHT).view(view).build().unwrap();
    let (_w, h) = app.window_rect().w_h();
    let ps = ParticleSystem::new();
    let noise = Perlin::new();
    Model { ps , noise}
}

fn one(point: Point2) -> Point2 {
    pt2(
        point.x.sin() + (2.0 * point.x).sin() * (2.0 * point.x).sin() * point.x.cos(),
        point.y.cos() + point.y.pow(2.0).cos() * (2.0 * point.y).sin(),
    )
}


fn two(point: Point2) -> Point2 {
    pt2(
        point.x.sin() * (point.y * point.x).sin(),
        point.y.cos() / ((point.y * point.y * 4.0).cos()),
    )
}

fn three(point: Point2) -> Point2 {
    pt2(
        (point.x.pow(2) as f32/ point.x + point.x).sin(),
        point.y.cos() + (point.y * point.y).sin()
    )
}

fn update(app: &App, m: &mut Model, _update: Update) {

    let max_particles = 300000;
    let max_iteraions = 300000;
    let accuracy = 100_000_000_000.0;
    let offset = 30000;
    let increment = 0.0005;
    let mut locations = HashMap::new();

    for i in offset..offset+max_iteraions{
        if locations.len() >= max_particles{break;}
        let seed = i as f32 * increment;

        let location = two(one(three(vec2(seed, seed))));

        let counter = locations
            .entry((
                (location.x * accuracy) as i64, 
                (location.y * accuracy) as i64,
            ))
            .or_insert(0);

        *counter += 1;
    }

    m.ps.delete_particles();
    
    locations
        .iter()
        .for_each(|((x, y), count)| {
            m.ps.add_particle(
                400.0 * vec2(
                    *x as f32 / accuracy,
                    *y as f32 / accuracy,
                )
            );
        });
    
    m.ps.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BLACK); 
    }
    
    draw.background().color(BLACK);
    m.ps.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}