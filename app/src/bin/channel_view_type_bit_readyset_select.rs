use app::docker;
use color_eyre::Result;
use entity::view_type_bit;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    let items = view_type_bit::Entity::find()
        .filter(view_type_bit::Column::ChannelId.eq(1))
        .all(&db.readyset)
        .await?;
    println!("items: {:?}", items);

    Ok(())
}
