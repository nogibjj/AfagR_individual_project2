use csv::Reader;
use rusqlite::{Connection, Result, MappedRows};


pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS game_sales (
             Rank INTEGER,
             Name TEXT,
             Platform TEXT,
             Publisher TEXT,
             Developer TEXT,
             Critic_Score REAL,
             User_Score REAL,
             Total_Shipped REAL,
             Year REAL
        )",
        [],
    )?;
    Ok(())
}

pub fn import_csv_to_sqlite(conn: &Connection) -> Result<()> {
    let mut reader = match Reader::from_path("src/game_sales_data.csv") {
        Ok(reader) => reader,
        Err(err) => {
            println!("Error reading file: {:?}", err);
            return Ok(());
        }
    };

    // Prepare an SQL statement outside the loop for better performance
    //let _stmt = conn.prepare("INSERT INTO game_sales (Rank, Name, Platform, Publisher, Developer, Critic_Score, User_Score, Total_Shipped, Year) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?;

    let mut batch_data = String::new();

    for result in reader.byte_records() {
        let record = match result {
            Ok(record) => record,
            Err(err) => {
                println!("Error reading record: {:?}", err);
                continue;
            }
        };
        
        let rank = String::from_utf8_lossy(&record[0]);
        let name = String::from_utf8_lossy(&record[1]);
        let platform = String::from_utf8_lossy(&record[2]);
        let publisher = String::from_utf8_lossy(&record[3]);
        let developer = String::from_utf8_lossy(&record[4]);
        let mut critic_score = String::from_utf8_lossy(&record[5]);
        let mut user_score = String::from_utf8_lossy(&record[6]);
        let total_shipped = String::from_utf8_lossy(&record[7]);
        let year = String::from_utf8_lossy(&record[8]);

        let name_escaped = name.replace('\'', "''");
        let platform_escaped = platform.replace('\'', "''");
        let publisher_escaped = publisher.replace('\'', "''");
        let developer_escaped = developer.replace('\'', "''");
        if user_score == ""{
            user_score = std::borrow::Cow::Borrowed("NULL");
        }
        if critic_score == ""{
            critic_score = std::borrow::Cow::Borrowed("NULL");
        }

        let curr_query = format!(
            "INSERT INTO game_sales (Rank, Name, Platform, Publisher, Developer, Critic_Score, User_Score, Total_Shipped, Year) VALUES ({}, '{}', '{}', '{}', '{}', {}, {}, {}, {});",
            rank, name_escaped, platform_escaped, publisher_escaped, developer_escaped, critic_score, user_score, total_shipped, year
        );
        batch_data.push_str(&curr_query);
        log::debug!("Current Query: {curr_query}");
        const BATCH_SIZE: usize = 9999999999999999999;
        if batch_data.len() >= BATCH_SIZE {
            log::debug!("Batch Data: {batch_data}");
            conn.execute_batch(&batch_data)?;
            log::debug!("Batch Data Executed");
            batch_data.clear();
        }
    }
    log::debug!("reached the end of the loop");
    //if !batch_data.is_empty() {
        //conn.execute_batch(&batch_data)?;}
    log::debug!("reached the end of the load csv");

    Ok(())
}

// make a function that queries the database and returns the result with a string as the argument

fn query_database<'a>(conn: &'a Connection, query: &str) -> Result<Vec<rusqlite::Row<'a>>> {
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([],|row| row)?;

    let result:Result<Vec<rusqlite::Row>> = rows.collect();

    Ok(result)
}
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
