use biopics_cli::{extract, query, transform_load}; // Import the library module

fn main() {
    // Initialize file paths and database names
    let url = "https://github.com/nruta-choudhari/Datasets/raw/refs/heads/main/biopics.csv"; // Replace with the actual URL
    let file_path = "data/biopics.csv";
    let _db_name = "biopics.db";

    // Step 1: Extract CSV
    extract(url, file_path, "data");

    // Step 2: Transform and load data into the SQLite database
    match transform_load(file_path) {
        Ok(database_file) => {
            println!("Data loaded into database: {}", database_file);
        }
        Err(e) => {
            eprintln!("Error loading data into database: {}", e);
            return;
        }
    }

    // Step 3: Perform a sample query
    let number_of_subjects = 4; // Change this value as needed
    if let Err(e) = query(number_of_subjects) {
        eprintln!("Error executing query: {}", e);
    }
}