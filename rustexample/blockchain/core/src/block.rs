use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use utils::coder;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,        //time stamp
    pub tx_hash: String,  // transactions merkle hash
    pub pre_hash: String, //hash of the pre block
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String, //hssh of the block header
    pub data: String, //transactions data
}

impl Block {
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn new_block(data: String, pre_hash: String) -> Block {
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions);

        let time = Utc::now().timestamp();
        let mut block = Block {
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash,
                pre_hash: pre_hash,
            },
            hash: "".to_string(),
            data: data,
        };
        block.set_hash();
        block
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
