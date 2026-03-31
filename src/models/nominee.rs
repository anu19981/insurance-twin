use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::nominees;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = nominees)]
pub struct Nominee {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub full_name: String,
    pub relationship: String,
    pub date_of_birth: NaiveDate,
    pub share_percentage: BigDecimal,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = nominees)]
pub struct NewNominee {
    pub policy_id: Uuid,
    pub full_name: String,
    pub relationship: String,
    pub date_of_birth: NaiveDate,
    pub share_percentage: Option<BigDecimal>,
    pub phone: Option<String>,
    pub email: Option<String>,
}
