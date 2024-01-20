
# NSKeyedArchiver Converter

![A demo image of NSKeyedArchiver Converter](https://raw.githubusercontent.com/michaelwright235/nskeyedarchiver_converter/master/demo.png)

Convert NSKeyedArchiver encoded plists to human readable formats.

Apple is known for inventing and using their own proprietary formats. Many programs across Apple OSes use NSKeyedArchiver to serialize and store custom objects. The problem is, reading it outside of Apple ecosystem maybe problematic. The format itself is just a binary plist, however it uses some techniques to store objects efficiently. Therefore figuring out what objects are encoded there is hard.

This tool helps you with converting NSKeyedArchiver encoded plists to human readable **regular plists, binary plists or JSON files** for further analysis.

A demo image demonstates an original and a converted file opened with [Xplist](https://github.com/ic005k/Xplist).

## Command line tool

```text
Convert NSKeyedArchiver encoded plists to human readable formats

Usage: nskeyedarchiver_converter [OPTIONS] <PLIST_IN> <FILE_OUT>

Arguments:
  <PLIST_IN>  Path to a NSKeyedArchiver encoded plist
  <FILE_OUT>  Path to an output file

Options:
  -p             Export in a plist format (default)
  -b             Export in a plist binary format
  -j             Export in a json format
  -n             Leave $null values. By default they're omitted
  -t             Treat dictionaries and arrays as regular classes. A $classes key gets retained. By default those are transformed into native plist structures
  -h, --help     Print help
  -V, --version  Print version
```

For instance, if you want to convert `foo.bin` to `foo.plist` run the following command:
`nskeyedarchiver_converter ./foo.bin ./foo.plist`.

## Rust library

Use `from_file`, `from_bytes`, `from_reader` or `new` method of `nskeyedarchiver_converter::Converter` to read an existing NSKeyedArchiver encoded file. Then call `decode` method that returns `plist::Value`. Under the hood this library uses [plist](https://crates.io/crates/plist) crate, so look at [their documentation](https://docs.rs/plist/latest/plist/) for further details.

```rust
use nskeyedarchiver_converter::Converter;

Converter::from_file("./foo.bin")?
    .decode()?
    .to_file_xml("./foo.plist")?;
```
