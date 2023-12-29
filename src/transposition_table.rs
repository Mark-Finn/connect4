use std::collections::HashMap;
use std::mem::size_of;
use bincode::{Decode, Encode};
use crate::position::{BoardType, WIDTH, HEIGHT};

type KeyType = u32;

#[derive(Encode, Decode)]
pub struct TranspositionTable
{
    keys: Vec<KeyType>,
    values: Vec<i8>
}

impl TranspositionTable {

    const SIZE: usize = 3_000_017;

    pub fn new() -> Self {
        debug_assert!({
            let table_key_bits = size_of::<KeyType>() * 8;
            let position_key_bits = WIDTH * (HEIGHT + 1);
            let min_size: BoardType = (1 << position_key_bits) / (1 << table_key_bits);
            get_prime_greater_than_or_equal(min_size) <= Self::SIZE as BoardType
                && is_prime(Self::SIZE as BoardType)
        });
        Self {
            keys: vec![KeyType::MAX; Self::SIZE],
            values: vec![0; Self::SIZE],
        }
    }

    pub fn put(&mut self, key: BoardType, value: i8) {
        let index = Self::index(key);
        self.keys[index] = key as KeyType;
        self.values[index] = value;
    }

    pub fn get(&self, key: BoardType) -> Option<i8> {
        let index = Self::index(key);
        if self.keys[index] != key as KeyType
        {
            return None;
        }
        Some(self.values[index])
    }

    pub fn utilization(&self) -> (usize, usize) {
        (self.values.iter()
            .filter(|v| **v != 0)
            .count(),
        Self::SIZE)
    }

    fn index(key: BoardType) -> usize {
        (key % Self::SIZE as BoardType) as usize
    }
}

fn get_prime_greater_than_or_equal(n: BoardType) -> BoardType {
    let mut composite_map: HashMap<BoardType, Vec<BoardType>> = HashMap::new();
    let mut number: BoardType = 2;

    loop {
        match composite_map.remove(&number) {
            None => {
                if number >= n {
                    return number;
                }
                composite_map.insert(number * number, vec![number]);
            }
            Some(primes) => for prime in primes {
                let composite = prime + number;
                match composite_map.get_mut(&composite) {
                    None => {
                        composite_map.insert(composite, vec![prime]);
                    },
                    Some(v) => {
                        v.push(prime);
                    },
                }
            },
        }
        number += 1;
    }
}

fn is_prime(n: BoardType) -> bool {
    if n <= 1 {
        return false;
    }
    let mut potential_factor: BoardType = 2;
    let stop_at: BoardType = (n as f64).sqrt() as BoardType + 1;

    while potential_factor <= stop_at {
        if n % potential_factor == 0 {
            return false;
        }
        potential_factor += 1 + potential_factor % 2;
    }

    return true;
}