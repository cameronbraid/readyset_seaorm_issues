use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "view_type_bit")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub channel_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub view_type: String,

    #[sea_orm(
        column_type = "custom(\"BIT(1)\")",
        nullable
    )]
    pub enabled: bool,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::channel::Entity",
        from = "Column::ChannelId",
        to = "super::channel::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Channel,

}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}
