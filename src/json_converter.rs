use crate::converter::ReadOptions;
use serde_json;
use crate::writer::ErrorRecord;
use rayon::prelude::*;

pub fn read_json(options:ReadOptions)->Result<(Vec<Vec<String>>, Vec<ErrorRecord>), String>
{
    let file_content = std::fs::read_to_string(&options.file_path).map_err(|e| e.to_string())?;
    let data:serde_json::Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;
    let mut errors:Vec<ErrorRecord> = Vec::new();
    if let Some(first_row) = data.as_array()
    .and_then(|arr| arr.first()) {
        for col in &options.keep {
            if first_row.get(col.as_str()).is_none() {
                errors.push(ErrorRecord {
                    row_number: 0,
                    reason: format!("Column '{}' not found in file", col),
                    row_data: vec![col.clone()],
                });
            }
        }
    }
    if !errors.is_empty() {
        return Err(format!("Missing columns: {}", 
            errors.iter().map(|e| e.reason.clone()).collect::<Vec<_>>().join(", ")));
    }
    
    let data:Vec<Vec<String>> = data.as_array().ok_or("Invalid JSON format".to_string())?.par_iter()
    .filter(|row| {
        options.filters.iter().all(|(col, value)| {
            row[col.as_str()].as_str().unwrap_or("") == *value
        })
    }).map(|row: &serde_json::Value| {
        options.keep.iter()
        .filter_map(|col| row.get(col.as_str()))
        .map(|cell| cell.as_str()
        .map(|s| s.to_string())
        .or_else(|| cell.as_i64().map(|n| n.to_string()))
        .or_else(|| cell.as_f64().map(|n| n.to_string()))
        .or_else(|| cell.as_bool().map(|b| b.to_string()))
        .unwrap_or_default())
            .collect()
    }).collect();
    Ok((data,errors))
    
}