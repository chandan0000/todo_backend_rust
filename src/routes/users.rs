use axum::{extract::State, http::StatusCode, Json};
use chrono::DateTime;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::entity::users;
#[derive(Deserialize)]
pub struct RequestUser {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    email: String,
    id: i32,
    // token: String,
}

pub async fn create_user(
    State(databse): State<DatabaseConnection>,
    Json(user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let user = users::ActiveModel {
        email: Set(user.email),
        password: Set(user.password),
        ..Default::default()
    }
    .save(&databse)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let responseuser = ResponseUser {
        id: user.id.unwrap(),
        email: user.email.unwrap(),

    };

    Ok(Json(responseuser))
}
pub async fn login() {}
