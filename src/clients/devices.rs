use crate::clients::client;
use crate::models;
use models::Device;
use reqwest::Error;
use std::collections::HashMap;

pub fn index(token: &str) -> Result<Vec<Device>, Error> {
    let url = "https://dx-api.thingpark.com/core/v155/api/devices";

    let users: Vec<Device> = client::get::<Vec<Device>>(url, token).unwrap();
    Ok(users)
}

pub fn show(token: &str, id: i32) -> Result<Device, Error> {
    let url = format!("https://dx-api.thingpark.com/core/v155/api/devices/{}", id);

    let device: Device = client::get::<Device>(&url, token).unwrap();
    Ok(device)
}

pub fn create(
    token: &str,
    name: &str,
    dev_eui: &str,
    app_eui: &str,
    app_key: &str,
) -> Result<Device, models::ApiError> {
    let url = "https://dx-api.thingpark.com/core/v155/api/devices";

    let mut body = HashMap::new();
    body.insert("name", name.to_string());
    body.insert("EUI", dev_eui.to_string());
    body.insert("activationType", "OTAA".to_string());
    body.insert(
        "deviceProfileId",
        "LORA/SwisscomA.1.0.2a_ETSI_Rx2-SF12".to_string(),
    );
    body.insert("processingStrategyId", "DEFAULTRP".to_string());
    body.insert("applicationEUI", app_eui.to_string());
    body.insert("applicationKey", app_key.to_string());
    body.insert("motionIndicator", "WALKING_SPEED".to_string());

    let result = client::post::<Device>(url, token, body);
    match result {
        Ok(device) => Ok(device),
        Err(err) => Err(err),
    }
}
