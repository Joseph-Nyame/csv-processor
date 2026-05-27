

#[derive(Debug)]
pub enum FileType{
    XLSX,
    PDF,
    JSON,
    XML,
    SQL,
}

pub fn detect_file_type(file_name: &str) -> Result<FileType, String> {
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