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

    if item.is_none() {
        has_decimal::ActiveModel {
            id: Set(1),
            price: Set(dec!(10.0)),
        }
        .insert(&db.mysql)
        .await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let item = has_decimal::Entity::find_by_id(1).one(&db.mysql).await?;
        println!("item: {:?}", item);
    } else {
        println!("item: {:?}", item);
    }

    Ok(())
}
