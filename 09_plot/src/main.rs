
use kd_tree::{KdTree, KdPoint};

use itertools::Itertools;

use rand::distributions::{Normal, Distribution};


mod utils;
extern crate nannou;
use nannou::prelude::*;
use nannou::color::{Alpha, Rgb, IntoLinSrgba};
use utils::FRange;

struct Model{
}


fn model(app: &App) -> Model {
    app.new_window()
        .size(2160, 3840)
        .view(view)
        .build()
        .unwrap();  

    Model{}
}




fn update(app: &App, model: &mut Model, _update: Update) {
    
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


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // if frame.nth() >= 30000 {
    //     return;
    // }
    // frame.clear(WHITE);

    // frame.nth() as f32 - 1.0, frame.nth() as f32

    let t = frame.nth() as f32 / 2000.0;
    
    let nth = (frame.nth() + 1000) as f32;
    // FRange::new(nth * 0.2 , (nth + 1.0) * 0.2 , 0.0005)
    //     .map(|t| vec2(t, t))
    //     .map(one)
    //     .map(two)
        
    //     .for_each(|t| {
    //         draw.ellipse()
    //             .xy(t * 300.0)
    //             // .color(rgb(12.75, 12.75, 12.75))
    //             .color(LIGHTGRAY)
    //             .radius(0.3)
    //             ;      
    //     });
    
    // FRange::new(nth * 0.2 , (nth + 1.0) * 0.2 , 0.0005)
    //     .map(|t| vec2(t, t))
    //     .map(two)
    //     .for_each(|t| {
    //         draw.ellipse()
    //             .xy(t * 300.0)
    //             // .color(rgb(12.75, 12.75, 12.75))
    //             .color(LIGHTGRAY)
    //             .radius(0.3)
    //             ;      
    //     });
    
    FRange::new(nth * 0.2 , (nth + 1.0) * 0.2 , 0.0005)
        .map(|t| vec2(t, t))
        .map(three)

        .map(one)
        .map(two)
        .map(|t| t * 300.0)
        // .map(|t| vec2(t.x.round(), t.y.round()))
        .for_each(|t| {
            draw.ellipse()
                .xy(t )
                // .color(rgb(12.75, 12.75, 12.75))
                .color(LIGHTGRAY)
                .radius(0.3)
                ;      
        });
    
    // draw.polyline()
    //     .points_colored(points);
    
    draw.to_frame(app, &frame).unwrap();
}


fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::rate_fps(64.0))
        .update(update)
        .run();
}