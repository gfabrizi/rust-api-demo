#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;

use diesel::prelude::*;

use rust_api_demo::{establish_connection, do_create_user, do_update_user, do_delete_user, models};
use rust_api_demo::schema::users::dsl::*;

pub mod json_responses;
use crate::json_responses::*;
use models::User;


// LIST ALL
#[get("/users", format = "application/json")]
fn list_users<'a>() -> status::Custom<Json<JsonResponses<'a>>> {
    let connection = establish_connection();

    match users.load::<User>(&connection) {
        Ok(u) => json_success_response(Payload::Users(u), Status::Ok),
        Err(_) => json_error_response("An error occurred while loading records", Status::InternalServerError)
    }
}

// CREATE
#[post("/users", format = "application/json", data = "<user>")]
fn create_user<'a>(user: Json<User>) -> status::Custom<Json<JsonResponses<'a>>> {
    let connection = establish_connection();

    let user = user.into_inner();

    match do_create_user(&connection, &user) {
        Ok(_) => json_success_response_null_data(Status::Created),
        Err(_) => json_error_response("An error occurred while creating the record", Status::InternalServerError)
    }
}

// RETRIEVE
#[get("/users/<uid>", format = "application/json")]
fn retrieve_user<'a>(uid: i32) -> status::Custom<Json<JsonResponses<'a>>> {
    let connection = establish_connection();

    match users.find(uid)
        .first::<User>(&connection) {
        Ok(u) => json_success_response(Payload::User(u), Status::Ok),
        Err(_) => json_fail_response("Record not found", Status::NotFound)
    }
}

// UPDATE
#[put("/users/<uid>", format = "application/json", data = "<user>")]
fn update_user<'a>(uid: i32, user: Json<User>) -> status::Custom<Json<JsonResponses<'a>>> {
    let connection = establish_connection();

    let new_data = user.into_inner();

    let user_to_update = match users.find(uid).first::<User>(&connection) {
        Ok(u) => u,
        Err(_) => return json_fail_response("Record not found", Status::NotFound)
    };

    match do_update_user(&connection, &user_to_update, &new_data) {
        Ok(_) => json_success_response_null_data(Status::Ok),
        Err(_) => json_error_response("An error occurred while updating the record", Status::InternalServerError)
    }
}

// DELETE
#[delete("/users/<uid>", format = "application/json")]
fn delete_user<'a>(uid: i32) -> status::Custom<Json<JsonResponses<'a>>> {
    let connection = establish_connection();

    if let Err(_) = users.find(uid)
        .first::<User>(&connection) {
        return json_fail_response("Record not found", Status::NotFound)
    };

    match do_delete_user(&connection, uid) {
        Ok(_) => json_success_response_null_data(Status::Ok),
        Err(_) => json_error_response("An error occurred while deleting the record", Status::InternalServerError)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![list_users, create_user, retrieve_user, update_user, delete_user])
        .launch();
}