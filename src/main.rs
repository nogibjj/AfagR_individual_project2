//make a main function that calls the add function in lib and prints the result
//import add from lib

use std::time::Instant;
use steam_cli::{create_table, import_csv_to_sqlite, query_db};

struct Profiler {
    start_time: Instant,
}

impl Profiler {
    fn new() -> Profiler {
        Profiler {
            start_time: Instant::now(),
        }
    }

    fn start(&mut self) {
        self.start_time = Instant::now();
    }

    fn stop(&self, name: &str) {
        let elapsed = self.start_time.elapsed();
        println!("{} took {:?}", name, elapsed);
    }
}
fn main() {
    env_logger::init();
    let mut profiler = Profiler::new();
    profiler.start();
    //run lib create table and import csv to sqlite
    let conn = rusqlite::Connection::open("game_sales.db").unwrap();
    create_table(&conn).unwrap();
    profiler.stop("create table");
    profiler.start();
    import_csv_to_sqlite(&conn).unwrap();
    profiler.stop("import csv to sqlite");
    profiler.start();
    let result = query_db(&conn, "SELECT * FROM game_sales LIMIT 5").unwrap();
    profiler.stop("query db");
    println!("{:?}", result);
}
