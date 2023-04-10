use super::block::Block;

pub fn calculate_pow(last_proof: &u64) -> u64 {
    let mut incrementor = last_proof + 1;
    while !(incrementor % 9 == 0 && incrementor % last_proof == 0) {
        incrementor = incrementor + 1;
    }
    return incrementor;
}
