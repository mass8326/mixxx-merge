//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "LibraryHashes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub directory_path: String,
    pub hash: Option<i32>,
    pub directory_deleted: Option<i32>,
    pub needs_verification: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}