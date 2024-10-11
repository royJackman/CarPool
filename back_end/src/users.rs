use actix_web::{error, get, post, web, HttpResponse, Responder};
use diesel::{r2d2, prelude::*};
use crate::models::{User, NewUser};

type DbPool = r2d2::Pool<r2d2::ConnectionManager::<SqliteConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_user(
    conn: &mut SqliteConnection,
    nm: &str,
) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let count = users
        .count()
        .get_result::<i64>(conn)
        .expect("Issue counting users");

    let new_user = User {
        id: count as i32,
        name: nm.to_owned(),
    };

    let res = diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error inserting user.");

    Ok(res)
}

#[post("/users")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>
) -> actix_web::Result<impl Responder> {
    let usr = web::block(move || {
        let mut conn = pool.get()?;
        insert_new_user(&mut conn, &form.name)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(usr))
}

pub fn find_one_car(
    conn: &mut SqliteConnection,
    cid: &i32,
) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let result = users.filter(id.eq(cid))
        .first(conn)
        .expect(format!("Cannot find user with id {}", cid).as_str());

    Ok(result)
}

#[get("/users/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let usr = web::block(move || {
        let mut conn = pool.get()?;
        find_one_car(&mut conn, &user_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(
        format!("User\nID: {}\nName: {}", usr.id, usr.name)
    ))
}

pub fn find_all_users(
    conn: &mut SqliteConnection,
) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;
    let results = users.load(conn).expect("Error loading users");
    Ok(results)
}

#[get("/users")]
async fn get_users(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let results = web::block(move || {
        let mut conn = pool.get()?;
        find_all_users(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(
        results.iter()
               .map(|usr| format!("{} {}", usr.id, usr.name))
               .collect::<Vec<String>>()
               .join("\n")
    ))
}