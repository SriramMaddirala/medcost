use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub hospital_name: String,
    pub doctor_name: String,
    pub doctor_type: String,
    pub service_rendered: String,
    pub service_cost: i64,
}
#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
    pub email: &'a str,
}

#[derive(Insertable)]
#[table_name = "reviews"]
pub struct NewReview<'a> {
    pub id: &'a i32,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub hospital_name: &'a str,
    pub doctor_name: &'a str,
    pub doctor_type: &'a str,
    pub service_rendered: &'a str,
    pub service_cost: &'a i64,
}
