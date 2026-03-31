use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::benefit_usage::{BenefitUsage, NewBenefitUsage};
use crate::schema::benefit_usages;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/benefit-usages")
            .route("", web::post().to(create))
            .route("/by-policy/{policy_id}", web::get().to(list_by_policy))
            .route("/summary/{policy_id}/{benefit_id}", web::get().to(usage_summary)),
    );
}

async fn list_by_policy(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let policy_id = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = benefit_usages::table
        .filter(benefit_usages::policy_id.eq(policy_id))
        .order(benefit_usages::usage_date.desc())
        .load::<BenefitUsage>(&mut conn);

    match results {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn usage_summary(pool: web::Data<DbPool>, path: web::Path<(Uuid, Uuid)>) -> HttpResponse {
    let (policy_id, benefit_id) = path.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection");

    let usages = benefit_usages::table
        .filter(benefit_usages::policy_id.eq(policy_id))
        .filter(benefit_usages::benefit_id.eq(benefit_id))
        .load::<BenefitUsage>(&mut conn);

    match usages {
        Ok(items) => {
            let total_used: bigdecimal::BigDecimal = items.iter()
                .map(|u| &u.amount_used)
                .sum();
            let usage_count = items.len();

            HttpResponse::Ok().json(serde_json::json!({
                "policy_id": policy_id,
                "benefit_id": benefit_id,
                "usage_count": usage_count,
                "total_amount_used": total_used,
                "usages": items
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn create(pool: web::Data<DbPool>, body: web::Json<NewBenefitUsage>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = diesel::insert_into(benefit_usages::table)
        .values(&body.into_inner())
        .get_result::<BenefitUsage>(&mut conn);

    match result {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}
