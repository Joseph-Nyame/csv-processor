use csv::Writer;

pub fn write_csv(data: &Vec<Vec<String>>, output_path: &str) -> Result<(), String> {
    let mut writer = Writer::from_path(output_path).map_err(|e| e.to_string())?;
    for row in data {
        writer.write_record(row).map_err(|e| e.to_string())?;
    }
    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}