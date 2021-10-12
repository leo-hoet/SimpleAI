use std::mem;
use std::rc::Rc;

use rand::distributions::Standard;
use smartcore::math::distance::manhattan::Manhattan;
use smartcore::math::distance::Distance;

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct State {
    pub blank_index: isize,
    pub board: [u32; 9],
}

impl State {
    /// Returns the complement of the manhattan distance in machine dependent word between
    /// the actual state and the desire state
    pub fn h_manhattan_inv(&self, desire: &State) -> usize {
        let v1 = Vec::from(self.board)
            .into_iter()
            .map(|e| e as f64)
            .collect();
        let v2 = Vec::from(desire.board)
            .into_iter()
            .map(|e| e as f64)
            .collect();
        let l1 = Manhattan {}.distance(&v1, &v2);
        usize::MAX - (l1 as usize)
    }

    /// Returns the number of plates in the right position
    pub fn h_right_pos(&self, desire: &State) -> usize {
        let mut count = 0;
        for (i, _) in self.board.iter().enumerate() {
            if self.board[i] == desire.board[i] {
                count += 1;
            }
        }
        count
    }

    pub fn next(&self) -> Vec<State> {
        let mut states = Vec::<State>::new();
        if let Some(state) = self.up() {
            states.push(state)
        };
        if let Some(state) = self.down() {
            states.push(state)
        };
        if let Some(state) = self.left() {
            states.push(state)
        };
        if let Some(state) = self.right() {
            states.push(state)
        };
        states
    }

    fn up(&self) -> Option<State> {
        let step_size = -3;
        let blank_index = self.blank_index as usize;
        if (self.blank_index + step_size) < 0 {
            return None;
        };
        let mut board = self.board.clone();
        board.swap(blank_index, (blank_index as isize + step_size) as usize);
        Some(State {
            board: board,
            blank_index: self.blank_index + step_size,
        })
    }

    fn down(&self) -> Option<State> {
        let step_size = 3;
        let blank_index = self.blank_index as usize;
        if blank_index + step_size > 8 {
            return None;
        };
        let mut board = self.board.clone();
        board.swap(blank_index, blank_index + step_size as usize);
        Some(State {
            board: board,
            blank_index: (blank_index + step_size) as isize,
        })
    }

    fn left(&self) -> Option<State> {
        let step_size = -1;
        let blank_index = self.blank_index as isize;
        if blank_index == 0 || blank_index == 3 || blank_index == 6 {
            return None;
        };
        let mut board = self.board.clone();
        board.swap(blank_index as usize, (blank_index + step_size) as usize);
        Some(State {
            board: board,
            blank_index: (blank_index + step_size) as isize,
        })
    }

    fn right(&self) -> Option<State> {
        let step_size = 1;
        let blank_index = self.blank_index as usize;
        if blank_index == 2 || blank_index == 5 || blank_index == 8 {
            return None;
        };
        let mut board = self.board.clone();
        board.swap(blank_index, blank_index + step_size as usize);
        Some(State {
            board: board,
            blank_index: (blank_index + step_size) as isize,
        })
    }
}

#[derive(PartialEq, Debug, Hash, Eq)]
pub struct Node {
    pub state: State,
    pub costo: usize,
    pub father: Option<Rc<Node>>,
    // This structure may need to implement Drop trait
    // in order to avoid stack overflow
}

impl Node {
    pub fn print_path(&self) {
        println!("{:?}", self.state.board);
        if let Some(father) = self.father.as_ref() {
            father.print_path();
        }
    }
}
