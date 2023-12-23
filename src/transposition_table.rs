use std::fs;
use std::io::Write;
use bincode::{config, Decode, Encode};
use bincode::config::Configuration;

const FILEPATH: &str = "./data/transposition_table";

#[derive(Encode, Decode)]
pub struct TranspositionTable
{
    keys: Vec<u64>,
    values: Vec<i8>
}

impl TranspositionTable {

    const SIZE: u64 = (1 << 23) + 9;

    pub fn new() -> Self {
        Self {
            keys: vec![u64::MAX; Self::SIZE as usize],
            values: vec![0; Self::SIZE as usize],
        }
    }

    pub fn from_file_or_new(filepath: &str) -> Self {
        return match &fs::read(filepath) {
            Ok(encoded) => {
                match bincode::decode_from_slice(&encoded[..], Self::bincode_config()) {
                    Ok((transposition_table, _)) => {
                        transposition_table
                    },
                    Err(_) => Self::new(),
                }
            }
            Err(_) => Self::new(),
        };
    }

    pub fn put(&mut self, key: u64, value: i8) {
        let index = Self::index(key);
        self.keys[index] = key;
        self.values[index] = value;
    }

    pub fn get(&self, key: u64) -> Option<i8> {
        let index = Self::index(key);
        if self.keys[index] != key
        {
            return None;
        }
        Some(self.values[index])
    }

    pub fn save(&self) -> std::io::Result<()> {
        let encoded: Vec<u8> = bincode::encode_to_vec(self, Self::bincode_config()).unwrap();

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(FILEPATH)?;
        file.write_all(&*encoded)
    }

    fn index(key: u64) -> usize {
        (key % Self::SIZE) as usize
    }

    fn bincode_config() -> Configuration {
        config::standard()
    }
}