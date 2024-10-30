// src/library/extract.rs
use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub fn extract(data: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Send a GET request
    let response = get(data)?;
    
    // Create a file to save the content
    let mut file = File::create(Path::new(file_path))?;
    
    // Write the content to the file
    copy(&mut response.take(usize::MAX as u64), &mut file)?;

    println!("File downloaded to: {}", file_path);
    Ok(file_path.to_string())
}