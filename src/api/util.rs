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

pub const SERVER_NAME: &str = "Dmitry Muratov";