use std::{
    fs::File,
    io::{BufWriter, Write},
};

use clap::{Args, Parser};
use nskeyedarchiver_converter::{Converter, ConverterError};

#[derive(Parser)]
#[command(author, version, about)]
struct Arguments {
    /// Path to a NSKeyedArchiver encoded plist
    plist_in: String,

    /// Path to an output file
    file_out: String,

    #[command(flatten)]
    output_format: Option<OutputFormat>,

    /// Leave $null values. By default they're omitted
    #[arg(short = 'n')]
    leave_null: bool,

    /// Treat dictionaries and arrays as regular classes. A $classes key gets retained.
    /// By default those are transformed into native plist structures.
    #[arg(short)]
    treat_all_as_classes: bool,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct OutputFormat {
    /// Export in a plist format (default)
    #[arg(short)]
    plist: bool,

    /// Export in a plist binary format
    #[arg(short = 'b')]
    plist_binary: bool,

    /// Export in a json format
    #[arg(short)]
    json: bool,
}

fn main() -> Result<(), ConverterError> {
    let args = Arguments::parse();
    let mut decoded_file = Converter::from_file(args.plist_in)?;

    decoded_file.set_leave_null_values(args.leave_null);
    decoded_file.set_treat_all_as_classes(args.treat_all_as_classes);
    let decoded_value = decoded_file.decode()?;

    if let Some(output_format) = args.output_format {
        if output_format.plist_binary {
            decoded_value.to_file_binary(args.file_out)?
        } else if output_format.json {
            let json = serde_json::to_string(&decoded_value).unwrap();
            let mut output = File::create(&args.file_out).unwrap();
            let mut writer = BufWriter::new(&mut output);
            writer.write_all(json.as_bytes()).unwrap();
            return Ok(());
        }
    } else {
        decoded_value.to_file_xml(args.file_out)?
    }

    Ok(())
}
