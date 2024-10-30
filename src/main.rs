mod lib {
    pub mod extract;        
    pub mod query;         
    pub mod transform_load; 
}

#[tokio::main]
async fn main() {
    let data = "biopics.csv";
    let file_path = "data/biopics.csv";
    let db_name = "biopics.db";

    // Step 1: Extract CSV
    match lib::extract::extract(data, file_path) {
        Ok(_) => println!("Data extracted successfully to {}", file_path),
        Err(e) => eprintln!("Failed to extract data: {}", e),
    }

    // Step 2: Create Database Table
    match lib::query::create_table(db_name) {
        Ok(_) => println!("Database table created successfully"),
        Err(e) => eprintln!("Failed to create database table: {}", e),
    }

    // Step 3: Load Data into Database
    match lib::transform_load::load(file_path, db_name) {
        Ok(_) => println!("Data loaded into database successfully"),
        Err(e) => eprintln!("Failed to load data into database: {}", e),
    }

    // Step 4: Perform CRUD operations
    // Create a new biopic record
    match lib::query::create_biopic(
        db_name,
        "Back to Black",
        "https://www.imdb.com/title/tt21261712",
        "United Kingdom",
        2024,
        "$50.8",
        "Sam Taylor-Johnson",
        1,
        "Amy Winehouse",
        "Musician",
        "Yes",
        "White",
        "No",
        "Female",
        "Marisa Abela",
    ){
        Ok(_) => println!("Biopic created successfully"),
        Err(e) => eprintln!("Failed to create biopic: {}", e),
    }

    // Read and display all biopic records
    match lib::query::read_biopics(db_name) {
        Ok(biopics) => {
            println!("All Biopics:");
            for (title, site, country, year, box_office, director, subjects, subject_name, subject_type, race_known, subject_race, person_of_color, subject_sex, lead_actor) in biopics {
                println!("Title: {}, Site: {}, Country: {}, Year: {}, Box Office: {}, Director: {}, Subjects: {}, Subject Name: {}, Subject Type: {}, Race Known: {}, Subject Race: {}, Person of Color: {}, Subject Sex: {}, Lead Actor: {}",
                    title, site, country, year, box_office, director, subjects, subject_name, subject_type, race_known, subject_race, person_of_color, subject_sex, lead_actor);
            }
        },
        Err(e) => eprintln!("Failed to read biopics: {}", e),
    }

    // Updating the Back to Black biopic
    match lib::query::update_biopic(
        db_name,
        "Back to Black",
        "Updated Site",
        "Updated Country",
        2024,
        "Updated Box Office",
        "Updated Director",
        2,
        "Updated Subject",
        "Updated Type of Subject",
        "Updated Race known",
        "Updated Subject Race",
        "Updated Person of Color",
        "Updated Subject Sex",
        "Updated Lead Actor/Actress",
    ) {
        Ok(_) => println!("Biopic updated successfully"),
        Err(e) => eprintln!("Failed to update biopic: {}", e),
    }

    // Final read to verify changes
    match lib::query::read_biopics(db_name) {
        Ok(final_biopics) => {
            println!("Final Biopics after update:");
            for (title, site, country, year, box_office, director, subjects, subject_name, subject_type, race_known, subject_race, person_of_color, subject_sex, lead_actor) in final_biopics {
                println!("Title: {}, Site: {}, Country: {}, Year: {}, Box Office: {}, Director: {}, Subjecs: {}, Subject Name: {}, Subject Type: {}, Race Known: {}, Subject Race: {}, Person of Color: {}, Subject Sex: {}, Lead Actor: {}",
                    title, site, country, year, box_office, director, subjects, subject_name, subject_type, race_known, subject_race, person_of_color, subject_sex, lead_actor); 
            }
        },
        Err(e) => eprintln!("Failed to read final biopics: {}", e),
    }

    // Deleting Back to Black biopic
    match lib::query::delete_biopic(db_name, "Back to Black") {
        Ok(_) => println!("Biopic deleted successfully"),
        Err(e) => eprintln!("Failed to delete biopic: {}", e),
    }

    // Final read to verify deletion
    match lib::query::read_biopics(db_name) {
        Ok(final_biopics) => {
            println!("Final Biopics after deletion:");
            for (title, site, country, year, box_office, director, subjects, subject_name, subject_type, race_known, subject_race, person_of_color, subject_sex, lead_actor) in final_biopics {
                println!("Title: {}, Site: {}, Country: {}, Year: {}, Box Office: {}, Director: {}, Subjects: {}, Subject Name: {}, Subject Type: {}, Race Known: {}, Subject Race: {}, Person of Color: {}, Subject Sex: {}, Lead Actor: {}",
                    title, site, country, year, box_office, director, subjects, subject_name, subject_type, race_known, subject_race, person_of_color, subject_sex, lead_actor);
            }
        },
        Err(e) => eprintln!("Failed to read final biopics: {}", e)
    }
}
