use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use okapi::{
    openapi3::{Parameter, ParameterValue},
    Map,
};
use rand::{distributions::Alphanumeric, Rng};
use rocket::{
    http::Status,
    outcome::Outcome::Success,
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};
use rocket::tokio::sync::broadcast::{channel, Sender};
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use schemars::JsonSchema;
use uuid::Uuid;

use crate::{
    error::Error,
    representation::config::{AuthConfig, GeneralConfig, HashConfig},
};
use crate::representation::Event;

#[derive(Clone, JsonSchema, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, JsonSchema, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, JsonSchema, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub user_id: String,
    pub id: String,
    pub exp: i64,
}

impl<'a, 's> OpenApiFromRequest<'a> for Session {
    fn from_request_input(
        _: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::Parameter(Parameter {
            name,
            location: "header".to_string(),
            description: Some(String::from("The current session of the requester")),
            required,
            deprecated: false,
            value: ParameterValue::Content {
                content: Map::new(),
            },
            extensions: Map::new(),
            allow_empty_value: false,
        }))
    }
}

#[rocket::async_trait]
impl<'r, 's> FromRequest<'r> for Session {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authentication");
        match token {
            Some(token) => {
                let config = request.rocket().state::<GeneralConfig>().unwrap();
                let token: TokenData<Session> = jsonwebtoken::decode(
                    token,
                    &DecodingKey::from_rsa_pem(config.auth.decode_public_key().as_bytes()).unwrap(),
                    &Validation::new(Algorithm::RS256),
                )
                .unwrap();

                // TODO: Check for existence of the session
                Success(token.claims)
            }
            None => {
                Outcome::Failure((Status::Unauthorized, Error::NotFound(String::from("Token"))))
            }
        }
    }
}

impl Session {
    pub fn to_jwt(&self, auth_config: &AuthConfig) -> Result<String, Error> {
        Ok(jsonwebtoken::encode(
            &Header::new(Algorithm::RS256),
            self,
            &EncodingKey::from_rsa_pem(auth_config.decode_private_key().as_bytes())?,
        )?)
    }
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub sender: Sender<Event>
}

impl User {
    pub fn new(
        email: String,
        username: String,
        password: String,
        hash_config: &HashConfig,
    ) -> Result<User, Error> {
        let salt = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();
        let hash = argon2::hash_encoded(
            password.as_bytes(),
            salt.as_bytes(),
            &hash_config.as_argon2_config(),
        )?;

        Ok(User {
            id: Uuid::new_v4().to_string(),
            email,
            username,
            password: hash,
            salt,
            sender: channel(1024).0
        })
    }

    #[inline]
    pub fn is_password_equal(&self, password: &String, config: &HashConfig) -> Result<bool, Error> {
        Ok(argon2::verify_encoded_ext(
            &self.password,
            password.as_bytes(),
            config.secret.as_bytes(),
            &[],
        )?)
    }
}
