use actix_web::{error, get, post, web, Responder};
use diesel::{prelude::*, r2d2};
use crate::models::{Reservation, NewReservation};

type DbPool = r2d2::Pool<r2d2::ConnectionManager::<SqliteConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_reservation(
    conn: &mut SqliteConnection,
    new_reservation: &NewReservation,
) -> Result<Reservation, DbError> {
    use crate::schema::reservations::dsl::*;

    let count = reservations
        .count()
        .get_result::<i64>(conn)
        .expect("Issue counting reservations");
    
    let new_reservation = Reservation {
        id: count as i32,
        description: new_reservation.description.to_owned(),
        start_time: new_reservation.start_time,
        end_time: new_reservation.end_time,
        car_id: new_reservation.car_id,
        user_id: new_reservation.user_id,
    };

    let res = diesel::insert_into(reservations)
        .values(&new_reservation)
        .returning(Reservation::as_returning())
        .get_result(conn)
        .expect("Error inserting reservation");

    Ok(res)
}

#[post("/reservations")]
async fn add_reservation(
    pool: web::Data<DbPool>,
    form: web::Json<NewReservation>
) -> actix_web::Result<impl Responder> {
    assert!(form.start_time < form.end_time);

    let res = web::block(move || {
        let mut conn = pool.get()?;
        insert_new_reservation(&mut conn, &form)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(web::Json(res))
}

pub fn find_one_reservation(
    conn: &mut SqliteConnection,
    rid: &i32,
) -> Result<Reservation, DbError> {
    use crate::schema::reservations::dsl::*;
    let result = reservations.filter(id.eq(rid))
        .first(conn)
        .expect(format!("Cannot find reservation with id {}", rid).as_str());

    Ok(result)
}

#[get("/reservations/{reservation_id}")]
async fn get_reservation(
    pool: web::Data<DbPool>,
    reservation_id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let reservation = web::block(move || {
        let mut conn = pool.get()?;
        find_one_reservation(&mut conn, &reservation_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(web::Json(reservation))
}

pub fn find_all_reservations(
    conn: &mut SqliteConnection,
) -> Result<Vec<Reservation>, DbError> {
    use crate::schema::reservations::dsl::*;
    let results = reservations.load(conn).expect("Error loading reservations");
    Ok(results)
}

#[get("/reservations")]
async fn get_reservations(
    pool: web::Data<DbPool>
) -> actix_web::Result<impl Responder> {
    let results = web::block(move || {
        let mut conn = pool.get()?;
        find_all_reservations(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(web::Json(results))
}