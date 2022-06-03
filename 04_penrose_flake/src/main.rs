
extern crate nannou;
use nannou::prelude::*;

mod penrose_flake;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::rate_fps(64.0))
        .update(update)
        .run();
}

struct Model {
    flakes: Vec<penrose_flake::PenroseFlake>
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .build()
        .unwrap();  


    Model {flakes: Vec::new()}
}


fn update(app: &App, model: &mut Model, _update: Update) {
    for flake in model.flakes.iter_mut() {
        flake.update();
    }

    for (frame, pos, axiom, rule, color, theta) in [
        // (0, app.window_rect().top_left(), WHITE)       
        (0, vec2(-300.0, 20.0), "F3-F3-F3-F3-F", "F3-F3-F45-F++F3-F", WHITE, PI / 10.0),
        // (30, app.window_rect().top_left(), "F", "F+F-F", LIGHTGRAY, PI * 2.0/ 3.0),
        (0, vec2(50.0, 0.0), "F", "F+F-F", LIGHTGRAY, PI * 2.0/ 3.0),
        (0, vec2(0.0, -200.0), "F", "+F+F-F-F+F+F+F-F-F+F", CYAN, PI / 2.0),

    ]{
        if app.elapsed_frames() != frame {continue;}
        println!("{}", app.elapsed_frames());

        let mut flake = penrose_flake::PenroseFlake::new(
            pos,
            color,
            axiom,
            rule,
            theta,
        );


        flake.reset();
        flake.simulate(4);
        model.flakes.push(flake);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // draw.background().color(WHITE);

    for flake in model.flakes.iter() {
        flake.view(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
