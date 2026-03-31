use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::plan_benefit::{NewPlanBenefit, PlanBenefit};
use crate::schema::plan_benefits;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/plan-benefits")
            .route("", web::post().to(create))
            .route("/by-plan/{plan_id}", web::get().to(list_by_plan))
            .route("/{id}", web::get().to(get))
            .route("/{id}", web::delete().to(delete)),
    );
}

async fn list_by_plan(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let plan_id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = plan_benefits::table
        .filter(plan_benefits::plan_id.eq(plan_id))
        .load::<PlanBenefit>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn get(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = plan_benefits::table.find(id).first::<PlanBenefit>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Benefit not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn create(pool: web::Data<DbPool>, body: web::Json<NewPlanBenefit>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::insert_into(plan_benefits::table)
        .values(&body.into_inner())
        .get_result::<PlanBenefit>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn delete(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::delete(plan_benefits::table.find(id)).execute(&mut conn);

    match result {
        Ok(0) => HttpResponse::NotFound().json(serde_json::json!({"error": "Benefit not found"})),
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}
