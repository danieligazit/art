use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use kd_tree::{KdTree, KdPoint};
use typenum;
use itertools::Itertools;
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
    nodes: Vec<Vec2>,
}

impl Path{
    fn new() -> Path{
        Path { 
            nodes: Vec::new(),
            // kd_tree: KdTree::build_by_ordered_float(vec![]),
        }
    }

    fn add_node(&mut self, node: Vec2){
        self.nodes.push(node);
    }

    fn apply_split(split_distance: f32){

    }

    fn apply_attraction_repulsion(&mut self, att_rep_crossing: f32, low_thresh: f32, high_thresh: f32){

        let points:Vec<[f32;2]> = self.nodes
            .iter()
            .map(|node|{[node.x, node.y]})
            .collect();
        
        let kdtree = kd_tree::KdTree::build_by_ordered_float(points.to_owned());

        let search_radius = high_thresh * high_thresh;


        self.nodes = points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let matches = kdtree.within_radius(point, search_radius);
                let cumulative_force = vec2(0.0, 0.0);
                let node = vec2(
                    point[0], 
                    point[1]
                );

                matches
                    .iter()
                    .for_each(|ref_match| {
                        let m = *ref_match;
                        if m == point {return;}

                        let o = vec2(m[0], m[1]);

                        let distance = o.distance(node);
                        if distance < low_thresh {return;}
                        if distance > high_thresh {return;}
                        
                        
                            double distance = sqrt(pow(first.x - second.x,2)+pow(first.y - second.y,2));
                            double lj_force = pow(zero_point/distance, 12) - pow(zero_point/distance, 6);
                        
                            dg::pt2 retval{};
                            retval.x = ((second.x - first.x) / distance)*lj_force;
                            retval.y = ((second.y - first.y) / distance)*lj_force;
                        
                            return retval;
                        }

                        
                    });
                
                vec2(
                    point[0], 
                    point[1]
                )
                
            })
            .collect();
            


    }

    fn apply_alignment(aligment_force: f32){

    }
    fn apply_pruning(min_distance: f32){

    }
    fn update_path(){

    }

    fn apply_brownian(&self){
        let normal_dis = Normal::new(0.0, 0.05);
        let scale = 35.0;

        self.nodes = self.nodes
            .iter_mut()
            .map(|node| {
                vec2(
                    node.x + scale * normal_dis.sample(&mut rand::thread_rng()) as f32,
                    node.y + scale * normal_dis.sample(&mut rand::thread_rng()) as f32,
                )        
            })
            .collect();
    }
    pub fn update(&mut self){
        self.apply_brownian();
        
        self.apply_attraction_repulsion(ZERO_CROSS, LOWER_FORCE_RADIUS, UPPER_FORCE_RADIUS);

            // self.kd_tree.build_by_ordered_float(self.m_first_node)
    
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

    }

    pub fn view(&self, draw: &Draw) {
        self.nodes
            .iter()
            .tuple_windows()
            .for_each(|(prev, node)| {
                draw.line()
                        .start(*prev)
                        .end(*node)
                        .stroke_weight(0.2)
                        .color(WHITE)
                        ;
            })
            

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
        path.add_node(vec2(0.0 + radius*step.sin(), 0.0 + radius*step.cos()));
    }
    Model {path: path, elapsed: 0}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.elapsed += 1;
    
    model.path.update();


}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    // frame.clear(BLACK);

    model.path.view(&draw);
    
    draw.to_frame(app, &frame).unwrap();
}
