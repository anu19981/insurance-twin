use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::dependants;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = dependants)]
pub struct Dependant {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub full_name: String,
    pub relationship: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub aadhaar_number: Option<String>,
    pub is_primary_insured: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = dependants)]
pub struct NewDependant {
    pub policy_id: Uuid,
    pub full_name: String,
    pub relationship: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub aadhaar_number: Option<String>,
    pub is_primary_insured: Option<bool>,
}
