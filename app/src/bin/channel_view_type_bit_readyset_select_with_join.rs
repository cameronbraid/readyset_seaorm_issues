use app::docker;
use color_eyre::Result;
use entity::channel;
use entity::view_type_bit;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    let items = channel::Entity::find()
        .left_join(view_type_bit::Entity)
        .filter(channel::Column::Id.eq(1))
        .filter(view_type_bit::Column::Enabled.eq(true))
        .all(&db.readyset)
        .await?;
    println!("items: {:?}", items);
    Ok(())
}
