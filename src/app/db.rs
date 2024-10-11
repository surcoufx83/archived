use mysql::{OptsBuilder, Pool, PooledConn};
use std::error::Error;

use super::config::DatabaseConfig;

pub fn db_connect(db_config: DatabaseConfig) -> Result<PooledConn, Box<dyn Error>> {
    // Create the database URL
    let connection_parms = OptsBuilder::new()
        .ip_or_hostname(Some(db_config.host))
        .tcp_port(db_config.port)
        .user(Some(db_config.user))
        .pass(Some(db_config.password))
        .db_name(Some(db_config.database_name));

    // Establish a connection pool
    let pool = Pool::new(connection_parms).map_err(|e| Box::<dyn Error>::from(e))?;

    // Get a connection from the pool
    let connection = pool.get_conn().map_err(|e| Box::<dyn Error>::from(e))?;

    Ok(connection)
}
