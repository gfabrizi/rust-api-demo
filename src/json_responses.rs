use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

use crate::models::User;

pub enum JsonResponses<'a> {
    JsonSuccessResponse(JsonSuccessResponse<'a>),
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
            JsonResponses::JsonFailResponse(resp) => {
                s = serializer.serialize_struct("JsonFailResponse", 2)?;
                s.serialize_field("status", resp.status)?;
                s.serialize_field("data", &resp.data)?;
            },
            JsonResponses::JsonErrorResponse(resp) => {
                s = serializer.serialize_struct("JsonErrorResponse", 2)?;
                s.serialize_field("status", resp.status)?;
                s.serialize_field("message", &resp.message)?;
            }
        };

        s.end()
    }
}

#[derive(Serialize)]
pub struct JsonSuccessResponse<'a> {
    status: &'a str,
    data: ResponsePayload<'a>,
}

#[derive(Serialize)]
pub struct JsonFailResponse<'a> {
    status: &'a str,
    data: ResponsePayload<'a>,
}

#[derive(Serialize)]
pub struct JsonErrorResponse<'a> {
    status: &'a str,
    message: ResponsePayload<'a>,
}

#[derive(Serialize)]
pub enum Payload {
    User(User),
    Users(Vec<User>),
}

pub enum ResponsePayload<'a> {
    Payload(Payload),
    JsonValue(JsonValue),
    Str(&'a str),
}

impl Serialize for ResponsePayload<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        return match self {
            ResponsePayload::Payload(p) => {
                let mut s = serializer.serialize_struct("Payload", 1)?;
                match p {
                    Payload::User(u) => s.serialize_field("user", u)?,
                    Payload::Users(u) => s.serialize_field("users", u)?,
                }
                s.end()
            },
            ResponsePayload::JsonValue(j) => {
                j.0.serialize(serializer)
            },
            ResponsePayload::Str(str) => {
                serializer.serialize_str(str)
            }
        };
    }
}

pub enum ResponseType {
    Success,
    Fail,
    Error,
}

pub fn json_response(payload: ResponsePayload, status_code: Status, response_type: ResponseType) -> status::Custom<Json<JsonResponses>> {
    let response = match response_type {
        ResponseType::Success => {
            JsonResponses::JsonSuccessResponse(
                JsonSuccessResponse {
                    status: "success",
                    data: payload,
                }
            )
        },
        ResponseType::Fail => {
            JsonResponses::JsonFailResponse(
                JsonFailResponse {
                    status: "fail",
                    data: payload,
                }
            )
        },
        ResponseType::Error => {
            JsonResponses::JsonErrorResponse(
                JsonErrorResponse {
                    status: "error",
                    message: payload,
                }
            )
        }
    };

    status::Custom(status_code, Json(response))
}