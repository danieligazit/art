
extern crate nannou;
use nannou::prelude::*;

mod penrose_flake;

fn main() {
    nannou::app(model)
        // .loop_mode(LoopMode::loop_once())
        .update(update)
        .run();
}

struct Model {
    flake: penrose_flake::PenroseFlake
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .build()
        .unwrap();

    Model {flake: penrose_flake::PenroseFlake::new()}
}


fn update(app: &App, model: &mut Model, _update: Update) {
    // model.flake.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    model.flake.view(draw);
    draw.to_frame(app, &frame).unwrap();
}
