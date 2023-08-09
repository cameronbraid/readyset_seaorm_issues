use app::docker;
use color_eyre::Result;
use entity::has_decimal;
use sea_orm::EntityTrait;


#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    let item = has_decimal::Entity::find_by_id(1)
        .one(&db.readyset)
        .await?
        .ok_or_else(|| color_eyre::eyre::eyre!("not found in readyset"))?;
    println!("item: {:?}", item);
    Ok(())
}
