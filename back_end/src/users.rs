use actix_web::{error, get, post, put, delete, web, Responder};
use diesel::{r2d2, prelude::*};
use crate::models::{User, NewUser};
use crate::schema::users::dsl::*;

type DbPool = r2d2::Pool<r2d2::ConnectionManager::<SqliteConnection>>;

#[post("/users")]
async fn add_user(pool: web::Data<DbPool>, form: web::Json<NewUser>) -> actix_web::Result<impl Responder> {
    let usr = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        diesel::insert_into(users)
            .values(form.to_owned())
            .returning(User::as_returning())
            .get_result::<User>(&mut conn)
            .expect("Error inserting user")
    }).await?;

    Ok(web::Json(usr))
}

#[get("/users/{user_id}")]
async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let usr = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        users.filter(id.eq(user_id.to_owned()))
            .first::<User>(&mut conn)
            .expect(format!("Cannot find user with id {}", user_id).as_str())
    }).await?;

    Ok(web::Json(usr))
}

#[get("/users")]
async fn get_users(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let results = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        users.load::<User>(&mut conn).expect("Error loading users")
    }).await?;

    Ok(web::Json(results))
}

#[put("/users")]
async fn update_user(pool: web::Data<DbPool>, form: web::Json<User>) -> actix_web::Result<impl Responder> {
    let updated_user = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        diesel::update(users.filter(id.eq(&form.id)))
            .set(name.eq(form.name.to_owned()))
            .get_result::<User>(&mut conn)
            .expect(format!("Cannot update user with id {}", &form.id).as_str())
    }).await?;

    Ok(web::Json(updated_user))
}

#[delete("/users/{user_id}")]
async fn delete_single_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let completed = web::block(move || {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError).unwrap();
        let res = diesel::delete(users)
            .filter(id.eq(user_id.to_owned()))
            .execute(&mut conn);
        match res { Ok(n) if n == 1 => Ok(()), _ => Err(()) }
    }).await?;

    Ok(match completed { Ok(_) => web::Json(true), Err(_) => web::Json(false) })
}