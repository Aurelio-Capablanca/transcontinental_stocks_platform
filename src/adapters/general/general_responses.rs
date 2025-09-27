use core::fmt;

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;
// use axum::http::status;
// use serde_json::error;
// use std::error::Error as StdError;

#[derive(Serialize, Debug)]
pub struct GeneralResponses<T>
where
    T: Serialize,
{
    pub message: Option<String>,
    pub dataset: Option<T>,
    pub code: Option<String>,
    pub error: Option<String>,
}

impl<T: serde::Serialize> IntoResponse for GeneralResponses<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub enum StopOperations {
    IO(std::io::Error),
    JSON(serde_json::Error),
    DB {
        mapped_nature_err: &'static str,
        severity: Option<String>,
        schema: Option<String>,
        column: Option<String>,
        constraint: Option<String>,
        datatype: Option<String>,
        line_error: Option<u32>,
        hint: Option<String>,
    },
    JWT(jsonwebtoken::errors::Error),
    InternalMessage(&'static str),
}

impl From<jsonwebtoken::errors::Error> for StopOperations {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        StopOperations::JWT(err)
    }
}

impl From<std::io::Error> for StopOperations {
    fn from(err: std::io::Error) -> Self {
        StopOperations::IO(err)
    }
}

impl From<tokio_postgres::error::DbError> for StopOperations {
    fn from(err: tokio_postgres::error::DbError) -> Self {
        StopOperations::DB {
            mapped_nature_err: "",
            severity: Some(err.severity().to_string()),
            schema: Some(err.schema().unwrap_or("").to_string()),
            column: Some(err.schema().unwrap_or("").to_string()),
            constraint: Some(err.constraint().unwrap_or("").to_string()),
            datatype: Some(err.datatype().unwrap_or("").to_string()),
            line_error: err.line(),
            hint: Some(err.hint().unwrap_or("").to_string()),
        }
    }
}

impl From<serde_json::Error> for StopOperations {
    fn from(err: serde_json::Error) -> Self {
        StopOperations::JSON(err)
    }
}

//impl StdError for StopOperations {}
impl fmt::Display for StopOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StopOperations::IO(e) => write!(f, "IO error: {}", e),
            StopOperations::DB {
                mapped_nature_err,
                severity,
                schema,
                column,
                constraint,
                datatype,
                line_error,
                hint,
            } => write!(
                f,
                "Database Error : {}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
                mapped_nature_err, severity, schema, column, constraint, datatype, line_error, hint
            ),
            StopOperations::JWT(e) => write!(f, "JWT error: {}", e),
            StopOperations::InternalMessage(e) => write!(f, "Custom Stop Operation error: {}", e),
            StopOperations::JSON(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl axum::response::IntoResponse for StopOperations {
    fn into_response(self) -> Response {
        let final_status = match &self {
            StopOperations::IO(_) | StopOperations::DB { .. } => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            StopOperations::JSON(_)
            | StopOperations::JWT(_)
            | StopOperations::InternalMessage(_) => axum::http::StatusCode::BAD_REQUEST,
        };
        let error_string = self.to_string();
        (
            final_status,
            axum::Json(serde_json::json!(GeneralResponses {
                message: Some("Platform Error".to_string()),
                dataset: Some("".to_string()),
                code: Some(final_status.as_str().to_string()),
                error: Some(error_string.to_string())
            })),
        )
            .into_response()
    }
}
