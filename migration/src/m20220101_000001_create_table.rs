use sea_orm_migration::{prelude::*, sea_orm::IdenStatic};

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

            manager
            .create_table(
                Table::create()
                    .table(ViewType::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ViewType::ChannelId).integer().not_null())
                    .col(ColumnDef::new(ViewType::ViewType).string().not_null())
                    .col(ColumnDef::new(ViewType::Enabled).boolean().not_null())
                    .primary_key(
                        Index::create()
                            .col(ViewType::ChannelId)
                            .col(ViewType::ViewType),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ViewTypeBit::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ViewTypeBit::ChannelId).integer().not_null())
                    .col(ColumnDef::new(ViewTypeBit::ViewType).string().not_null())
                    .col(ColumnDef::new(ViewTypeBit::Enabled).custom(ViewTypeBit::Bit1).not_null())
                    .primary_key(
                        Index::create()
                            .col(ViewType::ChannelId)
                            .col(ViewType::ViewType),
                    )
                    .to_owned(),
            )
            .await?;


        manager
        .create_table(
            Table::create()
                .table(Channel::Table)
                .if_not_exists()
                .col(ColumnDef::new(Channel::Id).integer().not_null().primary_key().auto_increment())
                .col(ColumnDef::new(Channel::Name).string().not_null())
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

#[derive(DeriveIden)]
enum ViewType {
    Table,
    ChannelId,
    ViewType,
    Enabled,
}

#[derive(DeriveIden)]
enum ViewTypeBit {
    Table,
    ChannelId,
    ViewType,
    Enabled,
    #[sea_orm(iden = "bit(1)")]
    Bit1,
}


#[derive(DeriveIden)]
enum Channel {
    Table,
    Id,
    Name,
}
