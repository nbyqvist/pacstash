use actix_web::{web::Data, Responder};
use crate::error::ApplicationError;
use crate::state::AppState;
use crate::templates::NotFoundTemplate;

pub async fn not_found_page(_: Data<AppState>) -> Result<impl Responder, ApplicationError> {
    Ok(NotFoundTemplate {})
}