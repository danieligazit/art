
use kdtree::KdTree;
use kdtree::ErrorKind;
use kdtree::distance::squared_euclidean;

mod utils;
extern crate nannou;
use nannou::prelude::*;
use utils::FRange;



fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::rate_fps(64.0))
        .update(update)
        .run();
}

struct Path<'a>{
    m_nodes: Vec<&'a Node<'a>>,
    m_last_node: Option<&'a mut Node<'a>>,
    m_first_node: Option<&'a mut Node<'a>>,
    // kd_tree: KdTree,
}

impl<'a> Path<'a>{
    fn new() -> Path<'a> {
        let tree = KdTree::new(2);
        
        Path { 
            m_nodes: Vec::new(), 
            m_last_node: None, 
            m_first_node: None,
            // kd_tree: KdTree::new(2),
        }
    }
    fn add_node(&self, node: &Node){
        self.m_nodes.push(node); // push back?

        match self.m_last_node {
            None => {
                self.m_first_node = node;
                self.m_last_node = node;
            }
            Some(m_last_node) => {
                self.m_last_node.n_node = node;
                node.p_node = m_last_node;
                self.m_last_node = node;
            }
        }
    }
    fn apply_brownian(scale: f32){

    }
    fn apply_split(split_distance: f32){

    }
    fn apply_attraction_repulsion(att_rep_crossing: f32, low_thresh: f32, high_thresh: f32){

    }
    fn apply_alignment(aligment_force: f32){

    }
    fn apply_pruning(min_distance: f32){

    }
    fn update_path(){

    }
}
struct Visualizer<'a>{
    first_node: Option<Node<'a>>,
}

impl Visualizer<'_> {
    pub fn view(&self, draw: &Draw) {

        let mut p1 = vec2(0.0, 0.0);
        let mut p2 = vec2(0.0, 0.0);
        let mut skip = true;

        let mut some_node = &self.first_node;
        while let Some(cur_node) = some_node{
            p2 = cur_node.cur_pos;

            match skip {
                false => {
                    draw.line()
                        .start(p1)
                        .end(p2)
                        .stroke_weight(0.5)
                        ;
                }
                true => {
                    skip = false;
                }
            }

            p1 = p2;
            some_node = cur_node.n_node;
        }
    }
}

struct Model {
    path: Path
}

const OUTPUT_DIM: u32 = 700;
const ZERO_CROSS: f32 = 4.0;
const LOWER_FORCE_RADIUS: f32 = 2.0;
const UPPER_FORCE_RADIUS: f32 = 10.0;
const PRUNING_RADIUS: f32 = 2.0;
const SPLIT_LENGTH: f32 = 10.0;

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .build()
        .unwrap();  

    
        /* Generate initial path (circle) */
    for step in FRange::new(0.0, 2.0 * PI, 0.1){
        let pt = vec2(100.0*step.sin(), 100.0*step.sin());
        
        // path.add_node(std::make_shared<dg::c_node>(350+x, 350+y));
    }
    Model {path: Path{}}
}


struct Node<'a>{
    pub cur_pos: Vec2,
    pub next_pos: Vec2,
    pub p_node: &'a Option<Node<'a>>,
    pub n_node: &'a Option<Node<'a>>,
}

impl Node<'_> {
    fn update_position(&mut self){
        self.cur_pos = self.next_pos
    }

}
fn update(app: &App, model: &mut Model, _update: Update) {

    

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
     
    draw.to_frame(app, &frame).unwrap();
}
