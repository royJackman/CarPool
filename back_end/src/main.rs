use actix_web::{web, App, HttpServer};
use diesel::{prelude::*, r2d2};

mod cars;
mod models;
mod reservations;
mod schema;
mod users;

type DbPool = r2d2::Pool<r2d2::ConnectionManager::<SqliteConnection>>;

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Database URL should be valid path to SQLite DB file")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let pool = initialize_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(cars::add_car)
            .service(cars::get_car)
            .service(cars::get_cars)
            .service(reservations::add_reservation)
            .service(reservations::get_reservation)
            .service(reservations::get_reservations)
            .service(users::add_user)
            .service(users::get_user)
            .service(users::get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
