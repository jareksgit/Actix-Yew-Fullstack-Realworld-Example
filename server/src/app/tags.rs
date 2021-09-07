use actix_web::{web::Data, HttpResponse, ResponseError};

use super::AppState;
use crate::prelude::*;

// Client Messages ↓

#[derive(Debug)]
pub struct GetTags {}

// JSON response objects ↓

#[derive(Serialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

// Route handlers ↓

pub async fn get(state: Data<AppState>) -> Result<HttpResponse, Error> {
    match state.db.send(GetTags {}).await? {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}
