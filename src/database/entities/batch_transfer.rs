//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

use crate::database::enums::TransferStatus;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "batch_transfer"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub idx: i32,
    pub txid: Option<String>,
    pub status: TransferStatus,
    pub created_at: i64,
    pub updated_at: i64,
    pub expiration: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Idx,
    Txid,
    Status,
    CreatedAt,
    UpdatedAt,
    Expiration,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Idx,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AssetTransfer,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Idx => ColumnType::Integer.def(),
            Self::Txid => ColumnType::String(None).def().null(),
            Self::Status => ColumnType::SmallInteger.def(),
            Self::CreatedAt => ColumnType::BigInteger.def(),
            Self::UpdatedAt => ColumnType::BigInteger.def(),
            Self::Expiration => ColumnType::BigInteger.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AssetTransfer => Entity::has_many(super::asset_transfer::Entity).into(),
        }
    }
}

impl Related<super::asset_transfer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssetTransfer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
