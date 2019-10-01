#[cfg(test)]
mod tests {
    // use super::*;
    use tarlib::*;
    
    #[test]
    fn can_find_last_byte_of_null_terminated_sequence() {
        assert_eq!(index_of_last_byte_in_null_terminated_sequence(b"Hello\0\0\0"), 4);
        assert_eq!(index_of_last_byte_in_null_terminated_sequence(b"Hello"), 4);
        assert_eq!(index_of_last_byte_in_null_terminated_sequence(b"Hello\0\0\0World"), 4);
    }

    #[test]
    #[should_panic]
    fn panic_if_trying_to_parse_empty_null_terminated_seq() {
        index_of_last_byte_in_null_terminated_sequence(b"");
    }
}