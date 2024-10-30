// src/library/query.rs
use rusqlite::{params, Connection, Result};

// Connect to the SQLite database
pub fn connect_db(db_name: &str) -> Result<Connection> {
    Connection::open(db_name)
}

// Create the biopics table if it doesn't already exist
pub fn create_table(db_name: &str) -> Result<()> {
    let conn = connect_db(db_name)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS biopics (
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
    Ok(())
}

// Insert a new biopic into the biopics table
pub fn create_biopic(
    db_name: &str,
    title: &str,
    site: &str,
    country: &str,
    year_release: i32,
    box_office: &str,
    director: &str,
    number_of_subjects: i32,
    subject: &str,
    type_of_subject: &str,
    race_known: &str,
    subject_race: &str,
    person_of_color: &str,
    subject_sex: &str,
    lead_actor_actress: &str,
) -> Result<()> {
    let conn = connect_db(db_name)?;
    conn.execute(
        "INSERT INTO biopics (title, site, country, year_release, box_office, director, number_of_subjects, subject, type_of_subject, race_known, subject_race, person_of_color, subject_sex, lead_actor_actress)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            title,
            site,
            country,
            year_release,
            box_office,
            director,
            number_of_subjects,
            subject,
            type_of_subject,
            race_known,
            subject_race,
            person_of_color,
            subject_sex,
            lead_actor_actress,
        ],
    )?;
    Ok(())
}

// Read all biopics from the biopics table
pub fn read_biopics(db_name: &str) -> Result<Vec<(String, String, String, i32, String, String, i32, String, String, String, String, String, String, String)>> {
    let conn = connect_db(db_name)?;
    let mut stmt = conn.prepare("SELECT * FROM biopics")?;
    let biopics_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
            row.get(10)?,
            row.get(11)?,
            row.get(12)?,
            row.get(13)?,
        ))
    })?;

    let mut results = Vec::new();
    for biopic in biopics_iter {
        results.push(biopic?);
    }
    Ok(results)
}

// Update an existing biopic in the biopics table
pub fn update_biopic(
    db_name: &str,
    title: &str,
    site: &str,
    country: &str,
    year_release: i32,
    box_office: &str,
    director: &str,
    number_of_subjects: i32,
    subject: &str,
    type_of_subject: &str,
    race_known: &str,
    subject_race: &str,
    person_of_color: &str,
    subject_sex: &str,
    lead_actor_actress: &str,
) -> Result<()> {
    let conn = connect_db(db_name)?;
    conn.execute(
        "UPDATE biopics SET site = ?2, country = ?3, year_release = ?4, box_office = ?5, director = ?6, number_of_subjects = ?7, subject = ?8, type_of_subject = ?9, race_known = ?10, subject_race = ?11, person_of_color = ?12, subject_sex = ?13, lead_actor_actress = ?14 WHERE title = ?1",
        params![
            title,
            site,
            country,
            year_release,
            box_office,
            director,
            number_of_subjects,
            subject,
            type_of_subject,
            race_known,
            subject_race,
            person_of_color,
            subject_sex,
            lead_actor_actress,
        ],
    )?;
    Ok(())
}

// Delete a biopic from the biopics table
pub fn delete_biopic(db_name: &str, title: &str) -> Result<()> {
    let conn = connect_db(db_name)?;
    conn.execute("DELETE FROM biopics WHERE title = ?1", params![title])?;
    Ok(())
}