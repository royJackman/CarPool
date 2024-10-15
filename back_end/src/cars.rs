use actix_web::{error, get, post, delete, put, web, Responder};
use diesel::{prelude::*, r2d2};
use crate::models::{Car, NewCar};
use crate::schema::cars::dsl::*;

type DbPool = r2d2::Pool<r2d2::ConnectionManager::<SqliteConnection>>;

#[post("/cars")]
async fn add_car(pool: web::Data<DbPool>, form: web::Json<NewCar>) -> actix_web::Result<impl Responder> {
    let car = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        diesel::insert_into(cars)
            .values(form.to_owned())
            .returning(Car::as_returning())
            .get_result::<Car>(&mut conn)
            .expect("Error inserting car")
    }).await?;

    Ok(web::Json(car))
}

#[get("/cars/{car_id}")]
async fn get_car(pool: web::Data<DbPool>, car_id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let car = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        cars.filter(id.eq(car_id.to_owned()))
            .first::<Car>(&mut conn)
            .expect(format!("Cannot find car with id {}", car_id).as_str())
    }).await?;

    Ok(web::Json(car))
}

#[get("/cars")]
async fn get_cars(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let results = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        cars.load::<Car>(&mut conn).expect("Error loading cars")
    }).await?;

    Ok(web::Json(results))
}

#[put("/cars")]
async fn update_car(pool: web::Data<DbPool>, form: web::Json<Car>) -> actix_web::Result<impl Responder> {
    let updated_car = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        diesel::update(cars.filter(id.eq(&form.id)))
            .set(name.eq(form.name.to_owned()))
            .get_result::<Car>(&mut conn)
            .expect(format!("Cannot update car with id {}", &form.id).as_str())
    }).await?;

    Ok(web::Json(updated_car))
}

#[delete("/cars/{car_id}")]
async fn delete_single_car(pool: web::Data<DbPool>, car_id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let completed = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        let res = diesel::delete(cars)
            .filter(id.eq(car_id.to_owned()))
            .execute(&mut conn);
        match res { Ok(n) if n == 1 => Ok(()), _ => Err(()) }
    }).await?;

    Ok(match completed { Ok(_) => web::Json(true), Err(_) => web::Json(false) })
}
