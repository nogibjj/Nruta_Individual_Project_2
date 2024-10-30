mod extract;
mod query;
mod transform_load;

#[tokio::main]
async fn main() {
    let data_url = "https://path-to-your-csv/biopics.csv";
    let file_path = "data/biopics.csv";
    let db_name = "biopics.db";

    // Step 1: Extract CSV
    match extract::extract(data_url, file_path).await {
        Ok(_) => println!("Data extracted successfully to {}", file_path),
        Err(e) => eprintln!("Failed to extract data: {}", e),
    }

    // Step 2: Create Database Table
    match query::create_table(db_name) {
        Ok(_) => println!("Database table created successfully"),
        Err(e) => eprintln!("Failed to create database table: {}", e),
    }

    // Step 3: Load Data into Database
    match transform_load::load(file_path, db_name) {
        Ok(_) => println!("Data loaded into database successfully"),
        Err(e) => eprintln!("Failed to load data into database: {}", e),
    }
}