use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use crate::models::User;

pub enum JsonResponses<'a> {
    JsonSuccessResponse(JsonSuccessResponse<'a>),
    JsonSuccessNullDataResponse(JsonSuccessNullDataResponse<'a>),
    JsonFailResponse(JsonFailResponse<'a>),
    JsonErrorResponse(JsonErrorResponse<'a>),
}

impl Serialize for JsonResponses<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s;

        match self {
            JsonResponses::JsonSuccessResponse(resp) => {
                s = serializer.serialize_struct("JsonSuccessResponse", 2)?;
                s.serialize_field("status", resp.status)?;
                s.serialize_field("data", &resp.data)?;
            },
            JsonResponses::JsonSuccessNullDataResponse(resp) => {
                s = serializer.serialize_struct("JsonSuccessNullDataResponse", 2)?;
                s.serialize_field("status", resp.status)?;
                s.serialize_field("data", &resp.data)?;
            },
            JsonResponses::JsonFailResponse(resp) => {
                s = serializer.serialize_struct("JsonFailResponse", 2)?;
                s.serialize_field("status", resp.status)?;
                s.serialize_field("data", resp.data)?;
            },
            JsonResponses::JsonErrorResponse(resp) => {
                s = serializer.serialize_struct("JsonErrorResponse", 2)?;
                s.serialize_field("status", resp.status)?;
                s.serialize_field("message", resp.message)?;
            }
        };

        s.end()
    }
}

#[derive(Serialize)]
pub struct JsonSuccessResponse<'a> {
    status: &'a str,
    data: Payload,
}

#[derive(Serialize)]
pub struct JsonSuccessNullDataResponse<'a> {
    status: &'a str,
    data: JsonValue,
}

#[derive(Serialize)]
pub struct JsonFailResponse<'a> {
    status: &'a str,
    data: &'a str,
}

#[derive(Serialize)]
pub struct JsonErrorResponse<'a> {
    status: &'a str,
    message: &'a str,
}

#[derive(Serialize)]
pub enum Payload {
    User(User),
    Users(Vec<User>),
}

pub fn json_success_response<'a>(data: Payload, status_code: Status) -> status::Custom<Json<JsonResponses<'a>>> {
    let result = JsonResponses::JsonSuccessResponse(
        JsonSuccessResponse {
            status: "success",
            data
        }
    );

    status::Custom(status_code, Json(result))
}

pub fn json_success_response_null_data<'a>(status_code: Status) -> status::Custom<Json<JsonResponses<'a>>> {
    let result = JsonResponses::JsonSuccessNullDataResponse(
        JsonSuccessNullDataResponse {
            status: "success",
            data: json!(null)
        }
    );

    status::Custom(status_code, Json(result))
}

pub fn json_error_response(message: &str, status_code: Status) -> status::Custom<Json<JsonResponses>> {
    let result = JsonResponses::JsonErrorResponse(
        JsonErrorResponse {
            status: "error",
            message
        }
    );

    status::Custom(status_code, Json(result))
}

pub fn json_fail_response(data: &str, status_code: Status) -> status::Custom<Json<JsonResponses>> {
    let result = JsonResponses::JsonFailResponse(
        JsonFailResponse {
            status: "fail",
            data
        }
    );

    status::Custom(status_code, Json(result))
}