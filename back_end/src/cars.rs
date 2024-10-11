use actix_web::{error::{self, ErrorInternalServerError}, get, post, web, HttpResponse, Responder};
use diesel::{r2d2, prelude::*};
use crate::models::{Car, NewCar};

type DbPool = r2d2::Pool<r2d2::ConnectionManager::<SqliteConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_car(
    conn: &mut SqliteConnection,
    nm: &str,
) -> Result<Car, DbError> {
    use crate::schema::cars::dsl::*;

    let count = cars
        .count()
        .get_result::<i64>(conn)
        .expect("Issue counting cars");

    let new_car = Car {
        id: count as i32,
        name: nm.to_owned(),
    };

    let res = diesel::insert_into(cars)
        .values(&new_car)
        .returning(Car::as_returning())
        .get_result(conn)
        .expect("Error inserting car.");

    Ok(res)
}

#[post("/cars")]
async fn add_car(
    pool: web::Data<DbPool>,
    form: web::Json<NewCar>
) -> actix_web::Result<impl Responder> {
    let car = web::block(move || {
        let mut conn = pool.get()?;
        insert_new_car(&mut conn, &form.name)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(car))
}

pub fn find_one_car(
    conn: &mut SqliteConnection,
    cid: &i32,
) -> Result<Car, DbError> {
    use crate::schema::cars::dsl::*;
    let result = cars.filter(id.eq(cid))
        .first(conn)
        .expect(format!("Cannot find car with id {}", cid).as_str());

    Ok(result)
}

#[get("/cars/{car_id}")]
async fn get_car(
    pool: web::Data<DbPool>,
    car_id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let car = web::block(move || {
        let mut conn = pool.get()?;
        find_one_car(&mut conn, &car_id)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(
        format!("Car\nID: {}\nName: {}", car.id, car.name)
    ))
}

pub fn find_all_cars(
    conn: &mut SqliteConnection,
) -> Result<Vec<Car>, DbError> {
    use crate::schema::cars::dsl::*;
    let results = cars.load(conn).expect("Error loading cars");
    Ok(results)
}

#[get("/cars")]
async fn get_cars(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let results = web::block(move || {
        let mut conn = pool.get()?;
        find_all_cars(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(
        results.iter()
               .map(|car| format!("{} {}", car.id, car.name))
               .collect::<Vec<String>>()
               .join("\n")
    ))
}