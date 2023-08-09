#![allow(clippy::unwrap_used)]

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct Connections {
    pub mysql: DatabaseConnection,
    pub readyset: DatabaseConnection,
}
pub async fn db_connect() -> Connections {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    Connections {
        mysql: Database::connect({
            let mut opt = ConnectOptions::new("mysql://user:password@localhost:3306/db?ssl-mode=disabled".to_owned());
            opt.sqlx_logging_level(log::LevelFilter::Debug);
            opt
        })
        .await
        .expect("connection to mysql"),

        // readyset
        readyset: Database::connect({
            let mut opt = ConnectOptions::new("mysql://user:password@localhost:3307/db".to_owned());
            opt.sqlx_logging_level(log::LevelFilter::Debug);
            opt
        })
        .await
        .expect("connection to readyset"),
    }
}
