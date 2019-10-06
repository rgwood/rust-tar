# rust-tar

A Rust implementation of the [tar](https://www.gnu.org/software/tar/manual/tar.html) archive utility. 

### Should I use this?

Probably not. The `tar` binary that came with your OS has more features and has been tested against millions of archives, use it instead.

### So what's it good for?

Building a tool like this is a really good way to learn Rust syntax+features! Some things I learned:

1. How to work with raw binary data
1. How to create iterators over arbitrary data
1. How to parse+validate CLI parameters using [StructOpt](https://github.com/TeXitoi/structopt)
1. Integration tests using [assert_cmd](https://github.com/assert-rs/assert_cmd)
1. Rust's built-in [unit tests](blob/master/tests/byte-sequences.rs)
1. File I/O
1. Option and Result types (I *love* not having to deal with nullable types and exceptions)

### How to use

Run with `--help` to see the list of options:

```
./rust-tar --help

rust-tar 0.1.1

USAGE:
    rust-tar [FLAGS] <file-name>

FLAGS:
    -x               Extract mode: extract an archive to disk
    -h, --help       Prints help information
    -t               List mode: list archive contents
    -V, --version    Prints version information

ARGS:
    <file-name>    The path to a tarball
```

Run with `-t` to show file contents (without saving to disk), or `-x` to save to disk:

```
./rust-tar -t test-files/multiple-files.tar

Read all 3072 bytes from tarball successfully 
Processing file header: 'ascii.txt'
  Decimal checksum: 6310
  Calculated checksum: 6310
  File size in bytes: 11
  File contents: Hello, tar!
Processing file header: 'ascii2.txt'
  Decimal checksum: 6354
  Calculated checksum: 6354
  File size in bytes: 17
  File contents: Hello again, tar!
```