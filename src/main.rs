use clap::Parser;
mod config;
mod converter;
mod writer;
mod json_converter;
mod file_type;
mod processor;
use config::read_config;
use converter::ReadOptions;
use file_type::detect_file_type;

fn main() {
    let args = Args::parse();
    let file_type = detect_file_type(&args.file_name);
    match file_type {
        Ok(file_type) => {
            println!("File type: {:?}", file_type);
            println!("Args: {:?}", &args);
            let config=read_config(&args.config);
             match config{
                Ok(config)=>{
                    println!("Config: {:?}", config);
                    let options = ReadOptions{
                        file_path: args.file_name,
                        keep: config.columns.keep,
                        filters: config.filters,
                    };
                    let result = processor::perform_conversion(file_type, options, &args.output);
                    match result{
                        Ok(_) => {
                        }
                        Err(error) => {
                            println!("Error: {}", error);
                            std::process::exit(1);
                        }
                    }
                },
                Err(error) =>{
                    println!("Error: {}", error);
                    std::process::exit(1);
                }
            }
        },
        Err(error) =>{
            println!("Error: {}", error);
            std::process::exit(1);
        }
    }
   

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

