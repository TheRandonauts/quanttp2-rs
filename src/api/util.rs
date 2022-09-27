use std::{time::{Duration, SystemTime, UNIX_EPOCH}};

use serde::Deserialize;
use validator::Validate;


#[derive(Deserialize, Validate)]
pub struct ControlParams {
    #[validate(length( equal = 8, message = "Requires 8 digit serial"))]
    pub device_id: String,
    pub length: i32,
}


#[derive(Deserialize, Validate)]
pub struct ControlParamsDeviceId {
    #[validate(length( equal = 8, message = "Requires 8 digit serial"))]
    pub device_id: String,
}


pub fn get_current_time() -> Duration {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
}