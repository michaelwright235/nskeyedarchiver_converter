use clap::Parser;
use nskeyedarchiver_converter::{Converter, ConverterError};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to a NSKeyedArchive encoded plist
    file_in: String,

    /// Path to an output plist
    file_out: String,

    /// Export plist in a binary format
    #[arg(short)]
    binary: bool,

    /// Leave $null values. By default they're omitted
    #[arg(short = 'n')]
    leave_null: bool,

    /// Treat dictionaries and arrays as regular classes. A $classes key gets retained.
    /// By default those are transformed into native plist structures.
    #[arg(short)]
    treat_all_as_classes: bool
}

fn main() -> Result<(), ConverterError> {
    let args = Args::parse();
    let mut decoded_file = Converter::from_file(args.file_in)?;

    decoded_file.set_leave_null_values(args.leave_null);
    decoded_file.set_treat_all_as_classes(args.treat_all_as_classes);

    match args.binary {
        true => decoded_file.decode()?.to_file_binary(args.file_out)?,
        false => decoded_file.decode()?.to_file_xml(args.file_out)?
    }
    Ok(())
}
