use color_eyre::Result;
use rust_decimal::Decimal;
use sqlx::{MySqlPool, types::BigDecimal};

#[tokio::main]
async fn main() -> Result<()> {

    let pool = MySqlPool::connect("mysql://user:password@localhost:3307/db?ssl-mode=disabled").await?;

    #[derive(sqlx::FromRow, Debug)]
    pub struct Row {
        pub id : i64,
        pub price: Decimal,
    }

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row : Row = sqlx::query_as("SELECT id, price from has_decimal")
        .fetch_one(&pool).await?;

    println!("row: {:?}", row);

    Ok(())
}
