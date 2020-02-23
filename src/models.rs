use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub age: &'a i32,
    pub email: &'a str,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub age: i32,
    pub email: String,
}
