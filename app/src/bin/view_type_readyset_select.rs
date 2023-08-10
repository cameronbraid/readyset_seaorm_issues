use app::docker;
use color_eyre::Result;
use entity::view_type;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    let items = view_type::Entity::find()
        .filter(view_type::Column::ChannelId.eq(1))
        .all(&db.readyset)
        .await?;
    println!("items: {:?}", items);
    Ok(())
}
