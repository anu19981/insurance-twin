use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::dependant::{Dependant, NewDependant};
use crate::schema::dependants;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dependants")
            .route("", web::post().to(create))
            .route("/by-policy/{policy_id}", web::get().to(list_by_policy))
            .route("/{id}", web::get().to(get))
            .route("/{id}", web::delete().to(delete)),
    );
}

async fn list_by_policy(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let policy_id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = dependants::table
        .filter(dependants::policy_id.eq(policy_id))
        .load::<Dependant>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn get(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = dependants::table.find(id).first::<Dependant>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(serde_json::json!({"error": "Dependant not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn create(pool: web::Data<DbPool>, body: web::Json<NewDependant>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::insert_into(dependants::table)
        .values(&body.into_inner())
        .get_result::<Dependant>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn delete(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::delete(dependants::table.find(id)).execute(&mut conn);

    match result {
        Ok(0) => HttpResponse::NotFound().json(serde_json::json!({"error": "Dependant not found"})),
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}
