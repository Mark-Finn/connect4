use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub struct TranspositionTable
{
    keys: Vec<u32>,
    values: Vec<i8>
}

impl TranspositionTable {

    const SIZE: usize = 3_000_017;

    pub fn new() -> Self {
        Self {
            keys: vec![u32::MAX; Self::SIZE],
            values: vec![0; Self::SIZE],
        }
    }

    pub fn put(&mut self, key: u64, value: i8) {
        let index = Self::index(key);
        self.keys[index] = key as u32;
        self.values[index] = value;
    }

    pub fn get(&self, key: u64) -> Option<i8> {
        let index = Self::index(key);
        if self.keys[index] != key as u32
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

    fn index(key: u64) -> usize {
        (key % Self::SIZE as u64) as usize
    }
}