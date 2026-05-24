# CSV Processor


## What is this?

A CLI tool that accepts Excel, JSON, XML, or TSV, a config.toml file and outputs a clean validated CSV based on the config.toml file. it also generates an error report for any rows it could not process.


## How to run it?

```
cargo run -- --file-name test.xlsx --config transform.toml --output result.csv
```

## what is the status of the project?

currently it can only process excel,sql,pdf,json,xml files and validates the file type