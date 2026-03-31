use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::customers;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = customers)]
pub struct Customer {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub pan_number: Option<String>,
    pub aadhaar_number: Option<String>,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub pincode: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = customers)]
pub struct NewCustomer {
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub pan_number: Option<String>,
    pub aadhaar_number: Option<String>,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub pincode: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = customers)]
pub struct UpdateCustomer {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
}
