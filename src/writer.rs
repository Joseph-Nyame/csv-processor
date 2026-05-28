use csv::Writer;

pub fn write_csv(data: &Vec<Vec<String>>, output_path: &str) -> Result<(), String> {
    let mut writer = Writer::from_path(output_path).map_err(|e| e.to_string())?;
    for row in data {
        writer.write_record(row).map_err(|e| e.to_string())?;
    }
    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn write_error_report(errors: &Vec<ErrorRecord>, output_path: &str) -> Result<(), String> {
    let mut writer = Writer::from_path(output_path).map_err(|e| e.to_string())?;
    writer.write_record(&["Row Number","Reason","Row Data"]).map_err(|e| e.to_string())?;
    for error in errors {
        writer.write_record(&[error.row_number.to_string(), error.reason.clone(), error.row_data.join(",")]).map_err(|e| e.to_string())?;
    }
    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}


#[derive(Debug)]
pub struct ErrorRecord{
    pub row_number:usize,
    pub reason:String,
    pub row_data:Vec<String>,
}