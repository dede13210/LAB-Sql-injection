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

fn fetch_user_from_db(user_id: &str) -> Option<User> {
    let url = "mysql://myuser:mypassword@127.0.0.1:3306/clientdb";
    let pool = Pool::new(url).expect("Failed to create pool.");
    let mut conn = pool.get_conn().expect("Failed to get connection.");

    // Utilisation de requêtes préparées pour éviter l'injection SQL
    let result = conn.exec_first::<(u32, String, String, String, String), _, _>(
        "SELECT id, first_name, last_name, email, phone FROM clients WHERE id = :id",
        params! {
            "id" => user_id,
        },
    );

    match result {
        Ok(Some((id, first_name, last_name, email, phone))) => Some(User { id, first_name, last_name, email, phone }),
        Ok(None) => None,
        Err(_) => None,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user/{id}", web::get().to(get_user_by_id))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
