pub fn index_of_first_null_byte(seq: &[u8]) -> Option<usize> {
    let null_byte: u8 = 0x0;
    for (i, item) in seq.iter().enumerate() {
        if *item == null_byte {
            return Some(i);
        }
    }
    None
}

// Given a null-terminated sequence of bytes, return a slice without the null bytes
pub fn truncate_null_terminated_seq(seq: &[u8]) -> &[u8] {
    let first_null_i = index_of_first_null_byte(seq);
    match first_null_i {
        Some(index) => &seq[..index],
        None => seq
    }
}