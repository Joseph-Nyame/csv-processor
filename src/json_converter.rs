use crate::converter::ReadOptions;
use serde_json;

pub fn read_json(options:ReadOptions)->Result<Vec<Vec<String>>,String>
{
    let file_content = std::fs::read_to_string(&options.file_path).map_err(|e| e.to_string())?;
    let data:serde_json::Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;
    let mut result:Vec<Vec<String>> = Vec::new();
    for row in data.as_array().ok_or("Invalid JSON format".to_string())?{
        let mut new_row:Vec<String> = Vec::new();
        let mut passes = true;
        for (col,val) in &options.filters{
            if row[col.as_str()].as_str().unwrap_or("") != *val{
                passes = false;
                break;
            }
        }
        if passes{
            for col in &options.keep{
                let cell = &row[col.as_str()];
                let value =cell.as_str()
                .map(|s| s.to_string())
                .or_else(|| cell.as_i64().map(|n| n.to_string()))
                .or_else(|| cell.as_f64().map(|n| n.to_string()))
                .or_else(|| cell.as_bool().map(|b| b.to_string()))
                .unwrap_or("".to_string());
                new_row.push(value);
            }
            result.push(new_row);
        }
    }
    Ok(result)
    
}