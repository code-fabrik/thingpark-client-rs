#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub r#ref: String,
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub organization: String,
    pub scopes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    pub r#ref: String,
    pub name: String,
    pub EUI: String,
    pub networkAddress: Option<String>,
    pub activationType: String,
    pub deviceClass: Option<String>,
    pub deviceProfileId: String,
    pub connectivityPlanId: String,
    pub processingStrategyId: String,
    pub applicationEUI: String,
    pub motionIndicator: String,
    pub statistics: Option<HashMap<String, String>>,
    pub commercialDetails: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    pub errorId: String,
}
