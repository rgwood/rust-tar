#[cfg(test)]
mod tests {
    // use super::*;
    use tarlib::*;

    #[test]
    fn can_find_last_byte_of_null_terminated_sequence() {
        assert_eq!(index_of_first_null_byte(b"Hello\0\0\0"), Some(5));
        assert_eq!(index_of_first_null_byte(b"Hello"), None);
        assert_eq!(index_of_first_null_byte(b"Hello\0\0\0World"), Some(5));
    }

    #[test]
    fn can_extract_byte_string_from_null_terminated_sequence() {
        assert_eq!(truncate_null_terminated_seq(b"Hello\0\0\0"), b"Hello");
        assert_eq!(truncate_null_terminated_seq(b"Hello"), b"Hello");
        assert_eq!(truncate_null_terminated_seq(b""), b"");
    }
}
