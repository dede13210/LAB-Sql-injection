extern crate mysql;
extern crate actix_web;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}

async fn get_user_by_id(user_id: web::Path<String>) -> impl Responder {
    let users = fetch_users_by_condition(&user_id);
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
    }
}

// Fonction vulnérable à l'injection SQL
fn fetch_users_by_condition(condition: &str) -> mysql::Result<Vec<User>> {
    let url = "mysql://myuser:mypassword@mysql_container:3306/clientdb";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Construction de la requête SQL avec concaténation de chaînes
    let query = format!("SELECT id, first_name, last_name, email, phone FROM clients WHERE id = '{}'", condition);
    let result: Vec<User> = conn.query_map(
        &query,
        |(id, first_name, last_name, email, phone)| User {
            id,
            first_name,
            last_name,
            email,
            phone,
        },
    )?;

    Ok(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user/{id}", web::get().to(get_user_by_id))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
