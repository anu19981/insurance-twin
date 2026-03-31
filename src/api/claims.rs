use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::claim::{Claim, NewClaim, UpdateClaim};
use crate::schema::claims;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/claims")
            .route("", web::get().to(list))
            .route("", web::post().to(create))
            .route("/{id}", web::get().to(get))
            .route("/{id}", web::put().to(update))
            .route("/by-policy/{policy_id}", web::get().to(list_by_policy)),
    );
}

async fn list(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = claims::table
        .order(claims::created_at.desc())
        .load::<Claim>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn get(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = claims::table.find(id).first::<Claim>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Claim not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn create(pool: web::Data<DbPool>, body: web::Json<NewClaim>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::insert_into(claims::table)
        .values(&body.into_inner())
        .get_result::<Claim>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn update(pool: web::Data<DbPool>, path: web::Path<Uuid>, body: web::Json<UpdateClaim>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::update(claims::table.find(id))
        .set(&body.into_inner())
        .get_result::<Claim>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Claim not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn list_by_policy(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let policy_id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = claims::table
        .filter(claims::policy_id.eq(policy_id))
        .order(claims::created_at.desc())
        .load::<Claim>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}
