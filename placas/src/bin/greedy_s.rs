use std::{collections::HashSet, rc::Rc};

use placas::node::{Node, State};
use priority_queue::PriorityQueue;

fn greedy_search(inital: State, desire: State) -> Option<Node> {
    let mut closed = HashSet::<[u32; 9]>::new();
    let mut nodes = PriorityQueue::new();
    let first = Node {
        state: inital,
        costo: 0,
        father: None,
    };
    let priority = first.state.h_right_pos(&desire);
    nodes.push(first, priority);

    let mut i = 0;
    while let Some((node, priority)) = nodes.pop() {
        if node.state.board == desire.board {
            return Some(node);
        }
        closed.insert(node.state.board.clone());
        //println!(
        //"Iter: {}, board: {:?}, h: {}",
        //i, node.state.board, priority
        //);

        let sp = Rc::new(node);
        let childs = sp.state.next();
        for child in childs.into_iter() {
            if closed.contains(&child.board) {
                continue;
            }
            let n = Node {
                state: child,
                costo: sp.costo + 1,
                father: Some(Rc::clone(&sp)),
            };
            let p = n.state.h_right_pos(&desire);
            nodes.push(n, p);
        }
        i += 1;
    }

    None
}

struct StateWrapper;
impl StateWrapper {
    pub fn greedy_search(initial: [u32; 9], desire: [u32; 9]) -> Option<Node> {
        let init = State {
            board: initial,
            blank_index: initial.iter().position(|x| *x == 0).unwrap() as isize,
        };

        let des = State {
            board: desire,
            blank_index: initial.iter().position(|x| *x == 0).unwrap() as isize,
        };

        greedy_search(init, des)
    }
}

fn main() {
    let state1 = [1, 2, 3, 0, 4, 5, 6, 7, 8];
    let state2 = [1, 2, 3, 4, 0, 5, 6, 7, 8];
    let state3 = [1, 0, 2, 3, 4, 5, 6, 7, 8];
    let desire = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    for _ in 0..10 {
        let t0 = std::time::Instant::now();
        let node = StateWrapper::greedy_search(state1, desire).unwrap();
        println!("h_right_pos, state1, {}", t0.elapsed().as_micros());
    }
}
