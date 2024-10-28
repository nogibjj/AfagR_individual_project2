use csv::Reader; // For loading from CSV
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
// For loading CSV and handling errors

pub fn extract(url: &str, file_path: &str) -> Result<String, String> {
    // Create directories if they do not exist
    if let Some(parent) = Path::new(file_path).parent() {
        create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Make a GET request to the specified URL
    let response = get(url).map_err(|e| format!("Failed to get the URL: {}", e))?;

    // Check if response status is valid
    if response.status().is_success() {
        // Save the content to the specified file path
        let mut file =
            File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;
        let content = response
            .bytes()
            .map_err(|e| format!("Failed to read response body: {}", e))?;
        file.write_all(&content)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        println!("File successfully downloaded to {}", file_path);
        Ok(file_path.to_string())
    } else {
        Err(format!(
            "Failed to retrieve the file. Status Code: {}",
            response.status()
        ))
    }
}

pub fn load(dataset: &str) -> Result<String, String> {
    println!("Current working directory: {:?}", std::env::current_dir());

    // Open the CSV file
    let mut rdr =
        Reader::from_path(dataset).map_err(|e| format!("Failed to open CSV file: {}", e))?;

    // Connect to the SQLite database
    let conn = Connection::open("urbanization.db")
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Drop the table if it exists, then create a new one
    conn.execute("DROP TABLE IF EXISTS urbanization", [])
        .map_err(|e| format!("Failed to drop table: {}", e))?;
    conn.execute(
        "CREATE TABLE urbanization (stcd TEXT, state TEXT, cd REAL, pvi_22 REAL, urbanindex REAL,
        rural REAL, exurban REAL, suburban REAL, urban REAL,  grouping TEXT)",
        [],
    )
    .map_err(|e| format!("Failed to create table: {}", e))?;

    // Insert data into the database
    for result in rdr.records() {
        let record = result.map_err(|e| format!("Failed to read record: {}", e))?;
        conn.execute(
            "INSERT INTO urbanization (stcd, state, cd, pvi_22, urbanindex, rural, exurban, suburban, urban, grouping)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &record[0], &record[1], &record[2], &record[3], &record[4], &record[5],
                &record[6], &record[7], &record[8], &record[9]
            ],
        )
        .map_err(|e| format!("Failed to insert record: {}", e))?;
    }

    Ok("urbanization.db".to_string())
}

pub fn create() -> Result<String, String> {
    // Connect to the SQLite database
    let conn = Connection::open("urbanization.db")
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    conn.execute(
        "INSERT INTO urbanization (stcd, state, cd, pvi_22,urbanindex,
        rural,exurban,suburban,urban,grouping) VALUES (?, ?, ?, ?, ?,?,?,?,?,?)",
        params![
            "SS-00",
            "SS",
            10,
            10.1,
            1.5,
            10.1,
            100.1,
            99.9,
            100.01,
            "dense urban"
        ],
    )
    .map_err(|e| format!("Failed to insert record: {}", e))?;

    // Query the newly inserted record
    let mut stmt = conn
        .prepare("SELECT * FROM urbanization WHERE stcd = ?")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    let rows = stmt
        .query_map(params!["SS-00"], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, f64>(3)?,
                row.get::<_, f64>(4)?,
                row.get::<_, f64>(5)?,
                row.get::<_, f64>(6)?,
                row.get::<_, f64>(7)?,
                row.get::<_, f64>(8)?,
                row.get::<_, String>(9)?,
            ))
        })
        .map_err(|e| format!("Failed to query data: {}", e))?;

    for row in rows {
        println!(
            "create: {:?}",
            row.map_err(|e| format!("Failed to read data: {}", e))?
        );
    }

    Ok("Create Success".to_string())
}


pub fn read() -> Result<String, String> {
    // Connect to the SQLite database
    let conn = Connection::open("urbanization.db")
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Query data from the Drinks table
    let mut stmt = conn
        .prepare("SELECT * FROM urbanization")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    let _data_iter = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, f64>(3)?,
                row.get::<_, f64>(4)?,
                row.get::<_, f64>(5)?,
                row.get::<_, f64>(6)?,
                row.get::<_, f64>(7)?,
                row.get::<_, f64>(8)?,
                row.get::<_, String>(9)?,
            ))
        })
        .map_err(|e| format!("Failed to query data: {}", e))?;
    // Counter to limit to the first 5 rows
    let mut count = 0;
    for row in _data_iter {
        if count >= 5 {
            break;
        }
        let row_data = row.map_err(|e| format!("Failed to read data: {}", e))?;
        println!("Row {}: {:?}", count + 1, row_data);
        count += 1;
    }
   
    Ok("Read Success".to_string())
}


pub fn delete() -> Result<String, String> {
    // Connect to the SQLite database
    let conn = Connection::open("urbanization.db")
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Delete certain rows from the Drinks table
    let rows_deleted = conn
        .execute("DELETE FROM urbanization WHERE stcd = ?", params!["NY-09"])
        .map_err(|e| format!("Failed to delete record: {}", e))?;

    println!("rows deleted: {}", rows_deleted);
    Ok("Delete Success".to_string())
}

pub fn update() -> Result<String, String> {
    // Connect to the SQLite database
    let conn = Connection::open("urbanization.db")
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Update certain rows in the Drinks table
    let rows_updated = conn
        .execute(
            "UPDATE urbanization SET grouping = ? WHERE stcd = ?",
            params!["Urban-Suburban", "IL-07"],
        )
        .map_err(|e| format!("Failed to update record: {}", e))?;

    println!("rows updated: {}", rows_updated);
    Ok("Update Success".to_string())
}

// pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
//     // Drop the existing table first (if it exists)
//     let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
//     conn.execute(&drop_query, [])?;
//     println!("Table '{}' dropped successfully.", table_name);

//     // Create the new table with the correct schema
//     let create_query = format!(
//         "CREATE TABLE {} (
//             year STRING NOT NULL,
//             state STRING NOT NULL,
//             county STRING NOT NULL,
//             total_population INTEGER NOT NULL
//         )",
//         table_name
//     );
//     conn.execute(&create_query, [])?;
//     println!("Table '{}' created successfully.", table_name);
//     Ok(())
// }

// // Read
// pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
//     let mut stmt = conn.prepare(query_string)?;

//     let rows = stmt.query_map([], |row| {
//         let year: String = row.get(0)?;
//         let state: String = row.get(1)?;
//         let county: String = row.get(2)?;
//         let total_population: i32 = row.get(3)?;
//         Ok((year, state, county, total_population))
//     })?;

//     for row in rows {
//         let (year, state, county, total_population) = row?;
//         println!(
//             "Year: {}, state: {}, county: {},total_population: {}",
//             year, state, county, total_population
//         );
//     }

//     Ok(())
// }

// // Delete (drop table)
// pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
//     let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
//     conn.execute(&drop_query, [])?;
//     println!("Table '{}' dropped successfully.", table_name);
//     Ok(())
// }

// // Load data from a CSV file into the table
// pub fn load_data_from_csv(
//     conn: &Connection,
//     table_name: &str,
//     file_path: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let mut rdr = ReaderBuilder::new().from_reader(file);

//     let insert_query = format!(
//         "INSERT INTO {} (year, state, county, total_population) VALUES (?, ?, ?, ?)",
//         table_name
//     );

//     for result in rdr.records() {
//         let record = result?;
//         let year: String = record[0].parse()?;
//         let state: String = record[1].parse()?;
//         let county: String = record[2].parse()?;
//         let total_population: i32 = record[3].parse()?;

//         conn.execute(
//             &insert_query,
//             params![year, state, county, total_population],
//         )?;
//     }

//     println!(
//         "Data loaded successfully from '{}' into table '{}'.",
//         file_path, table_name
//     );
//     Ok(())
// }
