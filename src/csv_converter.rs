
use crate::converter::ReadOptions;
use crate::writer::ErrorRecord;
use std::collections::HashMap;
use rayon::prelude::*;


pub fn read_csv(options:ReadOptions)->Result<(Vec<Vec<String>>, Vec<ErrorRecord>), String>
{
    let file_content = std::fs::read_to_string(&options.file_path).map_err(|e| e.to_string())?;
    let mut reader = csv::Reader::from_reader(file_content.as_bytes());
    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    let mut headers_map:HashMap<String,usize> = HashMap::new();
    for (index,header) in headers.iter().enumerate(){
        headers_map.insert(header.to_string().trim().to_string(),index);
    }
    let keep_indices: Vec<usize> = options.keep.iter()
    .filter_map(|col| headers_map.get(col.trim()))
    .copied()
    .collect();
    let mut errors:Vec<ErrorRecord> = Vec::new();
    for col in &options.keep{
        if !headers_map.contains_key(col.trim()){
            errors.push(ErrorRecord {
                row_number: 0,
                reason: format!("Column '{}' not found in file", col),
                row_data: vec![col.clone()],
            });
        }
    }
    if !errors.is_empty() {
        return Err(format!("Missing columns: {}", 
            errors.iter().map(|e| e.reason.clone()).collect::<Vec<_>>().join(", ")));
    }
    let data:Vec<Vec<String>> = reader.records().par_bridge()
    .filter_map(|record| record.ok())
    .filter(|record| {
        options.filters.iter().all(|(col, value)| {
            headers_map.get(col)
                .and_then(|&i| record.get(i))
                .map(|cell| cell.to_string().trim().to_string() == *value)
                .unwrap_or(true)
        })
    }).map(|record| {
        keep_indices.iter()
            .filter_map(|&i| record.get(i))
            .map(|cell| cell.to_string().trim().to_string())
            .collect()
    }).collect();
    Ok((data,errors))
    

}