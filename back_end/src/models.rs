use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::cars)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Car {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewCar {
    pub name: String,
}


#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::reservations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Reservation {
    pub id: i32,
    pub description: Option<String>,
    pub start_time: i32,
    pub end_time: i32,
    pub user_id: i32,
    pub car_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewReservation {
    pub description: Option<String>,
    pub start_time: i32,
    pub end_time: i32,
    pub user_id: i32,
    pub car_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    pub name: String,
}