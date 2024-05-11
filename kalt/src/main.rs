use polars::prelude::*;
use snowflake::Client;

/*
define function which takes in polars dataframe and returns polars dataframe, 
columns are "id", "date", "value",  calculate the year over year change for each id
 */
fn calculate_yoy(df: DataFrame) -> DataFrame {
    // Group by id and date
    let gb = df.groupby(vec!["id", "date"]).unwrap();

    // Calculate the year over year change
    let yoy = gb.select("value").shift(1).unwrap();
    let yoy = gb.select("value").subtract(&yoy).unwrap();

    // Return the result
    yoy
}
    

fn main() {
    // Create a Snowflake client
    let client = Client::new("account", "username", "password");

    // Connect to Snowflake
    let connection = client.connect().unwrap();

    // Execute a query to fetch raw data
    let query = "SELECT * FROM your_table";
    let result = connection.execute(query).unwrap();

    // Convert the result into a Polars DataFrame
    let dataframe = DataFrame::from_arrow_data(&result).unwrap();

    // Perform operations on the DataFrame
    // ...

    // Print the DataFrame
    println!("{:?}", dataframe);
}
