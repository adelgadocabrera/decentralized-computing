use super::block::Block;
use std::{
    fmt,
    fs::File,
    io::{Error, Read},
};

pub fn calculate_pow(last_proof: &u64) -> u64 {
    let mut incrementor = last_proof + 1;
    while !(incrementor % 9 == 0 && incrementor % last_proof == 0) {
        incrementor = incrementor + 1;
    }
    return incrementor;
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

#[derive(Debug, Clone)]
pub struct Miner {
    pub addr: [u8; 16],
}

impl Miner {
    pub fn sign() {}
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
