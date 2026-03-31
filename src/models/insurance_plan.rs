use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::insurance_plans;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = insurance_plans)]
pub struct InsurancePlan {
    pub id: Uuid,
    pub name: String,
    pub plan_type: String,
    pub insurer_name: String,
    pub description: String,
    pub base_premium_annual: BigDecimal,
    pub sum_insured: BigDecimal,
    pub min_age: i32,
    pub max_age: i32,
    pub coverage_duration_months: i32,
    pub waiting_period_days: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = insurance_plans)]
pub struct NewInsurancePlan {
    pub name: String,
    pub plan_type: String,
    pub insurer_name: String,
    pub description: String,
    pub base_premium_annual: BigDecimal,
    pub sum_insured: BigDecimal,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
    pub coverage_duration_months: Option<i32>,
    pub waiting_period_days: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = insurance_plans)]
pub struct UpdateInsurancePlan {
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_premium_annual: Option<BigDecimal>,
    pub sum_insured: Option<BigDecimal>,
    pub is_active: Option<bool>,
}
