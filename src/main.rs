use mysql::*;
use mysql::prelude::*;
use config::Config;
use toml::Table;
use std::fs;

// Struct to hold the database configuration
/* #[derive(Deserialize)]
struct DatabaseConfig {
    user: String,
    password: String,
    host: String,
    port: u16,
    database_name: String,
} */

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    let path = "config.toml";
    match fs::metadata(path) {
        Ok(_) => println!("File exists!"),
        Err(_) => println!("File not found!"),
    }

    /* let config = load_database_config()?;

    let cstring = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.database_name
    );
    let pool = Pool::new(cstring)?;
    let mut connection = pool.get_conn()?;

    let query = "SELECT * FROM files LIMIT 100";

    let results: Vec<Row> = connection.query(query)?;

    for row in results {
        println!("{:?}", row);
    }
 */
    Ok(())

}
