use biopics_cli::{extract, transform_load, query}; // Import the library module

fn main() {
    // Initialize file paths and database names
    let url = "https://example.com/biopics.csv"; // Replace with the actual URL
    let file_path = "data/biopics.csv";
    let _db_name = "biopics.db";

    // Step 1: Extract CSV
    extract(url, file_path, "data");
    
    // Step 2: Transform and load data into the SQLite database
    match transform_load(file_path) {
        Ok(database_file) => {
            println!("Data loaded into database: {}", database_file);
        },
        Err(e) => {
            eprintln!("Error loading data into database: {}", e);
            return;
        },
    }

    // Step 3: Perform a sample query
    let sample_query = "SELECT * FROM biopics"; // Example query, modify as needed
    if let Err(e) = query(sample_query) {
        eprintln!("Error executing query: {}", e);
    }
}