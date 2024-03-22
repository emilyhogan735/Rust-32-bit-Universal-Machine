/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {

    // mask to extract the desired bit field
    let mask = ((1u64 << width) - 1) << lsb;

    // return the extracted bit field
    (word & mask) >> lsb
}