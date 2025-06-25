use ::entity::{role_permissions, role_permissions::Entity as Role_Permissions};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct RolePermissionsService;
impl RolePermissionsService {
    pub async fn create(
        db: &DbConn,
        role_id: i32,
        permission_id: i32,
    ) -> Result<role_permissions::ActiveModel, DbErr> {
        let now: DateTimeWithTimeZone = Utc::now().into();
        role_permissions::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(permission_id),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .save(db)
        .await
    }

    pub async fn delete_by_role_id(db: &DbConn, role_id: i32) -> Result<DeleteResult, DbErr> {
        role_permissions::Entity::delete_many()
            .filter(role_permissions::Column::RoleId.eq(role_id))
            .exec(db)
            .await
        
    }
    pub async fn delete_by_permission_id(db: &DbConn, permission_id: i32) -> Result<DeleteResult, DbErr> {
        role_permissions::Entity::delete_many()
            .filter(role_permissions::Column::PermissionId.eq(permission_id))
            .exec(db)
            .await
    }

    pub async fn find_by_role_id(
        db: &DbConn,
        role_id: i32,
    ) -> Result<Vec<role_permissions::Model>, DbErr> {
        Role_Permissions::find()
            .filter(role_permissions::Column::RoleId.eq(role_id))
            .all(db)
            .await
    }

    pub async fn find_by_permission_id(
        db: &DbConn,
        permission_id: i32,
    ) -> Result<Vec<role_permissions::Model>, DbErr> {
        Role_Permissions::find()
            .filter(role_permissions::Column::PermissionId.eq(permission_id))
            .all(db)
            .await
    }
}
