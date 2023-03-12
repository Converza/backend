use argon2::{Config, ThreadMode, Variant, Version};
use base64::{engine, Engine};
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

impl AuthConfig {
    pub fn decode_private_key(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(
                engine::general_purpose::STANDARD
                    .decode(self.private_key.clone())
                    .unwrap(),
            )
        }
    }

    pub fn decode_public_key(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(
                engine::general_purpose::STANDARD
                    .decode(self.public_key.clone())
                    .unwrap(),
            )
        }
    }
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

impl HashConfig {
    #[allow(dead_code)]
    pub fn as_argon2_config(&self) -> Config {
        Config {
            ad: &[],
            hash_length: self.length as u32,
            lanes: self.lanes as u32,
            mem_cost: self.memory_cost,
            secret: self.secret.as_bytes(),
            thread_mode: ThreadMode::Parallel,
            time_cost: self.time_cost as u32,
            variant: Variant::Argon2id,
            version: Version::Version13,
        }
    }
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

impl PasswordConfig {
    pub fn check_password<'a>(&self, password: &str) -> Result<(), &'a str> {
        if self.min_length > password.len() as u8 {
            return Err("Password is too short!")
        }

        if self.max_length < password.len() as u8 {
            return Err("Password is too long!")
        }

        if self.lowercase && !LOWERCASE_REGEX.is_match(password) {
            return Err("Please use lowercase characters in your password!")
        }

        if self.uppercase && !UPPERCASE_REGEX.is_match(password) {
            return Err("Please use uppercase characters in your password!")
        }

        if self.numbers && !NUMBERS_REGEX.is_match(password) {
            return Err("Please use numbers in your password!")
        }

        if self.special && !SPECIAL_CHAR_REGEX.is_match(password.trim()) {
            return Err("Please use special characters in your password!")
        }

        Ok(())
    }
}

pub fn trustifier_config(config: &GeneralConfig) -> TrustifierConfig {
    TrustifierConfig {
        max_sessions: 10,
        session_lifetime: 30,
        password_config: trustifier::config::PasswordConfig {
            min_password_length: config.password.min_length,
            max_password_length: config.password.max_length,
            require_lowercase: false,
            require_uppercase: false,
            require_numbers: false,
            require_special: false,
            check_have_i_been_pwned: false,
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