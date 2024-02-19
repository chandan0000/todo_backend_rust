use axum::{
    extract::Request, http::StatusCode, middleware::Next, response::Response
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{api_error::APIError, jwt::decode_jwt};

use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};

pub async fn guard(mut req: Request, next: Next) -> Result<Response, APIError> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(APIError {
            message: "No Auth token found".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(40),
        })?
        .token()
        .to_owned();

    let claim = decode_jwt(token)
        .map_err(|err| APIError {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        })?
        .claims;

    let db = req
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(APIError {
            message: "Could not connect to database".to_owned(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    let identity = crate::entity::users::Entity::find()
        .filter(crate::entity::users::Column::Email.eq(claim.email.to_lowercase()))
        .one(db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        })?;

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}
