use ::entity::{permissions, permissions::Entity as Permissions};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct PermissionsService;
impl PermissionsService {
    pub async fn create(
        db: &DbConn,
        resource: String,
        action: String,
    ) -> Result<permissions::ActiveModel, DbErr> {
        let now: DateTimeWithTimeZone = Utc::now().into();
        permissions::ActiveModel {
            resource: Set(resource),
            action: Set(action),
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
        resource: String,
        action: String,
    ) -> Result<permissions::Model, DbErr> {
        let permission: permissions::ActiveModel = Permissions::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find permission.".to_owned()))
            .map(Into::into)?;

        permissions::ActiveModel {
            id: permission.id,
            resource: Set(resource),
            action: Set(action),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete(id: i32, db: &DbConn) -> Result<DeleteResult, DbErr> {
        Permissions::delete_by_id(id).exec(db).await
    }

    pub async fn find_by_id(id: i32, db: &DbConn) -> Result<Option<permissions::Model>, DbErr> {
        Permissions::find_by_id(id).one(db).await
    }
}
