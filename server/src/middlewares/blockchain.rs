use crate::middlewares::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let genesis_block = Block::new(0, "".to_string(), "Genesis Block".to_string(), difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
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
    let mut blockchain = Blockchain::new(1);
    blockchain.add_block(String::from("Sharon"));
    assert_eq!(blockchain.is_valid(), 0);
}
