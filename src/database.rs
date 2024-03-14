pub fn insert_data_into_db(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let db_connection: rusqlite::Connection = rusqlite::Connection::open("grade_distributions.db")?;

    // Remove the extension from the csv_file and store it in table_name
    let table_name: String = std::path::Path::new(csv_file)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap()
        .replace("-", "_");

    // Create a new table per semester
    db_connection.execute(
        &format!(
            r#"CREATE TABLE {} (
                Semester TEXT,
                Section INTEGER,
                Department TEXT,
                Department_Code TEXT,
                Course_Number TEXT,
                Course_Title TEXT,
                Course_Full_Title TEXT,
                A INTEGER,
                A_Minus INTEGER,
                B_Plus INTEGER,
                B INTEGER,
                B_Minus INTEGER,
                C_Plus INTEGER,
                C INTEGER,
                C_Minus INTEGER,
                D_Plus INTEGER,
                D INTEGER,
                D_Minus INTEGER,
                F INTEGER,
                Other INTEGER
            )"#,
            table_name
        ),
        [],
    )?;

    // Read the CSV file and insert the data into the table
    let mut csv_reader: csv::Reader<std::fs::File> = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_path(csv_file)?;
    for result in csv_reader.records() {
        match result {
            Ok(record) => {
                db_connection.execute(
                    &format!(
                        r#"INSERT INTO {} (
                            Semester,
                            Section,
                            Department,
                            Department_Code,
                            Course_Number,
                            Course_Title,
                            Course_Full_Title,
                            A,
                            A_Minus,
                            B_Plus,
                            B,
                            B_Minus,
                            C_Plus,
                            C,
                            C_Minus,
                            D_Plus,
                            D,
                            D_Minus,
                            F,
                            Other
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)"#,
                        table_name
                    ),
                    &[
                        &record[0].to_string(),
                        &record[1].to_string(),
                        &record[2].to_string(),
                        &record[3].to_string(),
                        &record[4].to_string(),
                        &record[5].to_string(),
                        &record[6].to_string(),
                        &record[7].to_string(),
                        &record[8].to_string(),
                        &record[9].to_string(),
                        &record[10].to_string(),
                        &record[11].to_string(),
                        &record[12].to_string(),
                        &record[13].to_string(),
                        &record[14].to_string(),
                        &record[15].to_string(),
                        &record[16].to_string(),
                        &record[17].to_string(),
                        &record[18].to_string(),
                        &record[19].to_string(),
                    ],
                )?;
            }
            Err(err) => {
                eprintln!("Error reading record: {}", err);
            }
        }
    }

    Ok(())
}

pub fn insert_data_into_db_from_dir(input_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new sqlite3 database
    // If the database already exists, delete it
    if std::path::Path::new("grade_distributions.db").exists() {
        std::fs::remove_file("grade_distributions.db")?;
    }
    std::fs::File::create("grade_distributions.db")?;

    for entry in std::fs::read_dir(input_dir)? {
        let entry: std::fs::DirEntry = entry?;
        let path: std::path::PathBuf = entry.path();
        let path_str: &str = path.to_str().unwrap();

        // TODO: Use a more robust method to check if the file is a CSV file
        if path_str.ends_with(".csv") {
            println!("Inserting data into database from: {}", path_str);
            insert_data_into_db(path_str)?;
        }
    }

    Ok(())
}
