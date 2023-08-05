use app::docker;
use color_eyre::Result;
use migration::Migrator;
use migration::MigratorTrait;

#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    Migrator::fresh(&db.mysql).await?;
    Ok(())
}
