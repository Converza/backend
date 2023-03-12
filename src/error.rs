use std::io::Cursor;

use okapi::{
    openapi3::{MediaType, RefOr, Response, Responses},
    Map,
};
use rocket::{http::Status, response::Responder, Request};
use rocket_okapi::{gen::OpenApiGenerator, response::OpenApiResponderInner};
use schemars::_serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // Do not respond specific information about this error the user
    #[error("Error while hashing some password with Argon2")]
    Argon2(#[from] argon2::Error),

    #[error("Unable to find the {0}!")]
    NotFound(String),

    #[error("The {0} is already existing!")]
    AlreadyExisting(String),

    #[error("The entered password is too weak => {0}")]
    WeakPassword(String),

    #[error("Error while generating a JWT")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("The entered credentials are invalid!")]
    InvalidCredentials,

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    Server(String),
}

impl From<trustifier::error::Error> for Error {
    fn from(value: trustifier::error::Error) -> Self {
        match value {
            trustifier::error::Error::Argon2(error) => Self::Argon2(error),
            trustifier::error::Error::IllegalPassword(error) => Error::BadRequest(error),
            trustifier::error::Error::IllegalCredentials => Error::InvalidCredentials,
            trustifier::error::Error::MaxSessionsExceeded => Error::BadRequest(String::from("Max sessions exceeded")),
            trustifier::error::Error::Jwt(error) => Error::Jwt(error),
            trustifier::error::Error::NoConfigurationFound => Error::Server(String::from("No configuration found!")),
            trustifier::error::Error::UnauthorizedRequest => Error::Unauthorized(String::from("Unauthorized"))
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let mut response = rocket::Response::new();
        match &self {
            Self::WeakPassword(_) => response.set_status(Status::BadRequest),
            Self::AlreadyExisting(_) => response.set_status(Status::BadRequest),
            Self::NotFound(_) => response.set_status(Status::NotFound),
            Self::InvalidCredentials => response.set_status(Status::BadRequest),
            Self::Argon2(error) => {
                response.set_status(Status::InternalServerError);
                log::error!("Error while interacting with the API:");
                log::error!("   - {}", error);
            }
            Self::Jwt(error) => {
                response.set_status(Status::InternalServerError);
                log::error!("Error while interacting with the API:");
                log::error!("   - {}", error);
            },
            Self::Server(error) => {
                response.set_status(Status::InternalServerError);
                log::error!("Error while interacting with the API:");
                log::error!("   - {}", error);
            },
            Self::Unauthorized(_) => response.set_status(Status::Unauthorized),
            Self::BadRequest(_) => response.set_status(Status::BadRequest)
        }

        let content = match response.status().code {
            503 => {
                json!({
                    "status": "Unexpected error, please contact the administrator",
                    "code": 500
                })
                .to_string()
            }
            _ => {
                json!({
                    "status": format!("{}", self),
                    "code": response.status().code
                })
                .to_string()
            }
        };
        response.set_sized_body(content.len(), Cursor::new(content));

        Ok(response)
    }
}

impl OpenApiResponderInner for Error {
    fn responses(_gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut map: Map<String, RefOr<Response>> = Map::default();

        map.insert(
            String::from("404"),
            RefOr::Object(new_response(
                "The application is unable to find some element (Derived from the Error Handler)",
            )),
        );
        map.insert(
            String::from("500"),
            RefOr::Object(new_response(
                "The application is unable to dome some action and the handler is unable to recognise the specified error (Derived from the Error Handler)"
            ))
        );
        map.insert(
            String::from("409"),
            RefOr::Object(new_response(
                "The requester want to create some object and the application already finds unique fields of the value (Derived from the Error Handler)"
            ))
        );
        map.insert(
            String::from("400"),
            RefOr::Object(new_response(
                "The requester transfers a too weak password for the registration! (Derived from the Error Handler)"
            ))
        );

        Ok(Responses {
            default: None,
            responses: map,
            extensions: Default::default(),
        })
    }
}

fn new_response(description: &str) -> Response {
    let mut response = Response::default();

    response.content.insert(
        String::from("application/json"),
        MediaType {
            example: Some(json!({ "status": "Error literal", "code": 500 })),
            ..Default::default()
        },
    );
    response.description = description.to_string();
    response
}
