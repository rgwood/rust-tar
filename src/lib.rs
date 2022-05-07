use std::str;

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
        None => seq,
    }
}

// Most numbers in tarball header are stored in octal (!?!?), convert to decimal to make life easier
pub fn convert_octal_to_decimal(slice: &[u8]) -> usize {
    let octal = bytes_to_str(slice);
    match usize::from_str_radix(octal, 8) {
        Ok(n) => n,
        Err(std::num::ParseIntError { .. }) => {
            // TODO: log value we failed on
            panic!("Could not parse octal checksum")
        }
    }
}

pub fn bytes_to_str(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).expect("Could not convert bytes to string")
}
