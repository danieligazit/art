
extern crate nannou;

use nannou::color::*;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        // .loop_mode(LoopMode::loop_once())
        .update(update)
        .run();
}

struct Model {
    perlin: Perlin,
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .build()
        .unwrap();

    Model {perlin: Perlin::new()}
}


fn update(app: &App, model: &mut Model, _update: Update) {
}


// number of vertices in each polyline
const num_steps: i32 = 5;
  // distance between each vertex in polyline
const default_length: f64 = 1.0;
  // higher is less noisy ("longer wavelength")
const noise_scale: f64 = 50.0;
  // number of points in our "grid" of starting points
const resolution: i32 = 75;
const pi2: f32 = PI * 2.0;

fn view(app: &App, model: &Model, frame: Frame) {

  // t is really cool as a third param to perlin.get if you want animation
  let _t = app.duration.since_start.secs() as f64;
  let win = app.window_rect();


  // Prepare to draw.
  let draw = app.draw();

  // Clear the background to black.
  draw.background().color(BLACK);

  for i in 0..resolution {
    let x_factor = i as f32 / resolution as f32;

    for j in 0..resolution {
      let y_factor = j as f32 / resolution as f32;

      // slightly randomize starting position
      let mut current_x = win.x.lerp(x_factor) as f64 + (random_f64() * 7.0) - 3.5;
      let mut current_y = win.y.lerp(y_factor) as f64 + (random_f64() * 7.0) - 3.5;

      let mut points = Vec::new();
      let radius = (current_x.powi(2) + current_y.powi(2)).sqrt();
      let color = Hsl::new(
        RgbHue::from_radians(radius as f32 / win.w() * pi2),
        0.7,
        0.5,
      );

      let line_weight = 4.0 - (radius as f32 / win.w() * 6.0);

      points.push((pt2(current_x as f32, current_y as f32), color));

      for _n in 0..num_steps {
        let radius = (current_x.powi(2) + current_y.powi(2)).sqrt();

        // theta is the angle from the center to the point
        // Since win.y() is a range from negative to positive, theta must be inverted when y is negative
        let theta = match current_y < 0.0 {
          true => (current_x / radius).acos() * -1.0,
          false => (current_x / radius).acos(),
        };

        // noisy weighted gradient pointing away from center
        // let angle =
        //   (model.perlin.get([current_x / noise_scale, current_y / noise_scale]) + 2.0 * theta) / 2.0;

        let new_x = current_x + default_length; // angle.cos() * default_length + current_x;
        let new_y = current_y+ default_length; //angle.sin() * default_length + current_y;

        let color = Hsl::new(
          RgbHue::from_radians(radius as f32 / win.w() * pi2),
          0.2,
          0.5,
        );

        points.push((pt2(new_x as f32, new_y as f32), color));
        current_x = new_x;
        current_y = new_y;
      }

      draw
        .polyline()
        .start_cap_round()
        .weight(line_weight)
        .points_colored(points);
    }
  }

  // Write to the window frame.
  draw.to_frame(app, &frame).unwrap();

}
