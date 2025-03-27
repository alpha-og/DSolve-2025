use crate::middlewares::{block::Block, order_data::OrderData};
use rand::thread_rng;
use rsa::{RsaPrivateKey, RsaPublicKey};

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        let genesis_data = OrderData::new(
            0,          // order_id: u32
            0,          // quantity: i32
            0,          // price: u32
            0,          // seller_id: u32
            None,       // pickup_point: Option<String>
            Vec::new(), // delivery_point: Vec<u8>
            None,       // delivery_partner_id: Option<u32>
            public_key, // public_key: RsaPublicKey
        );
        let genesis_block = Block::new(0, "".to_string(), genesis_data, difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: OrderData) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            data,
            self.difficulty,
        );
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> usize {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.prev_hash != previous_block.hash {
                return i;
            }

            if current_block.hash
                != Block::calculate_hash(
                    current_block.index,
                    &previous_block.hash,
                    current_block.timestamp,
                    &current_block.data,
                    current_block.nonce,
                )
            {
                return i;
            }
        }
        0
    }
}

#[test]
fn test_blockchain_new() {
    let mut rng = thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    let mut blockchain = Blockchain::new(1);
    let test_data = OrderData::new(
        1,                      // order_id: u32
        2,                      // quantity: i32
        100,                    // price: u32
        123,                    // seller_id: u32
        Some("Store A".into()), // pickup_point: Option<String>
        Vec::new(),             // delivery_point: Vec<u8>
        None,                   // delivery_partner_id: Option<u32>
        public_key,             // public_key: RsaPublicKey
    );
    blockchain.add_block(test_data);
    assert_eq!(blockchain.is_valid(), 0);
}
