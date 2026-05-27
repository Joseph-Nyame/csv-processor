use crate::file_type::FileType;
use crate::converter::{self ,ReadOptions};
use crate::writer::write_csv;
use crate::json_converter;



pub fn perform_conversion(file_type:FileType, options:ReadOptions, output: &str)-> Result<(), String>
{
    match file_type{
        FileType::XLSX=>{
            let read_options = ReadOptions{
                file_path: options.file_path,
                keep: options.keep,
                filters: options.filters,
            };
            let data = converter::read_xlsx(read_options);
            match data{
                Ok(data)=>{
                    println!("Data: {:?}", data);
                    write_csv(&data, output)?;
                    println!("Data written successfully");
                    Ok(())
                }
                Err(error)=>{
                    println!("Error: {}", error);
                    std::process::exit(1);
                }
            }
        }
        FileType::JSON=>{
            let read_options = ReadOptions{
                file_path: options.file_path,
                keep: options.keep,
                filters: options.filters,
            };
            let data = json_converter::read_json(read_options);
            match data{
                Ok(data)=>{
                    println!("Data{:?}",data);
                    write_csv(&data,output)?;
                    println!("Data written successfully");
                    Ok(())
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
}