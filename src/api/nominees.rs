use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::nominee::{NewNominee, Nominee};
use crate::schema::nominees;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/nominees")
            .route("", web::post().to(create))
            .route("/by-policy/{policy_id}", web::get().to(list_by_policy))
            .route("/{id}", web::get().to(get))
            .route("/{id}", web::delete().to(delete)),
    );
}

async fn list_by_policy(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let policy_id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = nominees::table
        .filter(nominees::policy_id.eq(policy_id))
        .load::<Nominee>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn get(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = nominees::table.find(id).first::<Nominee>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Nominee not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn create(pool: web::Data<DbPool>, body: web::Json<NewNominee>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::insert_into(nominees::table)
        .values(&body.into_inner())
        .get_result::<Nominee>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn delete(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::delete(nominees::table.find(id)).execute(&mut conn);

    match result {
        Ok(0) => HttpResponse::NotFound().json(serde_json::json!({"error": "Nominee not found"})),
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}
