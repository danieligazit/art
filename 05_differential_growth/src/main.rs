use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use kdtree::KdTree;
use kdtree::ErrorKind;
use kdtree::distance::squared_euclidean;
use nannou::text::glyph::X;
use rand::distributions::{Normal, Distribution};


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

struct Path{
    m_nodes: Vec<Rc<RefCell<Node>>>,
    m_last_node: Option<Rc<RefCell<Node>>>,
    m_first_node: Option<Rc<RefCell<Node>>>,
    // kd_tree: KdTree,
}

impl Path{
    fn new() -> Path{
        // let tree = KdTree::new(2);
        
        Path { 
            m_nodes: Vec::new(), 
            m_last_node: None, 
            m_first_node: None,
            // kd_tree: KdTree::new(2),
        }
    }
    fn add_node(&mut self, node: &Rc<RefCell<Node>>){
        self.m_nodes.push(Rc::clone(node)); // push back?

        match &self.m_last_node {
            None => {
                self.m_first_node = Some(Rc::clone(node));
                self.m_last_node = Some(Rc::clone(node));
            }
            Some(m_last_node) => {
                m_last_node.as_ref().borrow_mut().n_node = Some(Rc::clone(&node));
                node.as_ref().borrow_mut().p_node = Some(Rc::clone(m_last_node));
                self.m_last_node = Some(Rc::clone(node));
            }
        }
    }

    fn apply<F: Fn(&Rc<RefCell<Node>>)>(&mut self, f: F){
        let mut p: Option<Rc<RefCell<Node>>> = match &self.m_first_node {
            None => None, 
            Some(n) => Some(Rc::clone(&n))
        };

        loop {
            let node = match p {
                None => break,
                Some(ref n) => Rc::clone(n),
            };
            
            f(&node);

            p = match node.borrow().n_node {
                None => None,
                Some(ref next) => Some(Rc::clone(next)),
            };
        }
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

    pub fn update(&mut self, create_new: bool){

        
        self.apply(|ref_node| {
            let mut node = ref_node.as_ref().borrow_mut();

            // Brownian
            let normal_dis = Normal::new(0.0, 0.05);

            let scale = 35.0;
            
            let x = node.next_pos.x + scale * normal_dis.sample(&mut rand::thread_rng()) as f32;
            let y = node.next_pos.y + scale * normal_dis.sample(&mut rand::thread_rng()) as f32;

            node.next_pos.x = x;
            node.next_pos.y = y;
        });


        self.apply(|ref_node| {
            // let split_distance
            // Split
        });


        self.apply(|ref_node| {
                
        });

    
        // m_search_index.buildIndex();
    
        // std::shared_ptr<c_node> curr_node = m_first_node;
        // while(curr_node != nullptr)
        // {
        //     const double query_pt[2] = {curr_node->next_pos.x, curr_node->next_pos.y};
        //     const double search_radius = high_thresh*high_thresh;
        //     std::vector<std::pair<uint32_t, double>> ret_matches;
        //     nanoflann::SearchParams params;
    
        //     const size_t n_matches = m_search_index.radiusSearch(&query_pt[0], search_radius, ret_matches, params);
    
        //     dg::pt2 cumulative_force{};
        //     for (size_t i=0; i<n_matches; i++)
        //     {
        //         size_t match_idx = ret_matches[i].first;
        //         dg::pt2 force{};
    
        //         if(m_nodes[match_idx] == curr_node)
        //             continue;
                
        //         if(sqrt(ret_matches[i].second) < low_thresh)
        //             continue;
    
        //         if(sqrt(ret_matches[i].second) > high_thresh)
        //             continue;
    
        //         force = dg::get_force(curr_node->curr_pos, m_nodes[match_idx]->curr_pos, att_rep_crossing);
    
        //         cumulative_force.x += force.x;
        //         cumulative_force.y += force.y;
        //     }
            
        //     cumulative_force.x = std::max(std::min(cumulative_force.x, 1.0), -1.0);
        //     cumulative_force.y = std::max(std::min(cumulative_force.y, 1.0), -1.0);
            
        //     curr_node->next_pos.x += cumulative_force.x;
        //     curr_node->next_pos.y += cumulative_force.y;
    
        //     curr_node = curr_node->n_node.lock();
    
        // path.apply_attraction_repulsion(zero_cross, lower_force_radius, upper_force_radius);

// {
//     if(m_first_node == nullptr)
//     {
//         return;
//     }

//     std::shared_ptr<c_node> node_1 = m_first_node;
//     std::shared_ptr<c_node> node_2 = node_1->n_node.lock();;
//     while(node_1 != nullptr && node_2 != nullptr)
//     {
//         if(dg::node_distance(*node_1, *node_2) > split_distance)
//         {
//             dg::pt2 midpoint = dg::pts_midpoint(node_1->curr_pos, node_2->curr_pos);
//             auto node = std::make_shared<dg::c_node>(midpoint.x, midpoint.y);
//             /* Insert the node in between */
//             m_nodes.push_back(node);
//             node->p_node = node_1;
//             node->n_node = node_2;
//             node_1->n_node = node;
//             node_2->p_node = node;
//         }
//         /* Get next pair */
//         node_1 = node_2;
//         node_2 = node_2->n_node.lock();;
//     }
// }
        self.apply(|node| {
            node.as_ref().borrow_mut().update_position();
        })

    }

    pub fn view(&self, draw: &Draw) {

        let mut p1 = vec2(0.0, 0.0);
        let mut p2 = vec2(0.0, 0.0);
        let mut skip = true;
        
        let mut p: Option<Rc<RefCell<Node>>> = match &self.m_first_node {
            None => None, 
            Some(n) => Some(Rc::clone(&n))
        };

        loop {
            let node = match p {
                None => break,
                Some(ref n) => Rc::clone(n),
            };
            
            p2 = node.borrow().cur_pos;
            
            match skip {
                false => {
                    draw.line()
                        .start(p1)
                        .end(p2)
                        .stroke_weight(0.2)
                        .color(WHITE)
                        ;
                }
                true => {
                    skip = false;
                }
            }
        
            p1 = p2;

            p = match node.borrow().n_node {
                None => None,
                Some(ref next) => Some(Rc::clone(next)),
            };
        }

    }

}

struct Model{
    path: Path,
    elapsed: usize,
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

    let mut path = Path::new();

    let radius = 200.0;

    for step in FRange::new(0.0, 2.0 * PI, 0.15){    
        let pt = vec2(0.0 + radius*step.sin(), 0.0 + radius*step.cos());
        // println!("{}", pt);
        
        let node = Rc::new(RefCell::new(Node::new(pt)));
        path.add_node(&node);
    }
    Model {path: path, elapsed: 0}
}

#[derive(Debug)]
struct Node{
    pub cur_pos: Vec2,
    pub next_pos: Vec2,
    pub p_node: Option<Rc<RefCell<Node>>>,
    pub n_node: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(pos: Vec2) -> Node {
        Node{
            cur_pos: pos,
            next_pos: pos,
            p_node: None,
            n_node: None,
        }
    }

    fn insert_next(&mut self, next: Rc<RefCell<Node>>) {
        next.as_ref().borrow_mut().n_node = match &self.n_node {
            None=>None,
            Some(n_node) =>Some(Rc::clone(&n_node))
        };

        self.n_node = Some(next);
    }

    // fn insert_prev(&mut self, prev: Rc<RefCell<Node>>) {
        
    //     prev.as_ref().borrow_mut().p_node = match &self.p_node {
    //         None=>None,
    //         Some(p_node) =>{
    //             let v = self.p_node.as_ref().borrow_mut(); //.n_node = Some(Rc::clone(&p_node));
    //             Some(Rc::clone(&p_node))
    //         }
    //     };

    //     self.p_node = Some(prev);
    // }

    fn update_position(&mut self){
        // println!("{}, {}", self.cur_pos, self.next_pos);
        self.cur_pos = self.next_pos
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    model.elapsed += 1;
    
    model.path.update(model.elapsed % 60 == 0);


}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    // frame.clear(BLACK);

    model.path.view(&draw);
    
    draw.to_frame(app, &frame).unwrap();
}
