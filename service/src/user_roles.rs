use ::entity::{user_roles, user_roles::Entity as UserRoles};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct UserRolesService;
impl UserRolesService {
    pub async fn create(
        db: &DbConn,
        user_id: i32,
        role_id: i32,
    ) -> Result<user_roles::ActiveModel, DbErr> {
        let now: DateTimeWithTimeZone = Utc::now().into();
        user_roles::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            created_at: Set(now),
            updated_at: Set(now),
        }.save(db).await
    }

    pub async fn delete(
        user_id: i32,
        role_id: i32,
        db: &DbConn,
    ) -> Result<DeleteResult, DbErr> {
        UserRoles::delete_many()
            .filter(user_roles::Column::UserId.eq(user_id))
            .filter(user_roles::Column::RoleId.eq(role_id))
            .exec(db)
            .await
    }

    pub async fn find_by_user_id(
        user_id: i32,
        db: &DbConn,
    ) -> Result<Vec<user_roles::Model>, DbErr> {
        UserRoles::find()
            .filter(user_roles::Column::UserId.eq(user_id))
            .all(db)
            .await
    }
}