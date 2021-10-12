use std::{collections::HashSet, rc::Rc};

use rand::seq::SliceRandom;
use rand::thread_rng;

use placas::node::{Node, State};

fn dfs(initial: State, desire: State) -> Option<Node> {
    let mut nodes = Vec::<Node>::new();

    nodes.push(Node {
        state: initial,
        costo: 0,
        father: None,
    });

    let mut iter = 0;
    while let Some(node) = nodes.pop() {
        if node.state.board == desire.board {
            return Some(node);
        }
        //println!(
        //"Iter: {}, board: {:?}, cost: {}",
        //iter, node.state.board, node.costo
        //);
        let mut childs = node.state.next();
        childs.shuffle(&mut thread_rng());

        let sp = Rc::<Node>::new(node);
        for child in childs.into_iter() {
            nodes.push(Node {
                state: child,
                costo: sp.costo + 1,
                father: None, //Some(Rc::clone(&sp)),
            });
        }
        iter += 1;
    }
    None
}

struct StateWrapper;
impl StateWrapper {
    pub fn dfs(initial: [u32; 9], desire: [u32; 9]) -> Option<Node> {
        let init = State {
            board: initial,
            blank_index: initial.iter().position(|x| *x == 0).unwrap() as isize,
        };

        let des = State {
            board: desire,
            blank_index: initial.iter().position(|x| *x == 0).unwrap() as isize,
        };

        dfs(init, des)
    }
}

fn main() {
    let state1 = [1, 2, 3, 0, 4, 5, 6, 7, 8];
    let state2 = [1, 2, 3, 4, 0, 5, 6, 7, 8];
    let state3 = [1, 0, 2, 3, 4, 5, 6, 7, 8];
    let desire = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    for _ in 0..10 {
        let t0 = std::time::Instant::now();
        let node = StateWrapper::dfs(state3, desire).unwrap();
        println!("dfs, state2, {}", t0.elapsed().as_micros());
    }
}
