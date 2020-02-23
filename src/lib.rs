#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{NewUser, User};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn do_create_user(conn: &SqliteConnection, user: &User) -> Result<usize, Error> {
    use schema::users;

    let new_user = NewUser {
        firstname: &user.firstname[..],
        lastname: &user.lastname[..],
        age: &user.age,
        email: &user.email[..],
    };

    let inserted = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(inserted)
}

pub fn do_update_user(conn: &SqliteConnection, user_to_update: &User, new_data: &User) -> Result<usize, Error> {
    use schema::users::dsl::*;

    let rows_affected = diesel::update(user_to_update)
        .set((
            firstname.eq(&new_data.firstname),
            lastname.eq(&new_data.lastname),
            age.eq(new_data.age),
            email.eq(&new_data.email)))
        .execute(conn)?;

    Ok(rows_affected)
}

pub fn do_delete_user(conn: &SqliteConnection, uid: i32) -> Result<usize, Error> {
    use schema::users::dsl::*;

    let rows_affected = diesel::delete(users.find(uid))
        .execute(conn)?;

    Ok(rows_affected)
}
