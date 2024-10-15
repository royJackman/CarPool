use actix_web::{error, get, post, put, delete, web, Responder};
use diesel::{dsl::exists, prelude::*, r2d2};
use crate::models::{Reservation, NewReservation};
use crate::schema::reservations::dsl::*;
use crate::schema::cars::dsl as car_dsl;
use crate::schema::users::dsl as users_dsl;

type DbPool = r2d2::Pool<r2d2::ConnectionManager::<SqliteConnection>>;

#[post("/reservations")]
async fn add_reservation(pool: web::Data<DbPool>, form: web::Json<NewReservation>) -> actix_web::Result<impl Responder> {
    let failure_state = Reservation {
        id: -1,
        description: form.description.to_owned(),
        start_time: form.start_time.to_owned(),
        end_time: form.end_time.to_owned(),
        car_id: form.car_id.to_owned(),
        user_id: form.user_id.to_owned(),
    };
    if form.end_time <= form.start_time {
        return Ok(web::Json(failure_state));
    }
    let res = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        
        let car_exists = diesel::select(exists(car_dsl::cars.filter(car_dsl::id.eq(form.car_id))))
            .get_result::<bool>(&mut conn).unwrap();

        let user_exists = diesel::select(exists(users_dsl::users.filter(users_dsl::id.eq(form.user_id))))
            .get_result::<bool>(&mut conn).unwrap();
        
        return match (car_exists, user_exists) {
            (true, true) => Ok(diesel::insert_into(reservations)
                .values(form.to_owned())
                .returning(Reservation::as_returning())
                .get_result::<Reservation>(&mut conn)
                .expect("Error inserting reservation")),
            _ => Err(failure_state)
        };
    }).await?;

    Ok(web::Json(res.unwrap()))
}

#[get("/reservations/{reservation_id}")]
async fn get_reservation(pool: web::Data<DbPool>, reservation_id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let reservation = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        reservations.filter(id.eq(reservation_id.to_owned()))
            .first::<Reservation>(&mut conn)
            .expect(format!("Cannot find reservation with id {}", reservation_id).as_str())
    }).await?;

    Ok(web::Json(reservation))
}

#[get("/reservations")]
async fn get_reservations(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let results = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        reservations.load::<Reservation>(&mut conn).expect("Error loading reservations")
    }).await?;

    Ok(web::Json(results))
}

#[put("/reservations")]
async fn update_reservation(pool: web::Data<DbPool>, form: web::Json<Reservation>) -> actix_web::Result<impl Responder> {
    let updated_res = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        diesel::update(reservations.filter(id.eq(&form.id)))
            .set(form.to_owned())
            .execute(&mut conn)
            .expect(format!("Couldn't update reservation with id {}", &form.id).as_str())
    }).await?;

    Ok(web::Json(updated_res))
}

#[delete("/reservations/{res_id}")]
async fn delete_single_reservation(pool: web::Data<DbPool>, res_id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let completed = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        let res = diesel::delete(reservations)
            .filter(id.eq(res_id.to_owned()))  
            .execute(&mut conn);
        match res { Ok(n) if n == 1 => Ok(()), _ => Err(()) }
    }).await?;

    Ok(match completed { Ok(_) => web::Json(true), Err(_) => web::Json(false) })
}