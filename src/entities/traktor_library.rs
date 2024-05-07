//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "traktor_library")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub tracknumber: Option<String>,
    #[sea_orm(unique)]
    pub location: Option<String>,
    pub comment: Option<String>,
    pub duration: Option<i32>,
    pub bitrate: Option<i32>,
    #[sea_orm(column_type = "Double", nullable)]
    pub bpm: Option<f64>,
    pub key: Option<String>,
    pub rating: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::traktor_playlist_tracks::Entity")]
    TraktorPlaylistTracks,
}

impl Related<super::traktor_playlist_tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TraktorPlaylistTracks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
