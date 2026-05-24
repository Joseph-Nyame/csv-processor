use std::collections::HashMap;
use serde::Deserialize;

pub fn read_config(file_path: &str)->Result<Config,String>
{
    let file_content= std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let config=toml::from_str(&file_content).map_err(|e| e.to_string())?;
    Ok(config)
    
}

#[derive(Deserialize, Debug)]
pub struct Config{
    columns: Columns,
    filters: HashMap<String, String>,
}


#[derive(Deserialize, Debug)]
pub struct Columns{
    pub keep: Vec<String>,
    
}