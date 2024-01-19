use clap::Parser;
use nskeyedarchive_converter::{Converter, ConverterError};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to a NSKeyedArchive encoded plist
    file_in: String,

    /// Path to an output plist
    file_out: String,

    /// Export plist in a binary format
    #[arg(short, long)]
    binary: bool,
}

fn main() -> Result<(), ConverterError> {
    let args = Args::parse();
    let decoded_file = Converter::from_file(args.file_in)?.decode()?;

    match args.binary {
        true => decoded_file.to_file_binary(args.file_out)?,
        false => decoded_file.to_file_xml(args.file_out)?
    }
    Ok(())
}
