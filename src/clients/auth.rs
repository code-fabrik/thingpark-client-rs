use crate::clients::client;
use crate::models;
use models::Token;
use reqwest::Error;
use std::collections::HashMap;

pub fn create(target_profile_id: &str, username: &str, password: &str) -> Result<Token, Error> {
    let url = "https://dx-api.thingpark.com/admin/latest/api/oauth/token?validityPeriod=infinite";

    let client_id = format!("{}/{}", target_profile_id, username);

    let mut body = HashMap::new();
    body.insert("grant_type", "client_credentials".to_string());
    body.insert("client_id", client_id.to_string());
    body.insert("client_secret", password.to_string());

    let token: Token = client::post_form::<Token>(url, body).unwrap();

    Ok(token)
}
