use app::docker;
use color_eyre::Result;
use entity::has_decimal;
use rust_decimal_macros::dec;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::Set;

#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    let item = has_decimal::Entity::find_by_id(1).one(&db.mysql).await?;

    println!("item: {:?}", item);

    Ok(())
}
