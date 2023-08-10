use app::docker;
use color_eyre::Result;
use entity::channel;
use entity::view_type;
use entity::view_type_bit;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;

#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    let channel = channel::ActiveModel {
        id: Set(0),
        name: Set("test".to_string()),
    }
    .insert(&db.mysql)
    .await?;

    let _view_type = view_type_bit::ActiveModel {
        channel_id: Set(channel.id),
        view_type: Set("list".to_string()),
        enabled: Set(true),
    }
    .insert(&db.mysql)
    .await?;

    Ok(())
}
