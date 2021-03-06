mod particle;
mod utils;
mod particle_system;

use rand::distributions::{Normal, Distribution};
use utils::FRange;
use nannou::{prelude::*, noise::Perlin, noise::NoiseFn};
use particle_system::ParticleSystem;
use std::{collections::HashMap};


fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    ps: ParticleSystem,
}

const WIDTH: u32 = 1920; // 2160
const HEIGHT: u32 = 1080; // 3840

fn model(app: &App) -> Model {
    app.new_window().size(WIDTH, HEIGHT).view(view).build().unwrap();
    let (_w, _h) = app.window_rect().w_h();
    let mut ps = ParticleSystem::new();

    let max_particles = 300000;
    let max_iteraions = 3000;
    let accuracy = 100_000_000_000.0;
    let offset = 90000;
    let increment = 0.005;
    let mut locations = HashMap::new();

    for i in offset..offset+max_iteraions{
        if locations.len() >= max_particles{break;}
        let seed = i as f32 * increment;
        
        let normal_dis = Normal::new(0.0, 0.4);

        let location = vec2(
            normal_dis.sample(&mut rand::thread_rng()) as f32,
            normal_dis.sample(&mut rand::thread_rng()) as f32,
        );

        let counter = locations
            .entry((
                (location.x * accuracy) as i64, 
                (location.y * accuracy) as i64,
            ))
            .or_insert(0);

        *counter += 1;
    }

    locations
        .iter()
        .for_each(|((x, y), count)| {
            ps.add_particle(
                300.0 * vec2(
                    *x as f32 / accuracy,
                    *y as f32 / accuracy,
                )
            );
        });

    println!("{}", locations.len());
    ps.update();

    Model { ps }
}



fn update(app: &App, m: &mut Model, _update: Update) {
    
    m.ps.update();

    let normal_dis = Normal::new(0.0, 0.4);

}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    
    // draw.background().color(BLACK);
    // draw.rect().w_h(1920.0, 1080.0).color(srgba(0.0, 0.0, 0.0, 0.1));

    draw.rect().x_y(0.0, 0.0).w_h(300.0, 300.0).color(WHITE);
    

    m.ps.draw(&draw);

    draw.to_frame(app, &frame).unwrap();

    // let file_path = captured_frame_path(app, &frame);
    // app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join("output")
        .join("004")
        // Name each file after the number of the frame.
        .join(format!("{}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("jpeg")
}