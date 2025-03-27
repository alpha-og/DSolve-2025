use crate::middlewares::order_data::OrderData;
use chrono::Utc;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub hash: String,
    pub prev_hash: String,
    pub data: OrderData,
    pub difficulty: u32,
    pub nonce: u32,
}

impl Block {
    pub fn new(index: u32, prev_hash: String, data: OrderData, difficulty: u32) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut nonce: u32 = 0;
        let mut hash = Self::calculate_hash(index, &prev_hash, timestamp, &data, nonce);
        while !hash.starts_with(&"0".repeat(difficulty as usize)) {
            nonce += 1;
            hash = Self::calculate_hash(index, &prev_hash, timestamp, &data, nonce);
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

    pub fn calculate_hash(
        index: u32,
        pre_hash: &str,
        timestamp: i64,
        data: &OrderData,
        nonce: u32,
    ) -> String {
        let mut hash = Sha256::new();
        let input = format!("{}{}{}{:?}{}", index, pre_hash, timestamp, data, nonce);
        hash.update(input);
        format!("{:x}", hash.finalize())
    }
}

#[test]
fn test_create_new() {
    use rsa::{RsaPrivateKey, RsaPublicKey};
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    let data = OrderData::new(
        123,        // order_id: u32
        1,          // quantity: i32
        100,        // price: u32
        456,        // seller_id: u32
        None,       // pickup_point: Option<String>
        Vec::new(), // delivery_point: Vec<u8>
        None,       // delivery_partner_id: Option<u32>
        public_key, // public_key: RsaPublicKey
    );

    let block = Block::new(0, String::from("123"), data, 1);
    assert_eq!(block.index, 0);
    assert_eq!(block.data.order_id, 123);
    assert_eq!(block.difficulty, 1);
}
