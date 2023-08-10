use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "view_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub channel_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub view_type: String,

    pub enabled: bool,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
