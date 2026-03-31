pub mod customers;
pub mod plans;
pub mod plan_benefits;
pub mod policies;
pub mod dependants;
pub mod nominees;
pub mod claims;
pub mod benefit_usages;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(customers::configure)
            .configure(plans::configure)
            .configure(plan_benefits::configure)
            .configure(policies::configure)
            .configure(dependants::configure)
            .configure(nominees::configure)
            .configure(claims::configure)
            .configure(benefit_usages::configure),
    );
}
