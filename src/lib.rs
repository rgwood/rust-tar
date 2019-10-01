pub fn index_of_last_byte_in_null_terminated_sequence(seq: &[u8]) -> usize {
    assert!(seq.len() > 0);
    let null_byte: u8 = 0x0;
    for (i, item) in seq.iter().enumerate() {
        if *item == null_byte {
            println!("Found null byte in position {}", i);
            return i - 1;
        }
    }
    seq.len() - 1
}

// Given a null-terminated sequence of bytes, return a slice without the null bytes
pub fn extract_value_from_null_terminated_sequence(seq: &[u8]) -> &[u8] {
    &seq[..index_of_last_byte_in_null_terminated_sequence(seq) + 1]
}