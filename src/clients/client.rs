use crate::models;
use reqwest::Error;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

pub fn post_form<T: for<'de> Deserialize<'de>>(
    url: &str,
    body: HashMap<&str, String>,
) -> Result<T, Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.post(url).form(&body).send();

    response.unwrap().json()
}

pub fn post<T: for<'de> Deserialize<'de>>(
    url: &str,
    token: &str,
    body: HashMap<&str, String>,
) -> Result<T, models::ApiError> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(&body)
        .send();

    let res = response.unwrap();

    match res.status() {
        StatusCode::CREATED => Ok(res.json::<T>().unwrap()),
        _ => Err(res.json::<models::ApiError>().unwrap()),
    }
}

pub fn get<T: for<'de> Deserialize<'de>>(url: &str, token: &str) -> Result<T, Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send();

    response.unwrap().json()
}
