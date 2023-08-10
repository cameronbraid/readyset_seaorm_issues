use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "channel")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::view_type::Entity")]
    ViewType,

    #[sea_orm(has_many = "super::view_type_bit::Entity")]
    ViewTypeBit,

}

impl Related<super::view_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ViewType.def()
    }
}

impl Related<super::view_type_bit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ViewTypeBit.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
