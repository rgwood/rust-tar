use std::fs::File;
use std::io::prelude::*;
use std::str;
use structopt::StructOpt;
use tarlib::*;

#[derive(StructOpt)]
struct CliParams {
    /// Extract mode: extract an archive to disk
    #[structopt(short = "x")]
    extract_mode: bool,
    /// List mode: list archive contents
    #[structopt(short = "t")]
    list_mode: bool,
    /// The path to a tarball
    #[structopt(required_if("extract_mode", "true"))]
    file_name: String
}

fn main() -> std::io::Result<()> {
    let args = CliParams::from_args();
    println!("extract mode: {}", args.extract_mode);
    //let args: Vec<String> = env::args().collect();

    // if args.len() > 1 { // 1st "argument" is always the binary name
    //     println!("Arguments: {:?}", &args[1..]);
    // } else {
    //     println!("No arguments provided. Program name: {}", args[0]);
    // }

    if args.list_mode {
        let file_name = &args.file_name;
        let mut file = File::open(file_name).expect("Failed to open file");;

        // TODO: do something smarter than reading the entire file into memory, maybe a BufReader
        let mut file_contents: Vec<u8> = Vec::new();
        let file_size_in_bytes = file.read_to_end(&mut file_contents)?;
        println!("Read all {} bytes from file '{}' successfully ", file_size_in_bytes, file_name);

        let tar_header: &[u8] = &file_contents[..512];

        let header_file_name = extract_value_from_null_terminated_sequence(&tar_header[..99]);

        println!("File name from tarball: '{}'", bytes_to_str(header_file_name));

        // TODO: verify checksum.

        println!("Decimal checksum: {}", read_header_checksum_in_decimal(tar_header));
        println!("Calculated checksum: {}", calculate_header_checksum(tar_header));
    }

    Ok(())
}

// Tarball header checksums are stored in octal (!?!?), convert to decimal to make our life easier
fn read_header_checksum_in_decimal(header: &[u8]) -> u32 {
    let octal_checksum = bytes_to_str(&header[148..154]);
    println!("Octal checksum: '{}'", octal_checksum);
    match u32::from_str_radix(octal_checksum, 8) {
        Ok(n) => n,
        Err(_) => {panic!("Could not parse octal checksum")}
    }
}

fn calculate_header_checksum(header: &[u8]) -> u32 {
    let mut calculated_checksum: u32 = 0;
    for b in &header[..148] {
        calculated_checksum += *b as u32;
    }

    // checksum bytes are taken to be ASCII spaces (decimal value 32)
    calculated_checksum += 8 * 32;
    
    for b in &header[156..512] {
        calculated_checksum += *b as u32;
    }

    calculated_checksum
}

fn bytes_to_str(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).expect("Could not convert file name header to string")
}
