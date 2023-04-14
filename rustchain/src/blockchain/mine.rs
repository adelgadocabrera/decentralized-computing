use sha2::{Digest, Sha256};

use super::block::BlockHeader;
use std::{
    fmt,
    fs::File,
    io::{Error, Read},
};

#[derive(Debug, Clone)]
pub struct Miner {
    pub addr: [u8; 16],
}

impl Miner {
    pub fn sign() {}
}

pub fn new_miner() -> Result<Miner, Error> {
    let mut file = File::open("/dev/urandom")?;
    let mut addr = [0u8; 16];
    file.read_exact(&mut addr)?;

    // Set version (4) and variant (RFC4122)
    addr[6] = (addr[6] & 0x0F) | 0x40;
    addr[8] = (addr[8] & 0x3F) | 0x80;

    Ok(Miner { addr })
}

impl fmt::Display for Miner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.addr[4], self.addr[5], self.addr[6],
            self.addr[7], self.addr[8], self.addr[9], self.addr[10], self.addr[11], self.addr[12], self.addr[13], self.addr[14], self.addr[15]
        )
    }
}

pub fn proof_of_work(header: &mut BlockHeader) -> (Vec<u8>, u64) {
    let mut nonce = 0;
    let mut hash: Vec<u8> = vec![];
    while !satisfies_difficulty(&hash, header.difficulty.to_owned()) {
        nonce = nonce + 1;
        header.nonce = nonce;
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&header).unwrap());
        hash = hasher.finalize().to_vec();
    }
    return (hash.to_vec(), nonce);
}

fn satisfies_difficulty(hash: &Vec<u8>, difficulty: u64) -> bool {
    let mut counter = 0;
    for &byte in hash {
        for i in (0..8).rev() {
            if byte & (1 << i) == 0 {
                counter += 1;
            } else {
                if counter >= difficulty {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
    if counter >= difficulty {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::blockchain::block::{create_genesis_block, Block};

    use super::*;

    #[test]
    fn test_leading_zeroes_difficulty_12() {
        let hash = vec![0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00]; // Three leading zeroes
        assert!(satisfies_difficulty(&hash, 12));
    }

    #[test]
    fn test_leading_zeroes_difficulty_48() {
        let hash = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Eight leading zeroes
        assert!(satisfies_difficulty(&hash, 48));
    }

    #[test]
    fn test_leading_zeroes_difficulty_4() {
        let hash = vec![0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Eight leading zeroes
        assert!(satisfies_difficulty(&hash, 4));
    }

    #[test]
    fn test_leading_zeroes_difficulty_4_fails() {
        let hash = vec![0b00010000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Eight leading zeroes
        assert!(!satisfies_difficulty(&hash, 4));
    }

    #[test]
    fn test_leading_zeroes_difficulty_0() {
        let hash = vec![0xa1, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]; // No leading zeroes
        assert!(satisfies_difficulty(&hash, 0));
    }

    #[test]
    fn test_proof_of_work() {
        let difficulty = 15;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let genesis = create_genesis_block();
        let mut header = BlockHeader {
            timestamp,
            previous_hash: genesis.block_hash,
            block_index: 1,
            merkle_root: vec![],
            difficulty,
            nonce: 0,
        };
        let (hash, nonce) = proof_of_work(&mut header);
        assert!(satisfies_difficulty(&hash, difficulty));
        assert_eq!(nonce, header.nonce);
    }

    #[test]
    fn test_proof_of_work_2_blocks() {
        let difficulty = 15;
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let genesis = create_genesis_block();
        let mut header_1 = BlockHeader {
            timestamp,
            previous_hash: genesis.block_hash,
            block_index: 1,
            merkle_root: vec![],
            difficulty,
            nonce: 0,
        };
        let (hash_1, nonce_1) = proof_of_work(&mut header_1);
        timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut header_2 = BlockHeader {
            timestamp,
            previous_hash: hash_1.to_owned(),
            block_index: 2,
            merkle_root: vec![],
            difficulty,
            nonce: 0,
        };
        let (hash_2, nonce_2) = proof_of_work(&mut header_2);
        assert!(satisfies_difficulty(&hash_1, difficulty));
        assert_eq!(nonce_1, header_1.nonce);
        assert!(satisfies_difficulty(&hash_2, difficulty));
        assert_eq!(nonce_2, header_2.nonce);
    }
}
