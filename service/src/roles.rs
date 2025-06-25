use ::entity::{roles, roles::Entity as Roles};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct RolesService;
impl RolesService {
    pub async fn create(
        db: &DbConn,
        name: String,
    ) -> Result<roles::ActiveModel, DbErr> {
        let now: DateTimeWithTimeZone = Utc::now().into();
        roles::ActiveModel {
            name: Set(name),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .save(db)
        .await
    }


    pub async fn update(
        db: &DbConn,
        id: i32,
        name: String,
    ) -> Result<roles::Model, DbErr> {
        let role: roles::ActiveModel = Roles::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find role.".to_owned()))
            .map(Into::into)?;
        roles::ActiveModel {
            id: role.id,
            name: Set(name),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete(id: i32, db: &DbConn) -> Result<DeleteResult, DbErr> {
        Roles::delete_by_id(id).exec(db).await
    }

    pub async fn find_by_id(id: i32, db: &DbConn) -> Result<Option<roles::Model>, DbErr> {
        Roles::find_by_id(id).one(db).await
    }
}