use biopics_cli::{extract, query, transform_load};

#[test]
fn test_extract() {
    let url = "https://github.com/nruta-choudhari/Datasets/raw/refs/heads/main/biopics.csv";
    let file_path = "data/biopics.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/biopics.csv";
    let result = transform_load(dataset);

    assert_eq!(result.unwrap(), "biopics.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM biopics WHERE year_release = 2009";
    let result = query(select_query);

    assert!(result.is_ok());
}
