use crate::file_type::FileType;
use crate::converter::{self ,ReadOptions};
use crate::writer::write_csv;
use crate::json_converter;
use crate::writer::write_error_report;



pub fn perform_conversion(file_type:FileType, options:ReadOptions, output: &str)-> Result<(), String>
{
    match file_type{
        FileType::XLSX=>{
            let (data,errors) = converter::read_xlsx(options)?;
            write_csv(&data,output)?;
            println!("Data written successfully");
            if !errors.is_empty(){
              let error_path = format!("{}_errors.csv", output.trim_end_matches(".csv"));
              write_error_report(&errors, &error_path)?;
            }
            Ok(())
        }
        FileType::JSON=>{
            let (data,errors) = json_converter::read_json(options)?;
            write_csv(&data,output)?;
            println!("Data written successfully");
            if !errors.is_empty(){
                let error_path = format!("{}_errors.csv", output.trim_end_matches(".csv"));
                write_error_report(&errors, &error_path)?;
            }
            Ok(())
        }
        _=>{
            println!("Converter not implemented yet");
            std::process::exit(1);
        }
    }  
}