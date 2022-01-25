#[macro_use]
extern crate diesel;

use actix_web::{ web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::*;
use diesel::*;
use crate::schema::reviews::dsl::*;
use crate::schema::users::dsl::*;
use actix_web::HttpResponse;
use serde::{Serialize, Deserialize};
use actix_cors::Cors;


mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize,Deserialize)]
struct InputReview {
    id: i32,
    first_name: String,
    last_name: String,
    hospital_name: String,
    doctor_name: String,
    doctor_type: String,
    service_rendered: String,
    service_cost: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckUser {
    pub username: String,
    pub password: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = "";
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new().wrap(Cors::permissive())
            .data(pool.clone())
            .route("/review", web::post().to(add_review))
            .route("/reviews", web::get().to(get_reviews))
            .route("/signup",web::post().to(add_user))
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}

async fn add_review(
    db: web::Data<Pool>,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_review(db, item))
        .await
        .map(|review| HttpResponse::Created().json(review))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_review(
    db: web::Data<Pool>,
    item: web::Json<InputReview>,
) -> Result<Review, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_review = NewReview {
        id: &item.id,
        first_name: &item.first_name,
        last_name: &item.last_name,
        hospital_name: &item.hospital_name,
        doctor_name: &item.doctor_name,
        doctor_type: &item.doctor_type,
        service_rendered: &item.service_rendered,
        service_cost: &item.service_cost,
    };
    let res = insert_into(reviews).values(&new_review).get_result(&conn)?;
    Ok(res)
}
async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
async fn check_login(
    db: web::Data<Pool>,
    item: web::Json<CheckUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || check_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
fn check_user(
    db: web::Data<Pool>,
    item: web::Json<CheckUser>,
) -> Result<bool, diesel::result::Error> {
    let conn = db.get().unwrap();
    let us = sql_query("SELECT * FROM users WHERE username= {Ditching this backend cus Diesel is an ORM and ORMS suck}").load::<User>(&conn);
    Ok(true)
}
fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        username: &item.username,
        passwd: &item.password,
        email: &item.email,
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}
async fn get_reviews(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_reviews(db))
        .await
        .map(|review| HttpResponse::Ok().json(review))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
fn get_all_reviews(pool: web::Data<Pool>) -> Result<Vec<Review>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = reviews.load::<Review>(&conn)?;
    Ok(items)
}