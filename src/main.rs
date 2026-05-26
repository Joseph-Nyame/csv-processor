use clap::Parser;
mod config;
mod converter;
use config::read_config;
use converter::ReadOptions;


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
                    match file_type{
                        FileType::XLSX=>{
                            let options = ReadOptions{
                                file_path: args.file_name,
                                keep: config.columns.keep,
                                filters: config.filters,
                            };
                            let data = converter::read_xlsx(options);
                            match data{
                                Ok(data)=>{
                                    println!("Data: {:?}", data);
                                }
                                Err(error)=>{
                                    println!("Error: {}", error);
                                    std::process::exit(1);
                                }
                            }
                        }
                        _=>{
                            println!("Converter not implemented yet");
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



#[derive(Debug)]
enum FileType{
    XLSX,
    PDF,
    JSON,
    XML,
    SQL,
}

fn detect_file_type(file_name: &str) -> Result<FileType, String> {
  let file_extension = file_name.split('.').last().unwrap();
  match file_extension {
    "xlsx" => Ok(FileType::XLSX),
    "pdf" => Ok(FileType::PDF),
    "json" => Ok(FileType::JSON),
    "xml" => Ok(FileType::XML),
    "sql" => Ok(FileType::SQL),
    _ => Err(format!("Unsupported file type: {}", file_extension)),
    
}
}