//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "crates")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub count: Option<i32>,
    pub show: Option<i32>,
    pub locked: Option<i32>,
    pub autodj_source: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::crate_tracks::Entity")]
    CrateTracks,
}

impl Related<super::library::Entity> for Entity {
    fn to() -> RelationDef {
        super::crate_tracks::Relation::Library.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::crate_tracks::Relation::Crates.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
