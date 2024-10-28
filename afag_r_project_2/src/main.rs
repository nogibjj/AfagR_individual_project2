// mod lib; // Import the library module

// This will be the CLI portion of the project where we accept
// user-defined arguments and call lib.rs logic to handle them

use afag_r_project_2::{create, delete, extract, load, read, update};
use clap::{Parser, Subcommand};
use rusqlite::Result;

// Define a struct to hold our CLI arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Enum for commands: Create, Query (Read/Update), Delete, Load
#[derive(Debug, Subcommand)]
enum Commands {
    /// Download and save a file from a URL
    #[command(alias = "e", short_flag = 'e')]
    Extract { url: String, file_path: String },

    /// Load data from a CSV file into the SQLite database
    #[command(alias = "l", short_flag = 'l')]
    Load { dataset: String },

    /// Create a sample record in the database
    #[command(alias = "c", short_flag = 'c')]
    Create,

    /// Query data in the database (Read)
    #[command(alias = "r", short_flag = 'r')]
    Read,

    /// Update a record in the database
    #[command(alias = "u", short_flag = 'u')]
    Update,

    /// Delete a record in the database
    #[command(alias = "d", short_flag = 'd')]
    Delete,
}

fn main() -> Result<()> {
    // Parse the CLI arguments
    let args = Cli::parse();
    // let url = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/district-urbanization-index-2022/urbanization-index-2022.csv";
    // let file_path = "data/urbanization.csv";

    // Establish a connection to the SQLite database
    // let conn = Connection::open("births_database.db")?;

    // Match the subcommand and execute the corresponding library function
    match args.command {
        Commands::Extract { url, file_path } => {
            println!("Downloading file from '{}' to '{}'", url, file_path);
            extract(&url, &file_path).expect("Failed to download file");
        }
        Commands::Load { dataset } => {
            println!("Loading data from '{}'", dataset);
            load(&dataset).expect("Failed to load data from CSV");
        }
        Commands::Create => {
            println!("Creating a sample data record");
            create().expect("Failed to create data");
        }
        Commands::Read => {
            println!("Reading data from the database");
            read().expect("Failed to read data from database");
        }
        Commands::Update => {
            println!("Updating a record in the database");
            update().expect("Failed to update data");
        }
        Commands::Delete => {
            println!("Deleting a specific record in the database");
            delete().expect("Failed to delete data");
        }
    }

    Ok(())
}

// fn main() -> Result<(), String> {
//     // URL for the CSV file to be downloaded
//     let url = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/district-urbanization-index-2022/urbanization-index-2022.csv"; // Replace with actual URL
//     let file_path = "data/urbanization.csv"; // Path to save the downloaded file

//     // Call extract function
//     match lib::extract(url, file_path) {
//         Ok(path) => println!("File downloaded successfully: {}", path),
//         Err(e) => {
//             eprintln!("Error downloading file: {}", e);
//             return Err(e);
//         }
//     }

//     // Call load function to load data into SQLite database
//     match lib::load(file_path) {
//         Ok(db_path) => println!("Data loaded into database: {}", db_path),
//         Err(e) => {
//             eprintln!("Error loading data: {}", e);
//             return Err(e);
//         }
//     }

//     // Call read_data to verify data in the database
//     match lib::read_data() {
//         Ok(_) => println!("Data read successfully from database."),
//         Err(e) => {
//             eprintln!("Error reading data: {}", e);
//             return Err(e);
//         }
//     }

//     // Call create_data to insert a sample record
//     match lib::create_data() {
//         Ok(_) => println!("Sample data created successfully."),
//         Err(e) => {
//             eprintln!("Error creating data: {}", e);
//             return Err(e);
//         }
//     }

//     // Call update_data to update a record
//     match lib::update_data() {
//         Ok(_) => println!("Data updated successfully."),
//         Err(e) => {
//             eprintln!("Error updating data: {}", e);
//             return Err(e);
//         }
//     }

//     // Call delete_data to delete a specific record
//     match lib::delete_data() {
//         Ok(_) => println!("Data deleted successfully."),
//         Err(e) => {
//             eprintln!("Error deleting data: {}", e);
//             return Err(e);
//         }
//     }

//     Ok(())
// }
