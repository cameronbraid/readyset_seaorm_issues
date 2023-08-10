use app::docker;
use color_eyre::Result;
use entity::view_type;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;

#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    let item = view_type::ActiveModel {
        channel_id: Set(1),
        view_type: Set("list".to_string()),
        enabled: Set(true)
    }
    .insert(&db.mysql)
    .await?;

    println!("item: {:?}", item);
    Ok(())
}
