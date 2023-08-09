use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HasDecimal::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HasDecimal::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(HasDecimal::Price).decimal().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().null())
                    .to_owned(),
            )
            .await?;
        manager
        .create_table(
            Table::create()
                .table(Order::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Order::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                // .col(ColumnDef::new(Product::Name).string().null())
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        Ok(())
    }
}

#[derive(DeriveIden)]
enum HasDecimal {
    Table,
    Id,
    Price,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
}
