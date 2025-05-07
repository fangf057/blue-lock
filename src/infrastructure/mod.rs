use std::sync::Arc;

use sea_orm::DatabaseConnection;
use shaku::{Component, Interface};

use crate::errors::AppResult;

pub mod model;
pub mod sample_repo;

#[async_trait::async_trait]
pub trait IDbProvider: Interface {
    fn get_connection(&self) -> Arc<DatabaseConnection>;
}

#[derive(Component)]
#[shaku(interface = IDbProvider)]
pub struct DbProvider {
    conn: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl IDbProvider for DbProvider {
    fn get_connection(&self) -> Arc<DatabaseConnection> {
        Arc::clone(&self.conn)
    }
}
