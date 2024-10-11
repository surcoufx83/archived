use app::db::db_connect;
use mysql::prelude::Queryable;
use mysql::*;
use std::error::Error;

// Import the config module
mod app;
use app::config::load_config;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Load the configuration
    let config = load_config().map_err(|e| Box::<dyn Error>::from(e))?;

    let mut db_connection = db_connect(config.database).map_err(|e| Box::<dyn Error>::from(e))?;

    let query = "SELECT * FROM files LIMIT 100";

    let results: Vec<Row> = db_connection.query(query)?;

    for row in results {
        println!("{:?}", row);
    }

    Ok(())
}
