#[derive(Debug)]
pub enum ApplicationError {
    Database(sqlx::Error),
    General(anyhow::Error),
}

impl actix_web::error::ResponseError for ApplicationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ApplicationError::Database(e) => {
                actix_web::HttpResponse::build(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("{:?}", e))
            }
            ApplicationError::General(g) => {
                actix_web::HttpResponse::build(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("{:?}", g))
            }
        }
    }
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::Database(e) => write!(f, "database error: {e}"),
            ApplicationError::General(e) => write!(f, "cannot parse template: {e}"),
        }
    }
}

impl From<sqlx::Error> for ApplicationError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}

impl From<anyhow::Error> for ApplicationError {
    fn from(value: anyhow::Error) -> Self {
        Self::General(value)
    }
}