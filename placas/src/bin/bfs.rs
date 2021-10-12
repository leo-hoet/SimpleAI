use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use placas::node::{Node, State};

fn bfs(initial: State, desire: State) -> Option<Node> {
    let mut closed = HashSet::<[u32; 9]>::new();
    let mut nodes = VecDeque::<Node>::new();
    nodes.push_back(Node {
        state: initial,
        costo: 0,
        father: None,
    });

    let mut iter = 0;
    while let Some(node) = nodes.pop_front() {
        //println!(
        //"Iter: {}, Node: {:?}, Cost: {}",
        //iter, node.state.board, node.costo
        //);
        //closed.insert(node.state.board.clone());
        if node.state.board == desire.board {
            return Some(node);
        }
        let nexts = node.state.next();
        let sp = Rc::new(node);
        for child in nexts {
            //if closed.contains(&child.board) {
            //continue;
            //}
            nodes.push_back(Node {
                state: child,
                costo: sp.costo + 1,
                father: Some(Rc::clone(&sp)),
            })
        }
        iter += 1;
    }
    None
}

struct StateWrapper;
impl StateWrapper {
    pub fn bfs(initial: [u32; 9], desire: [u32; 9]) -> Option<Node> {
        let init = State {
            board: initial,
            blank_index: initial.iter().position(|x| *x == 0).unwrap() as isize,
        };

        let des = State {
            board: desire,
            blank_index: initial.iter().position(|x| *x == 0).unwrap() as isize,
        };

        bfs(init, des)
    }
}

fn main() {
    let state1 = [1, 2, 3, 0, 4, 5, 6, 7, 8];
    let state2 = [1, 2, 3, 4, 0, 5, 6, 7, 8];
    let state3 = [1, 0, 2, 3, 4, 5, 6, 7, 8];
    let desire = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    for _ in 0..10 {
        let t0 = std::time::Instant::now();
        let node = StateWrapper::bfs(state3, desire).unwrap();
        println!("bfs, state3, {}", t0.elapsed().as_micros());
    }

    //node.print_path();
}
