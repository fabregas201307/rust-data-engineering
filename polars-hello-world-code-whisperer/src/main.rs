/*
Polars hello world script that uses AWS Code Catalyst and Code Whisperer
*/
use polars::prelude::*;

pub fn calculate() -> Result<DataFrame, PolarsError> {
    // Read the CSV data using CsvReader
    let file = std::fs::File::open("data/iris.csv").unwrap();

    let df = CsvReader::new(file)
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;
    Ok(df)
}

fn main() {
    let df = calculate().unwrap();
    println!("{}", df);
}