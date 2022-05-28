// nt-no2---perlin-noise
use nannou::noise::NoiseFn;
use nannou::prelude::*;

const CAPTURE: bool = false;

fn main() {
    nannou::app(model)
        // .loop_mode(LoopMode::loop_once())
        .update(update)
        .run();
}

struct Model {
    items: Vec<Vec<Vec3>>,
}

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH,HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model {items: vec![vec![vec3(0.0, 0.0, 0.0); COLS as usize]; ROWS as usize]}
}

const MAX_SOME: f32 = 2.0;
const MAX_ROTATION: f32 = 0.5;
fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 1 != 0 {
        return;   
    }

    let rows = model.items.len();
    
    for (y, row) in model.items.iter_mut().enumerate() {
        for (x, item) in row.iter_mut().enumerate(){
            let factor = y as f32 / rows as f32;

            let mut new_x = item.x + factor * random_range(-0.1, 0.1);
            let mut new_y = item.y + factor * random_range(-0.1, 0.1);

            if new_x > factor * MAX_SOME {
                new_x = MAX_SOME - (new_x - MAX_SOME);
            }

            
            if new_y > factor * MAX_SOME { 
                new_y = MAX_SOME - (new_y - MAX_SOME);
            }

            let mut new_rotation = if app.elapsed_frames() % 1 == 0  {
                item.z + factor * random_range(-PI / 9.0, PI / 9.0)
            } else {
                item.z
            };

            // if new_rotation > factor * MAX_ROTATION {
            //     new_rotation = MAX_SOME - (new_rotation - MAX_SOME);
            // }


            *item = vec3(new_x, new_y, new_rotation);
            
        }
    }


}

const LINE_WIDTH: f32 = 0.06;


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    let gdraw = draw.scale(SIZE as f32)
                  .scale_y(-1.0)
                  .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);
    
    for (y, row) in model.items.iter().enumerate() {
        for (x, item) in row.iter().enumerate(){
            let cdraw = gdraw.x_y(x as f32, y as f32);
            cdraw.rect()
                .color(WHITE)
                .stroke(BLACK)
                .stroke_weight(LINE_WIDTH)
                .w_h(1.0, 1.0)
                .x_y(item.x, item.y)
                .rotate(item.z);
        }
    }
            
    
    draw.to_frame(app, &frame).unwrap();
}
