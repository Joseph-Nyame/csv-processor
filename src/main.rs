use clap::Parser;
mod config;
mod converter;
mod writer;
mod json_converter;
mod file_type;
mod processor;
mod csv_converter;
use config::read_config;
use converter::ReadOptions;
use file_type::detect_file_type;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args = Args::parse();
    let file_type = detect_file_type(&args.file_name)?;
    println!("File type: {:?}", file_type);
    println!("Args: {:?}", &args);
    let config=read_config(&args.config)?;
    println!("Config: {:?}", config);
    let options = ReadOptions{
        file_path: args.file_name,
        keep: config.columns.keep,
        filters: config.filters,
    };
    processor::perform_conversion(file_type, options, &args.output)?;
    Ok(())
   

}




#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {

    /// The name of the file to process
    #[arg(short, long)]
    file_name: String,

    /// The name of the config file containing column mappings to use in extraction
    #[arg(short, long)]
    config: String,

    /// The name of the output file to use
    #[arg(short, long)]
    output: String,

}

