# CSV Processor

## What is this?
A high-performance CLI tool that accepts multiple file formats (XLSX, JSON, CSV), 
applies column selection and row filtering via a TOML config, and outputs a clean 
validated CSV. 

## Usage
cargo run -- --file-name input.xlsx --config transform.toml --output result.csv

## Supported formats
- XLSX (Excel)
- JSON
- CSV

## Config format (eg transform.toml)
[columns]
keep = ["name", "email", "age"]

[filters]
status = "active"

## Benchmark
1 million rows processed:
- Rust csv-processor: 1.51s
- Python pandas: 2.77s
- Speedup: ~1.8x faster

## Status
Core pipeline complete. XML, JSONL converters planned.