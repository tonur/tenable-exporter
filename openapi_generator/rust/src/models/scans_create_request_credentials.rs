/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScansCreateRequestCredentials : An object that specifies credential parameters that enable a scanner to authenticate a connection to a target host.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScansCreateRequestCredentials {
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<Box<crate::models::ScansCreateRequestCredentialsAdd>>,
}

impl ScansCreateRequestCredentials {
    /// An object that specifies credential parameters that enable a scanner to authenticate a connection to a target host.
    pub fn new() -> ScansCreateRequestCredentials {
        ScansCreateRequestCredentials {
            add: None,
        }
    }
}

