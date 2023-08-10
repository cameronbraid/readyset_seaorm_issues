use app::docker;
use color_eyre::Result;
use entity::has_decimal;
use mysql::Pool;
use mysql::prelude::FromRow;
use rust_decimal::Decimal;

use mysql::prelude::Queryable;
#[tokio::main]
async fn main() -> Result<()> {
    
    #[derive(FromRow, Debug)]
    pub struct Row {
        pub id : i64,
        pub price: Decimal,
    }

    let url = "mysql://user:password@localhost:3307/db?enable_cleartext_plugin=true";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    let row: Option<Row> = conn
    .exec_first(
        "SELECT * from has_decimal",())?;

    println!("row: {:?}", row);

    Ok(())
}
