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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HasDecimal::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum HasDecimal {
    Table,
    Id,
    Price,
}
