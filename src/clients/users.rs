use crate::clients::client;
use crate::models;
use models::User;
use reqwest::Error;

pub fn index(token: &str) -> Result<Vec<User>, Error> {
    let url = "https://dx-api.thingpark.com/core/v155/api/users";

    let users: Vec<User> = client::get::<Vec<User>>(url, token).unwrap();
    Ok(users)
}
