use calamine::{Reader, open_workbook_auto};
use std::collections::HashMap;
use crate::writer::ErrorRecord;
use rayon::prelude::*;


pub fn read_xlsx(options:ReadOptions)->Result<(Vec<Vec<String>>, Vec<ErrorRecord>), String>
{
    let mut open_workbook = open_workbook_auto(&options.file_path).map_err(|e| e.to_string())?;
    let sheet_name = open_workbook.sheet_names().first().ok_or("No sheets found".to_string())?.clone();
    let work_sheet = open_workbook.worksheet_range(&sheet_name).map_err(|e| e.to_string())?;
    let mut rows=work_sheet.rows();
    let header_row= rows.next().ok_or("No header row found".to_string())?; //get first row as header
    let rows:Vec<_> = rows.collect();
    let mut headers:HashMap<String,usize> = HashMap::new();
    let mut errors:Vec<ErrorRecord> = Vec::new();
    for (index,header) in header_row.iter().enumerate(){
        headers.insert(header.to_string().trim().to_string(),index);
    }
    let keep_indices: Vec<usize> = options.keep.iter()
    .filter_map(|col| headers.get(col.trim()))
    .copied()
    .collect();

    for col in &options.keep {
        if !headers.contains_key(col.trim()) {
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
    let data:Vec<Vec<String>> = rows.par_iter().filter(|row| {
        options.filters.iter().all(|(col, value)| {
            headers.get(col)
                .and_then(|&i| row.get(i))
                .map(|cell| cell.to_string().trim().to_string() == *value)
                .unwrap_or(true)
        })
    }).map(|row| {
        keep_indices.iter()
            .filter_map(|&i| row.get(i))
            .map(|cell| cell.to_string().trim().to_string())
            .collect()
    }).collect();
    Ok((data,errors))
}


#[derive(Debug)]
pub struct ReadOptions{
    pub file_path: String,
    pub keep: Vec<String>,
    pub filters: HashMap<String, String>,
}