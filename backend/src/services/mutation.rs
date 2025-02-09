use sea_orm::{DbConn, EntityTrait};

use crate::entity::prelude::User;

pub struct Mutation;

// impl Mutation {
//     pub async fn create_user(db: &DbConn, ) -> String {
//         let
//         User::insert(model).exec(db).await
//     }
// }