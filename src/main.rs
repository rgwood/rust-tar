// Because this one is super annoying during development
// #![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::str;
use structopt::StructOpt;
use tarlib::*;
use std::fs::OpenOptions;
use std::time::{Duration, Instant};

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

const BLOCK_SIZE: usize = 512;

struct Header<'a> {
    bytes: &'a[u8],
    file_name: &'a str,
    checksum: usize,
    file_size_in_bytes: usize
}

impl Header<'_> {
    fn new(bytes: &[u8]) -> Header {
        let ret = Header {
            bytes, 
            file_name: bytes_to_str(truncate_null_terminated_seq(&bytes[..99])),
            checksum: convert_octal_to_decimal(&bytes[148..154]),
            file_size_in_bytes: convert_octal_to_decimal(&bytes[124..135])
        };
        ret.verify_checksum();

        ret
    }

    fn calculate_checksum(&self) -> usize {
        let mut calculated_checksum: usize = 0;
        for b in &self.bytes[..148] {
            calculated_checksum += usize::from(*b) ;
        }   

        // checksum bytes are taken to be ASCII spaces (decimal value 32)
        calculated_checksum += 8 * 32;

        for b in &self.bytes[156..] {
            calculated_checksum += usize::from(*b);
        }   

        calculated_checksum
    }

    fn verify_checksum(&self) {
        let calculated_checksum = self.calculate_checksum();
        if self.checksum != calculated_checksum {
            panic!("Invalid checksum {} for file '{}' in tarball. We calculated {}", self.checksum, self.file_name, calculated_checksum);
        }
    }
}

struct TarballEntry<'a> {
    header: Header<'a>,
    file_bytes: &'a [u8]
}

struct TarballIterator<'a> {
    bytes: &'a [u8],
    byte_offset: usize
}

impl<'a> Iterator for TarballIterator<'a> {
    type Item = TarballEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_offset+BLOCK_SIZE >= self.bytes.len() {
            return None;
        }

        let header_bytes = &self.bytes[self.byte_offset..self.byte_offset+BLOCK_SIZE];

        // let zero_byte_check_start = Instant::now();
        // If we see a record that is all empty bytes, we're done
        if header_bytes.iter().all(|&i| i == 0) {
            return None;
        }
        // println!("Time to check whether 512b block is empty: {:?}", Instant::now().duration_since(zero_byte_check_start));

        let header = Header::new(header_bytes);
        self.byte_offset += BLOCK_SIZE;

        let file_size = header.file_size_in_bytes;
        let file_bytes = &self.bytes[self.byte_offset..self.byte_offset+file_size];
        self.byte_offset += file_size;

        // Round up to a multiple of 512
        self.byte_offset += BLOCK_SIZE - (file_size % BLOCK_SIZE);
        
        Some(TarballEntry {header, file_bytes})
    }
}

fn main() -> std::io::Result<()> {
    let args = CliParams::from_args();

    if args.list_mode || args.extract_mode {
        let tarball_filename = &args.file_name;
        let mut tarball = File::open(tarball_filename).expect("Failed to open file");

        // TODO: do something smarter than reading the entire file into memory, maybe a BufReader
        let mut tarball_contents: Vec<u8> = Vec::new();
        let read_start = Instant::now();
        let file_size_in_bytes = tarball.read_to_end(&mut tarball_contents)?;
        println!("Read all {} bytes from tarball successfully in {:?}", file_size_in_bytes, Instant::now().duration_since(read_start));

        let iterator = TarballIterator {bytes: &tarball_contents[..], byte_offset: 0};
        for entry in iterator {
            let header = entry.header;
            println!("Processing file header: '{}'", header.file_name);
            println!("  Decimal checksum: {}", header.checksum);
            println!("  Calculated checksum: {}", header.calculate_checksum());
            println!("  File size in bytes: {}", header.file_size_in_bytes);
            println!("  File contents: {}", bytes_to_str(entry.file_bytes));

            if args.extract_mode {
                let mut extracted_file = OpenOptions::new()
                                .write(true)
                                .create_new(true)
                                .open(header.file_name)?;
                extracted_file.write_all(entry.file_bytes)?;
                println!("  Extracted file {} successfully", header.file_name);
            }
        }
    }

    Ok(())
}
