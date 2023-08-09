use app::docker;
use color_eyre::Result;
use entity::product;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DbBackend;
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::Statement;

#[tokio::main]
async fn main() -> Result<()> {
    let db = docker::db_connect().await;

    let item = product::Entity::find_by_id(1).one(&db.mysql).await?;

    if item.is_none() {
        product::ActiveModel {
            id: Set(1),
            name: Set("test".to_string()),
        }
        .insert(&db.mysql)
        .await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    let _item = product::Entity::find_by_id(1)
        .one(&db.readyset)
        .await?
        .ok_or_else(|| color_eyre::eyre::eyre!("not found in readyset"))?;

    let _item = product::Entity::find()
        .filter(product::Column::Id.is_in(vec![1, 2, 3]))
        .one(&db.readyset)
        .await?
        .ok_or_else(|| color_eyre::eyre::eyre!("not found in readyset"))?;

    let _item = product::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"SELECT * FROM product WHERE id = ?"#,
            [1.into()],
        ))
        .one(&db.readyset)
        .await?
        .ok_or_else(|| color_eyre::eyre::eyre!("not found in readyset"))?;

    let _item = product::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"SELECT * FROM product WHERE id in (1)"#,
            [],
        ))
        .one(&db.readyset)
        .await?
        .ok_or_else(|| color_eyre::eyre::eyre!("not found in readyset"))?;

    #[derive(Debug, FromQueryResult)]
    pub struct ProductName {
        pub name: String,
    }

    let _foo = ProductName::find_by_statement(Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT product.name as name from product left join has_decimal on product.id = has_decimal.id where product.id = 1"#,
                [],
            ))
            .one(&db.readyset)
            .await?
            .ok_or_else(|| color_eyre::eyre::eyre!("not found in readyset"))?;

    Ok(())
}
