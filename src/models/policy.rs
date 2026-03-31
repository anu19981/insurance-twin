use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::policies;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = policies)]
pub struct Policy {
    pub id: Uuid,
    pub policy_number: String,
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    pub status: String,
    pub premium_amount: BigDecimal,
    pub sum_insured: BigDecimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub payment_frequency: String,
    pub auto_renew: bool,
    pub purchased_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = policies)]
pub struct NewPolicy {
    pub policy_number: String,
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    pub premium_amount: BigDecimal,
    pub sum_insured: BigDecimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub payment_frequency: Option<String>,
    pub auto_renew: Option<bool>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = policies)]
pub struct UpdatePolicy {
    pub status: Option<String>,
    pub auto_renew: Option<bool>,
    pub payment_frequency: Option<String>,
}
