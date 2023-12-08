#![feature(adt_const_params)]

use std::str::FromStr;

use anyhow::Result;
use bson::oid::ObjectId;

#[cfg(feature = "server")]
pub mod models;

#[cfg(feature = "server")]
pub async fn connect(uri: &str, name: &str) -> Result<mongodb::Database> {
    use mongodb::{Database, Client, options::{ConnectionString, ClientOptions}};

    use crate::models::User;

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

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy, core::marker::ConstParamTy)]
pub enum Role {
    Admin,
    Instructor,
    Assistant,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Self::Admin => "Admin",
            Self::Instructor => "Instructor",
            Self::Assistant => "Assistant",
        }.to_string()
    }
}

impl FromStr for Role {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "instructor" => Ok(Role::Instructor),
            "assistant" => Ok(Role::Assistant),
            _ => Err(anyhow::anyhow!("Invalid type from str for Role!")),
        }
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self {
            Self::Admin => if *other == Self::Admin {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater
            },
            Self::Assistant => if *other == Self::Assistant {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            },
            Self::Instructor => match *other {
                Self::Admin => std::cmp::Ordering::Less,
                Self::Assistant => std::cmp::Ordering::Greater,
                Self::Instructor => std::cmp::Ordering::Equal,
            }
        })
    }
}

pub type UserId = ObjectId;
