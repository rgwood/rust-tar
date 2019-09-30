use std::fs::File;
use std::io::prelude::*;
// use std::io::ErrorKind;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 { // 1st "argument" is always the binary name
        println!("Arguments: {:?}", &args[1..]);
    } else {
        println!("No arguments provided. Program name: {}", args[0]);
    }

    let file_name = &args[1];
    let mut file = File::open(file_name).expect("Failed to open file");;
    // let mut file = match file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => panic!("File '{}' not found", file_name),
    //         other_error => panic!("Unexpected error opening the file: {:?}", other_error)
    //     }
    // };

    let mut file_contents: Vec<u8> = Vec::new();
    let file_size_in_bytes = file.read_to_end(&mut file_contents)?;

    println!("Read all {} bytes from file '{}' successfully ", file_size_in_bytes, file_name);

    let tar_header: &[u8] = &file_contents[..511];
    let header_file_name_section = &tar_header[..99];

    let null_byte: u8 = 0x0;

    let mut file_name_length_in_bytes = 100;
    for (i, item) in tar_header.iter().enumerate() {
        if *item == null_byte {
            println!("Found null byte in position {}", i);
            file_name_length_in_bytes = i;
            break;
        }
    }
    
    let header_file_name = &header_file_name_section[..file_name_length_in_bytes];

    println!("File name from tarball: {}", std::str::from_utf8(header_file_name).expect("Could not convert file name header to string"));

    //file.write_all(file_name.as_bytes())?;
    // println!("Hello, world!");
    Ok(())
}

#[test]
fn test_test() {
    assert_eq!(1, 2);
}