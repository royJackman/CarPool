use actix_web::{error, get, post, web, HttpResponse, Responder};
use diesel::{prelude::*, r2d2};
use crate::models::{Reservation, NewReservation};

type DbPool = r2d2::Pool<r2d2::ConnectionManager::<SqliteConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_reservation(
    conn: &mut SqliteConnection,
    desc: Option<String>,
    start_time: i32,
    end_time: i32,
    car_id: i32,
    user_id: i32,
) -> Result<Reservation, DbError> {
    use crate::schema::reservations::dsl::*;

    let count = reservations
        .count()
        .get_result::<i64>(conn)
        .expect("Issue counting reservations");
    
    let new_reservation = Reservation {
        id: count as i32,
        description: desc,
        startTime: start_time,
        endTime: end_time,
        carId: car_id,
        userId: user_id
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
    assert!(form.startTime < form.endTime);

    let res = web::block(move || {
        let mut conn = pool.get()?;
        insert_new_reservation(
            &mut conn,
            form.description.clone(),
            form.startTime,
            form.endTime,
            form.carId,
            form.userId
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(res))
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

    Ok(HttpResponse::Ok().body(
        format!(
            "Reservation\nID: {}\nDescription: {}\nStart time: {}\nEnd time: {}\nCar ID: {}\nUser Id: {}",
            reservation.id,
            match reservation.description {
                Some(d) => d, None => "".to_owned()
            },
            reservation.startTime,
            reservation.endTime,
            reservation.carId,
            reservation.userId
        )
    ))
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

    Ok(HttpResponse::Ok().body(
        results.iter()
               .map(|res| format!("{} {} {}", res.id, res.startTime, res.endTime))
               .collect::<Vec<String>>()
               .join("\n")
    ))
}