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
pub struct ScansTimezones200ResponseInner {
    /// The readable name of the timezone.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The system value for the timezone.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ScansTimezones200ResponseInner {
    pub fn new() -> ScansTimezones200ResponseInner {
        ScansTimezones200ResponseInner {
            name: None,
            value: None,
        }
    }
}

