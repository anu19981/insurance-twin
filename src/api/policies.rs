use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::policy::{NewPolicy, Policy, UpdatePolicy};
use crate::schema::policies;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/policies")
            .route("", web::get().to(list))
            .route("", web::post().to(create))
            .route("/{id}", web::get().to(get))
            .route("/{id}", web::put().to(update))
            .route("/by-customer/{customer_id}", web::get().to(list_by_customer)),
    );
}

async fn list(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = policies::table
        .order(policies::created_at.desc())
        .load::<Policy>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn get(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = policies::table.find(id).first::<Policy>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Policy not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn create(pool: web::Data<DbPool>, body: web::Json<NewPolicy>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::insert_into(policies::table)
        .values(&body.into_inner())
        .get_result::<Policy>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn update(pool: web::Data<DbPool>, path: web::Path<Uuid>, body: web::Json<UpdatePolicy>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::update(policies::table.find(id))
        .set(&body.into_inner())
        .get_result::<Policy>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Policy not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn list_by_customer(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let customer_id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = policies::table
        .filter(policies::customer_id.eq(customer_id))
        .order(policies::created_at.desc())
        .load::<Policy>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}
