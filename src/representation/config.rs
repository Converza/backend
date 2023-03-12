use lazy_static::lazy_static;
use regex::Regex;
use rocket::serde::Deserialize;
use trustifier::config::TrustifierConfig;

lazy_static! {
    static ref UPPERCASE_REGEX: Regex = Regex::new("[A-Z]").unwrap();
    static ref LOWERCASE_REGEX: Regex = Regex::new("[a-z]").unwrap();
    static ref NUMBERS_REGEX: Regex = Regex::new("[0-9]").unwrap();
    static ref SPECIAL_CHAR_REGEX: Regex = Regex::new("[^A-Za-z0-9]").unwrap();
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneralConfig {
    pub hashing: HashConfig,
    pub password: PasswordConfig,
    pub auth: AuthConfig,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthConfig {
    pub private_key: String,
    pub public_key: String,
    pub session_lifetime: u8,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HashConfig {
    pub memory_cost: u32,
    pub time_cost: u8,
    pub length: u8,
    pub lanes: u8,
    pub salt_length: u8,
    pub secret: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PasswordConfig {
    pub min_length: u8,
    pub max_length: u8,
    pub lowercase: bool,
    pub uppercase: bool,
    pub numbers: bool,
    pub special: bool,
    pub check_hibp: bool,
}

pub fn trustifier_config(config: &GeneralConfig) -> TrustifierConfig {
    TrustifierConfig {
        max_sessions: 10,
        session_lifetime: 30,
        password_config: trustifier::config::PasswordConfig {
            min_password_length: config.password.min_length,
            max_password_length: config.password.max_length,
            require_lowercase: config.password.lowercase,
            require_uppercase: config.password.uppercase,
            require_numbers: config.password.numbers,
            require_special: config.password.special,
            check_have_i_been_pwned: config.password.check_hibp,
            salt_length: config.hashing.salt_length as usize,
            hash_length: config.hashing.length as u32,
            hash_lanes: config.hashing.lanes as u32,
            hash_time_cost: config.hashing.time_cost as u32,
            hash_memory_cost: config.hashing.memory_cost,
            secret: config.hashing.secret.clone(),
            public_key: config.auth.public_key.clone(),
            private_key: config.auth.private_key.clone(),
        },
    }
}
