use rusqlite::{params, Connection, Result};
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;

pub fn load(dataset: &str, db_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(dataset)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    let conn = Connection::open(db_name)?;
    conn.execute("DROP TABLE IF EXISTS biopics", [])?;
    conn.execute(
        "CREATE TABLE biopics (
            title TEXT, 
            site TEXT, 
            country TEXT, 
            year_release INTEGER, 
            box_office TEXT, 
            director TEXT,
            number_of_subjects INTEGER,
            subject TEXT,
            type_of_subject TEXT,
            race_known TEXT,
            subject_race TEXT,
            person_of_color TEXT,
            subject_sex TEXT,
            lead_actor_actress TEXT
        )", 
        [],
    )?;
    
    for result in rdr.records() {
        let record = result?;
        conn.execute(
            "INSERT INTO biopics (title, site, country, year_release, box_office, director, number_of_subjects, subject, type_of_subject, race_known, subject_race, person_of_color, subject_sex, lead_actor_actress) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                record.get(0),
                record.get(1),
                record.get(2),
                record.get(3),
                record.get(4),
                record.get(5),
                record.get(6),
                record.get(7),
                record.get(8),
                record.get(9),
                record.get(10),
                record.get(11),
                record.get(12),
                record.get(13),
            ],
        )?;
    }

    Ok(())
}