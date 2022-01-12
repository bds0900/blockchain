
use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use serde::{Deserialize, Serialize};
pub type BlockHash = U256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,// or use u32?
    pub timestamp: u64, 
    pub nonce: u64,
    pub previous_hash: BlockHash,
    pub hash: BlockHash,
    pub payload : String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new (index: u64, nonce: u64, previous_hash: BlockHash, transactions: Vec<Transaction>)
        -> Block {
            let mut block = Block {
                index, timestamp
            };
        }
}