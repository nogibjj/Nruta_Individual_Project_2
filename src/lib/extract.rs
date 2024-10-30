use reqwest::blocking::get;
use std::fs::File;
use std::io::{copy, Cursor};

pub fn extract(data: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Send a GET request
    let response = get(data)?;
    
    // Create a file to save the content
    let mut file = File::create(file_path)?;
    
    // Read the response body and write it to the file
    let response_body = response.bytes()?;
    let mut cursor =  Cursor::new(response_body); // Wrapping the bytes in a Cursor
    copy(&mut cursor, &mut file)?;

    Ok(file_path.to_string())
}