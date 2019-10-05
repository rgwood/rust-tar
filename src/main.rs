// Because this one is super annoying during development
#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::str;
use structopt::StructOpt;
use tarlib::*;
use std::fs::OpenOptions;

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

struct Header<'a> {
    bytes: &'a[u8],
    file_name: &'a str,
    checksum: usize,
    file_size_in_bytes: usize
}

impl Header<'_> {
    fn new(bytes: &[u8]) -> Header {
        Header 
        {bytes, 
        file_name: bytes_to_str(truncate_null_terminated_seq(&bytes[..99])),
        checksum: convert_octal_to_decimal(&bytes[148..154]),
        file_size_in_bytes: convert_octal_to_decimal(&bytes[124..135])
        }
    }

    fn calculate_checksum(&self) -> usize {
        let mut calculated_checksum: usize = 0;
        for b in &self.bytes[..148] {
            calculated_checksum += usize::from(*b) ;
        }   

        // checksum bytes are taken to be ASCII spaces (decimal value 32)
        calculated_checksum += 8 * 32;

        for b in &self.bytes[156..512] {
            calculated_checksum += usize::from(*b);
        }   

        calculated_checksum
    }
}



const TAR_HEADER_LENGTH_IN_BYTES: usize = 512;

fn main() -> std::io::Result<()> {
    let args = CliParams::from_args();
    println!("extract mode: {}", args.extract_mode);
    //let args: Vec<String> = env::args().collect();

    // if args.len() > 1 { // 1st "argument" is always the binary name
    //     println!("Arguments: {:?}", &args[1..]);
    // } else {
    //     println!("No arguments provided. Program name: {}", args[0]);
    // }

    if args.list_mode || args.extract_mode {
        let tarball_filename = &args.file_name;
        let mut tarball = File::open(tarball_filename).expect("Failed to open file");

        // TODO: do something smarter than reading the entire file into memory, maybe a BufReader
        let mut tarball_contents: Vec<u8> = Vec::new();
        let file_size_in_bytes = tarball.read_to_end(&mut tarball_contents)?;
        println!("Read all {} bytes from file '{}' successfully ", file_size_in_bytes, tarball_filename);

        let header = Header::new(&tarball_contents[..TAR_HEADER_LENGTH_IN_BYTES]);

        println!("File name from tarball: '{}'", header.file_name);

        // TODO: verify checksum.
        println!("Decimal checksum: {}", header.checksum);
        println!("Calculated checksum: {}", header.calculate_checksum());

        // let file_size_bytes = convert_octal_to_decimal(&tar_header[124..135]);

        println!("File size in bytes: {}", header.file_size_in_bytes);

        let file_contents = &tarball_contents[TAR_HEADER_LENGTH_IN_BYTES..TAR_HEADER_LENGTH_IN_BYTES+header.file_size_in_bytes];

        println!("File contents: {}", bytes_to_str(file_contents));

        if args.extract_mode {
            // let path = Path::new(file_name_from_header);
            let mut extracted_file = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(header.file_name)?;
            extracted_file.write_all(file_contents)?;
            println!("Extracted file {} successfully", header.file_name);
        }

    }

    Ok(())
}

// Most numbers in tarball header are stored in octal (!?!?), convert to decimal to make life easier
fn convert_octal_to_decimal(slice: &[u8]) -> usize {
    let octal = bytes_to_str(slice);
    match usize::from_str_radix(octal, 8) {
        Ok(n) => n,
        Err(std::num::ParseIntError { .. }) => {
            // TODO: log value we failed on
            panic!("Could not parse octal checksum")
            }
    }
}

fn bytes_to_str(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).expect("Could not convert file name header to string")
}
