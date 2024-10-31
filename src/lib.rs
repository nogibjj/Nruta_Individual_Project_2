use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn extract(url: &str, file_path: &str, directory: &str) {
    if fs::metadata(directory).is_err() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }

    // Check if the local CSV file already exists
    if fs::metadata(file_path).is_ok() {
        eprintln!("Warning: File already exists at: {}", file_path);
        return; // Or handle it as needed, e.g., prompt to overwrite
    }

    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");

    // Ensuring we write to the file only if the response is a success
    if response.status().is_success() {
        let mut file = fs::File::create(file_path).expect("Failed to create file");
        std::io::copy(&mut response, &mut file).expect("Failed to copy content");
        println!("Extraction successful!");
    } else {
        eprintln!(
            "Failed to fetch CSV, received status: {}",
            response.status()
        );
    }
}

pub fn transform_load(dataset: &str) -> Result<String, rusqlite::Error> {
    if fs::metadata(dataset).is_err() {
        eprintln!("Error: Dataset file does not exist: {}", dataset);
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let conn = Connection::open("biopics.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Biopics (
            title TEXT,
            country TEXT,
            year_release INTEGER,
            box_office TEXT,
            director TEXT,
            number_of_subjects INTEGER,
            subject TEXT,
            type_of_subject TEXT,
            subject_race TEXT,
            subject_sex TEXT,
            lead_actor_actress TEXT
        )",
        [],
    )?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(dataset)
        .map_err(|e| {
            eprintln!("Failed to create CSV reader: {:?}", e);
            rusqlite::Error::QueryReturnedNoRows
        })?;

    let mut stmt = conn.prepare(
        "INSERT INTO Biopics (
            title,
            country,
            year_release,
            box_office,
            director,
            number_of_subjects,
            subject,
            type_of_subject,
            subject_race,
            subject_sex,
            lead_actor_actress
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                if record.len() < 11 {
                    eprintln!(
                        "Warning: Record has fewer than expected fields: {:?}",
                        record
                    );
                    continue;
                }
                stmt.execute([
                    &record[0],
                    &record[1],
                    &record[2],
                    &record[3],
                    &record[4],
                    &record[5],
                    &record[6],
                    &record[7],
                    &record[8],
                    &record[9],
                    &record[10],
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }
    conn.close();
    Ok("biopics.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("biopics.db")?;
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, String>(0)?,  // title
                row.get::<usize, String>(1)?,  // country
                row.get::<usize, i32>(2)?,     // year_release
                row.get::<usize, String>(3)?,  // box_office
                row.get::<usize, String>(4)?,  // director
                row.get::<usize, i32>(5)?,     // number_of_subjects
                row.get::<usize, String>(6)?,  // subject
                row.get::<usize, String>(7)?,  // type_of_subject
                row.get::<usize, String>(8)?,  // subject_race
                row.get::<usize, String>(9)?,  // subject_sex
                row.get::<usize, String>(10)?, // lead_actor_actress
            ))
        })?;

        for result in results {
            match result {
                Ok((
                    title,
                    country,
                    year_release,
                    box_office,
                    director,
                    number_of_subjects,
                    subject,
                    type_of_subject,
                    subject_race,
                    subject_sex,
                    lead_actor_actress,
                )) => {
                    println!(
                        "Result: title={}, country={}, year_release={}, box_office={}, director={}, number_of_subjects={}, subject={}, type_of_subject={}, subject_race={}, subject_sex={}, lead_actor_actress={}",
                        title, country, year_release, box_office, director, number_of_subjects, subject, type_of_subject, subject_race, subject_sex, lead_actor_actress
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    conn.close();
    Ok(())
}
