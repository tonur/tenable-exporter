/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScansScheduleRequest {
    /// Enables or disables the scan schedule.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl ScansScheduleRequest {
    pub fn new(enabled: bool) -> ScansScheduleRequest {
        ScansScheduleRequest {
            enabled,
        }
    }
}

