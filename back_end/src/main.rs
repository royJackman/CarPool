use actix_web::{web, App, HttpServer};
use diesel::{prelude::*, r2d2};

mod cars;
mod models;
mod reservations;
mod schema;
mod users;

use cars::{add_car, get_car, get_cars, update_car, delete_single_car};
use reservations::{add_reservation, get_reservation, get_reservations, update_reservation, delete_single_reservation};
use users::{add_user, get_user, get_users, update_user, delete_single_user};

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
            .service(add_car).service(get_car).service(get_cars).service(update_car).service(delete_single_car)
            .service(add_reservation).service(get_reservation).service(get_reservations).service(update_reservation).service(delete_single_reservation)
            .service(add_user).service(get_user).service(get_users).service(update_user).service(delete_single_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
