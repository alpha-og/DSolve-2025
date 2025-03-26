use chrono::Utc;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub hash: String,
    pub prev_hash: String,
    pub data: String,
    pub difficulty: u32,
    pub nonce: u32,
}

impl Block {
    pub fn new(index: u32, prev_hash: String, data: String, difficulty: u32) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut nonce: u32 = 0;
        let mut hash = Self::calcuate_hash(index, &prev_hash, timestamp, &data, nonce);
        while !hash.starts_with(&"0".repeat(difficulty as usize)) {
            nonce += 1;
            hash = Self::calcuate_hash(index, &prev_hash, timestamp, &data, nonce);
        }
        Block {
            index,
            timestamp,
            hash,
            prev_hash,
            data,
            difficulty,
            nonce,
        }
    }

    pub fn calcuate_hash(
        index: u32,
        pre_hash: &str,
        timestamp: i64,
        data: &str,
        nonce: u32,
    ) -> String {
        let mut hash = Sha256::new();
        let input = format!("{}{}{}{}{}", index, pre_hash, timestamp, data, nonce);
        hash.update(input);
        format!("{:x}", hash.finalize())
    }
}

#[test]
fn test_create_new() {
    let block = Block::new(0, String::from("123"), String::from("Hello world!"), 1);
    println!("{:?}", block);
    assert_eq!(block.index, 0);
    assert_eq!(block.data, "Hello world!");
    assert_eq!(block.difficulty, 1);
}
