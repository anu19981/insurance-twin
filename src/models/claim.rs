use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::claims;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = claims)]
pub struct Claim {
    pub id: Uuid,
    pub claim_number: String,
    pub policy_id: Uuid,
    pub claimant_dependant_id: Option<Uuid>,
    pub benefit_id: Uuid,
    pub status: String,
    pub claim_type: String,
    pub claim_amount: BigDecimal,
    pub approved_amount: Option<BigDecimal>,
    pub diagnosis_summary: Option<String>,
    pub hospital_name: Option<String>,
    pub admission_date: Option<NaiveDate>,
    pub discharge_date: Option<NaiveDate>,
    pub rejection_reason: Option<String>,
    pub submitted_at: NaiveDateTime,
    pub reviewed_at: Option<NaiveDateTime>,
    pub settled_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = claims)]
pub struct NewClaim {
    pub claim_number: String,
    pub policy_id: Uuid,
    pub claimant_dependant_id: Option<Uuid>,
    pub benefit_id: Uuid,
    pub claim_type: String,
    pub claim_amount: BigDecimal,
    pub diagnosis_summary: Option<String>,
    pub hospital_name: Option<String>,
    pub admission_date: Option<NaiveDate>,
    pub discharge_date: Option<NaiveDate>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = claims)]
pub struct UpdateClaim {
    pub status: Option<String>,
    pub approved_amount: Option<BigDecimal>,
    pub rejection_reason: Option<String>,
    pub reviewed_at: Option<NaiveDateTime>,
    pub settled_at: Option<NaiveDateTime>,
}
