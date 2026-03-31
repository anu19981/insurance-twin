use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::insurance_plan::{InsurancePlan, NewInsurancePlan, UpdateInsurancePlan};
use crate::schema::insurance_plans;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/plans")
            .route("", web::get().to(list))
            .route("", web::post().to(create))
            .route("/{id}", web::get().to(get))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete)),
    );
}

async fn list(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = insurance_plans::table
        .filter(insurance_plans::is_active.eq(true))
        .order(insurance_plans::created_at.desc())
        .load::<InsurancePlan>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn get(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = insurance_plans::table.find(id).first::<InsurancePlan>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Plan not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn create(pool: web::Data<DbPool>, body: web::Json<NewInsurancePlan>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::insert_into(insurance_plans::table)
        .values(&body.into_inner())
        .get_result::<InsurancePlan>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn update(pool: web::Data<DbPool>, path: web::Path<Uuid>, body: web::Json<UpdateInsurancePlan>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::update(insurance_plans::table.find(id))
        .set(&body.into_inner())
        .get_result::<InsurancePlan>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Plan not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn delete(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::delete(insurance_plans::table.find(id)).execute(&mut conn);

    match result {
        Ok(0) => HttpResponse::NotFound().json(serde_json::json!({"error": "Plan not found"})),
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}
