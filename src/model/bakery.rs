//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bakery")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub image: String,
    pub details: String,
    pub in_stocks: i32,
    #[sea_orm(column_type = "Float")]
    pub price: f32,
    pub created_at: DateTime,
    pub restock_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::purchase_bakery::Entity")]
    PurchaseBakery,
}

impl Related<super::purchase_bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PurchaseBakery.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
