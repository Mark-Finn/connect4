use std::collections::HashSet;
use crate::position::{Position, WIDTH};

pub struct Explorer {
    visited: HashSet<u64>,
    pub output: Vec<String>,
}

impl Explorer {
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            output: vec![],
        }
    }

    pub fn visited_count(&self) -> usize {
        self.visited.len()
    }

    pub fn explore(&mut self, position: impl Position, mut position_string: String, depth: u8) {
        let key = position.symmetric_key();
        if self.visited.contains(&key) || position.get_move_count() > depth {
            return;
        }
        self.visited.insert(key);
        if position.get_move_count() == depth {
            self.output.push(position_string.clone());
        }

        let valid_plays = (0..WIDTH)
            .filter(|col| position.can_play(*col) && !position.is_winning_move(*col));

        for col in valid_plays {
            let mut position2 = position.clone();
            position2.play(col);
            position_string.push(('0' as u8 + col) as char);
            self.explore(position2, position_string.clone(), depth);
            position_string.pop();
        }
    }
}

