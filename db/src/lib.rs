use anyhow::Result;
use mongodb::{Database, Client, options::{ConnectionString, ClientOptions}};

use crate::models::User;

pub mod models;

pub async fn connect(uri: &str, name: &str) -> Result<Database> {
    let config = ClientOptions::parse_connection_string(ConnectionString::parse(uri)?).await?;

    let client = Client::with_options(config)?;

    let database = client.database(name);

    // Not needed now, but maybe later
    User::create_indices(&database).await?;
    // Course::create_indices(&database).await?;
    // Assignment::create_indices(&database).await?;
    // Submission::create_indices(&database).await?;
    
    Ok(database)
}
