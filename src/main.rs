use std::fs::File;
use std::io::prelude::*;
use std::str;
use structopt::StructOpt;
use tarlib::*;
use std::fs::OpenOptions;
// use std::path::Path;

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
        let mut tarball = File::open(tarball_filename).expect("Failed to open file");;

        // TODO: do something smarter than reading the entire file into memory, maybe a BufReader
        let mut tarball_contents: Vec<u8> = Vec::new();
        let file_size_in_bytes = tarball.read_to_end(&mut tarball_contents)?;
        println!("Read all {} bytes from file '{}' successfully ", file_size_in_bytes, tarball_filename);

        let tar_header: &[u8] = &tarball_contents[..TAR_HEADER_LENGTH_IN_BYTES];

        let file_name_from_header = bytes_to_str(truncate_null_terminated_seq(&tar_header[..99]));

        println!("File name from tarball: '{}'", file_name_from_header);

        // TODO: verify checksum.
        println!("Decimal checksum: {}", convert_octal_to_decimal(&tar_header[148..154]));
        println!("Calculated checksum: {}", calculate_header_checksum(tar_header));

        let file_size_bytes = convert_octal_to_decimal(&tar_header[124..135]);

        println!("File size in bytes: {}", file_size_bytes);

        let file_contents = &tarball_contents[TAR_HEADER_LENGTH_IN_BYTES..TAR_HEADER_LENGTH_IN_BYTES+file_size_bytes];

        println!("File contents: {}", bytes_to_str(file_contents));

        if args.extract_mode {
            // let path = Path::new(file_name_from_header);
            let mut extracted_file = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(file_name_from_header)?;
            extracted_file.write_all(file_contents)?;
            println!("Extracted file {} successfully", file_name_from_header);
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

fn calculate_header_checksum(header: &[u8]) -> u32 {
    let mut calculated_checksum: u32 = 0;
    for b in &header[..148] {
        calculated_checksum += u32::from(*b) ;
    }

    // checksum bytes are taken to be ASCII spaces (decimal value 32)
    calculated_checksum += 8 * 32;
    
    for b in &header[156..512] {
        calculated_checksum += u32::from(*b);
    }

    calculated_checksum
}

fn bytes_to_str(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).expect("Could not convert file name header to string")
}
