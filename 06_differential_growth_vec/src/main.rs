
use kd_tree::{KdTree, KdPoint};

use itertools::Itertools;

use rand::distributions::{Normal, Distribution};


mod utils;
extern crate nannou;
use nannou::prelude::*;
use nannou::color::{Alpha, Rgb, IntoLinSrgba};
use utils::FRange;

#[derive(Clone)]
struct Node {
    cur_pos: Vec2,
    next_pos: Vec2,
}

impl Node {
    fn new(pos: Vec2) -> Node {
        Node {
            cur_pos: pos,
            next_pos: pos,
        }
    }
}

struct Path{
    nodes: Vec<Node>,
    color: Srgb<f32>,
}

impl Path{
    fn new(color: Srgb<f32>) -> Path{
        Path { 
            nodes: Vec::new(),
            color: color,
            // kd_tree: KdTree::build_by_ordered_float(vec![]),
        }
    }

    fn add_node(&mut self, node: Node){
        self.nodes.push(node);
    }

    fn apply_split(split_distance: f32){

    }

    fn apply_attraction_repulsion(&mut self, att_rep_crossing: f32, low_thresh: f32, high_thresh: f32){

        let points:Vec<[f32;2]> = self.nodes
            .iter()
            .map(|node|{[node.cur_pos.x, node.cur_pos.y]})
            .collect();
        
        let kdtree = kd_tree::KdTree::build_by_ordered_float(points.to_owned());

        let search_radius = high_thresh * high_thresh;


        self.nodes.iter_mut()
            .for_each(|node| {
                let point = [node.cur_pos.x, node.cur_pos.y];
                let matches = kdtree.within_radius(&point, search_radius);
                let mut cumulative_force = vec2(0.0, 0.0);
                
                matches
                    .iter()
                    .for_each(|ref_match| {
                        let o = vec2(ref_match[0], ref_match[1]);
                        if o == node.cur_pos {return;}

                        let distance = o.distance(node.cur_pos);
                        if distance.sqrt() < low_thresh {return;}
                        if distance.sqrt() > high_thresh {return;}
                        
                        let lj_force = (att_rep_crossing/distance).pow(12) - (att_rep_crossing/distance).pow(6);
                
                        cumulative_force.x += ((o.x - node.cur_pos.x) / distance) * lj_force;
                        cumulative_force.y += ((o.y - node.cur_pos.y) / distance) * lj_force;

                    });
                
                
                node.next_pos.x += cumulative_force.x.min(10.0).max(-10.0);
                node.next_pos.y += cumulative_force.y.min(10.0).max(-10.0);
            });

    }

    fn apply_alignment(&mut self, aligment_force: f32){

        let copied_nodes = self.nodes.to_owned();

        self.nodes
            .iter_mut()
            .enumerate()
            .for_each(|(i, node)| {
                if i == 0 || i >= copied_nodes.len() - 1 {
                    return;
                }

                let prev = copied_nodes[i - 1].cur_pos;
                let next = copied_nodes[i + 1].cur_pos;

                let midpoint = vec2(
                    prev.x.min(next.x) + (prev.x - next.x).abs() / 2.0,
                    prev.y.min(next.y) + (prev.y - next.y).abs() / 2.0,
                );
                
                let distance = prev.distance(next);
                
                let direction = {
                    if distance < 1e-6 {
                        vec2(0.0, 0.0)
                    } else {
                        vec2 (
                            (midpoint.x - node.next_pos.x) / distance,
                            (midpoint.y - node.next_pos.y) / distance
                        )
                    }
                };
                
                node.next_pos.x += aligment_force * direction.x;
                node.next_pos.y += aligment_force * direction.y;

            });

    }
    fn apply_pruning(min_distance: f32){

    }
    
    fn update_path(&mut self){
        self.nodes
            .iter_mut()
            .for_each(|node| {
                node.cur_pos = node.next_pos;
            });
    }

    fn apply_brownian(&mut self){
        let normal_dis = Normal::new(0.0, 0.4);
        let scale = 6.0;

        self.nodes
            .iter_mut()
            .for_each(|node| {
                node.next_pos.x += scale * normal_dis.sample(&mut rand::thread_rng()) as f32;
                node.next_pos.y += scale * normal_dis.sample(&mut rand::thread_rng()) as f32;
            });
    }
    pub fn update(&mut self){
        self.apply_brownian();
        self.apply_attraction_repulsion(ZERO_CROSS, LOWER_FORCE_RADIUS, UPPER_FORCE_RADIUS);
        self.apply_alignment(0.5);
        self.update_path()
       

    }

    pub fn view(&self, draw: &Draw) {
        self.nodes
            .iter()
            .tuple_windows()
            .for_each(|(prev, node)| {
                if prev.cur_pos.distance(node.cur_pos) > 40.0 {
                    return;
                }
                draw.line()
                        .start(prev.cur_pos)
                        .end(node.cur_pos)
                        .stroke_weight(0.5)
                        .color(self.color)
                        ;
            })
            

    }

}

struct Model{
    paths: Vec<Path>,
    elapsed: usize,
}

const OUTPUT_DIM: u32 = 700;
const ZERO_CROSS: f32 = 5.0;
const LOWER_FORCE_RADIUS: f32 = 2.0;
const UPPER_FORCE_RADIUS: f32 = 10.0;
const PRUNING_RADIUS: f32 = 2.0;
const SPLIT_LENGTH: f32 = 10.0;

fn model(app: &App) -> Model {
    app.new_window()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();  

    
    
    let mut path = Path::new(WHITE.into_lin_srgba().try_into().unwrap());

    let radius = 200.0;

    for step in FRange::new(0.0, 2.0 * PI, 0.05){    
        path.add_node(Node::new(vec2(0.0 + radius*step.sin(), 0.0 + radius*step.cos())));
    }

    let mut path2 = Path::new(GRAY.into_lin_srgba().try_into().unwrap());

    let radius = 450.0;

    for step in FRange::new(0.0, 2.0 * PI, 0.05){    
        path2.add_node(Node::new(vec2(0.0 + radius*step.sin(), 0.0 + radius*step.cos())));
    }


    Model {paths: vec![path, path2], elapsed: 0}
}


fn update(app: &App, model: &mut Model, _update: Update) {
    model.paths
        .iter_mut()
        .for_each(|path| {
            path.update();
        });

}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    
    // frame.clear(BLACK);

    draw.rect().w_h(1920.0, 1080.0).color(srgba(0.0, 0.0, 0.0, 0.05));

    model.paths[0].view(&draw);

    if model.elapsed % 200 == 0 || model.elapsed % 201 == 0 || model.elapsed % 202 == 0 || model.elapsed % 203 == 0 || model.elapsed % 204 == 0 || model.elapsed % 205 == 0 || model.elapsed % 206 == 0 {
        model.paths[1].view(&draw);
    }
    

    draw.to_frame(app, &frame).unwrap();

}


fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::rate_fps(64.0))
        .update(update)
        .run();
}