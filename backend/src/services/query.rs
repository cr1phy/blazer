use sea_orm::{DbConn, DbErr, EntityTrait};

use crate::entity::{prelude::User, user};

pub struct Query;

impl Query {
    pub async fn get_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }
}