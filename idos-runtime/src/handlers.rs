use crate::error::AppError;
use crate::AppState;
use actix_web::post;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ZkvmQuery {
    pub image_id: String,
}

#[post("/dataverse/run_zkvm")]
async fn run_zkvm(
    query: web::Query<ZkvmQuery>,
    data: web::Bytes,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let image_id = query.image_id.clone();
    let res = state
        .bonsai_client
        .run_bonsai(image_id, data.into())
        .await?;

    Ok(HttpResponse::Ok().json(json!({
        "session_id": res.0,
    })))
}
