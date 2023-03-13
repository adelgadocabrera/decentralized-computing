use blockchain::Block;
use blockchain::BlockBody;
use blockchain::BlockHeader;

pub mod blockchain {
    tonic::include_proto!("blockchain");
}

fn new_block() -> Block {
    let mut block = Block::default();
    block.header = Some(BlockHeader::default());
    block.body = Some(BlockBody::default());
    return block;
}

impl Block {
    fn calculate_hash(&self) {
        print!("block hash calculated!");
    }
}

fn main() {
    let block: Block = new_block();
    println!("{:?}", block);
    block.calculate_hash();
}
