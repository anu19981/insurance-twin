use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::benefit_usages;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = benefit_usages)]
pub struct BenefitUsage {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub benefit_id: Uuid,
    pub claim_id: Option<Uuid>,
    pub usage_date: NaiveDate,
    pub amount_used: BigDecimal,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = benefit_usages)]
pub struct NewBenefitUsage {
    pub policy_id: Uuid,
    pub benefit_id: Uuid,
    pub claim_id: Option<Uuid>,
    pub usage_date: Option<NaiveDate>,
    pub amount_used: BigDecimal,
    pub notes: Option<String>,
}
