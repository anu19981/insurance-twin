use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::plan_benefits;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = plan_benefits)]
pub struct PlanBenefit {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub benefit_name: String,
    pub benefit_category: String,
    pub description: String,
    pub max_usage_per_year: Option<i32>,
    pub max_amount_per_usage: Option<BigDecimal>,
    pub max_amount_per_year: Option<BigDecimal>,
    pub is_cashless: bool,
    pub waiting_period_days: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = plan_benefits)]
pub struct NewPlanBenefit {
    pub plan_id: Uuid,
    pub benefit_name: String,
    pub benefit_category: String,
    pub description: String,
    pub max_usage_per_year: Option<i32>,
    pub max_amount_per_usage: Option<BigDecimal>,
    pub max_amount_per_year: Option<BigDecimal>,
    pub is_cashless: Option<bool>,
    pub waiting_period_days: Option<i32>,
}
