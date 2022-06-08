
use kd_tree::{KdTree, KdPoint};

use itertools::Itertools;

use rand::distributions::{Normal, Distribution};


mod utils;
extern crate nannou;
use nannou::prelude::*;
use nannou::color::{Alpha, Rgb, IntoLinSrgba};
use utils::FRange;

struct Model{
    elapsed: usize,
    texture: wgpu::Texture,
    detection: edge_detection::Detection,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(200, 200)
        .view(view)
        .build()
        .unwrap();  

    
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("heart.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    
    let img_path = assets.join("heart.png");
    let source_image = image::open(img_path).unwrap().to_luma8();
    
    let detection = edge_detection::canny(
        source_image,
        1.2,  // sigma
        0.2,  // strong threshold
        0.01, // weak threshold
    );

    // let texture = wgpu::Texture::from_image(app, );
    Model {
        elapsed: 0,
        texture,   
        detection: detection 
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    
}

fn view(app: &App, model: &Model, frame: Frame) {

    frame.clear(GRAY);
    // draw.rect().w_h(1920.0, 1080.0).color(BLACK); // srgba(0.0, 0.0, 0.0, 0.05)

    let win = app.window_rect();
    let r = Rect::from_w_h(200.0, 200.0).top_left_of(win);

    let draw = app.draw();


    draw.texture(&model.texture)
        .xy(r.xy())
        .wh(r.wh());
    
    
    // let edge = rand::thread_rng().gen_range(1..);

    let mut start = (0, 0);

    for width in 0..model.detection.width() {
        for height in 0..model.detection.height() {
            if model.detection[(width, height)].dir() != (0.0, 0.0){
                start = (width * (200 /width ), height  * (200/height ));
                break;
            }
        }
    }

    let edge = model.detection[(start.0, start.1)];
    println!("{:?}", edge.dir());

    draw.line()
        .start(vec2(start.0 as f32, start.1 as f32))
        .end(vec2(start.0 as f32 + edge.dir().0 * 100.0, start.1 as f32 + edge.dir().1 * 100.0))
        .stroke_weight(1.0)
        .color(BLACK)
        ;
        
    draw.to_frame(app, &frame).unwrap();

}


fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::rate_fps(64.0))
        .update(update)
        .run();
}