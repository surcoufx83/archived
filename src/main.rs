use mysql::*;
use mysql::prelude::*;
use std::error::Error;

// Import the config module
mod app;
use app::config::{load_config};


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    // Load the configuration
    let config = load_config().map_err(|e| {
        eprintln!("Error loading configuration: {}", e);
        Box::<dyn Error>::from(e)
    })?;

    // Extract database configuration
    let db = config.database;

    // Create the database URL
    let connection_parms = OptsBuilder::new()
        .ip_or_hostname(Some(db.host))
        .tcp_port(db.port)
        .user(Some(db.user))
        .pass(Some(db.password))
        .db_name(Some(db.database_name));
    println!("{:?}", connection_parms);
    // Establish a connection pool
    let pool = Pool::new(connection_parms)?;

    let mut connection = pool.get_conn()?;

    let query = "SELECT * FROM files LIMIT 100";

    let results: Vec<Row> = connection.query(query)?;

    for row in results {
        println!("{:?}", row);
    }
 
    Ok(())

}
