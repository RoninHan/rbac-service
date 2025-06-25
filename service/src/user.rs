use ::entity::{users, users::Entity as User};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserModel {
    pub name: String,
    pub sex: String,
    pub email: String,
    pub app_id: String,
    pub phone: String,
    pub birthday: Option<DateTimeWithTimeZone>,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}

pub struct UserServices;

impl UserServices {
    pub async fn create_user(
        db: &DbConn,
        form_data: UserModel,
    ) -> Result<users::ActiveModel, DbErr> {
        let sex: i32 = form_data.sex.parse().expect("msg");
        users::ActiveModel {
            name: Set(form_data.name.to_owned()),
            sex: Set(sex),
            email: Set(Some(form_data.email)),
            password: Set(form_data.password),
            phone: Set(Some(form_data.phone.to_owned())),
            birthday: Set(form_data.birthday),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: i32,
        form_data: UserModel,
    ) -> Result<users::Model, DbErr> {
        let users: users::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find users.".to_owned()))
            .map(Into::into)?;
        let sex: i32 = form_data.sex.parse().expect("msg");
        users::ActiveModel {
            id: users.id,
            name: Set(form_data.name.to_owned()),
            email: Set(Some(form_data.email)),
            password: Set(form_data.password),
            sex: Set(sex),
            phone: Set(Some(form_data.phone)),
            birthday: Set(form_data.birthday),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let users: users::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find users.".to_owned()))
            .map(Into::into)?;

        users.delete(db).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }

    pub async fn find_user(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<users::Model>, u64), DbErr> {
        let paginator = User::find()
            .order_by_asc(users::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<users::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_user_by_email(
        db: &DbConn,
        email: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        User::find()
            .filter(users::Column::Email.contains(email))
            .one(db)
            .await
    }
}
