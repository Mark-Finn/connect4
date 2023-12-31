use std::collections::HashSet;
use std::fs;
use std::io::Write;
use bincode::config;
use bincode::config::Configuration;
use crate::position::{Position, WIDTH, HEIGHT};
use crate::transposition_table::TranspositionTable;

pub struct Explorer {
    visited: HashSet<u64>,
    pub output: Vec<String>,
}

pub fn get_opening_book() -> TranspositionTable {
    return match &fs::read(filepath().as_str()) {
        Ok(encoded) => {
            match bincode::decode_from_slice(&encoded[..], bincode_config()) {
                Ok((transposition_table, _)) => {
                    transposition_table
                },
                Err(_) => TranspositionTable::new(),
            }
        }
        Err(_) => TranspositionTable::new(),
    };
}

pub fn save_opening_book(opening_book: TranspositionTable) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::encode_to_vec(opening_book, bincode_config()).unwrap();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(filepath().as_str())?;
    file.write_all(&*encoded)
}

fn filepath() -> String {
    format!("./data/{}x{}_opening_book", WIDTH, HEIGHT)
}

fn bincode_config() -> Configuration {
    config::standard()
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

