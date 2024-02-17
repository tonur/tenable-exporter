/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScansConfigureRequestCredentials : An object that specifies credential parameters that enable a scanner to authenticate a connection to a target host.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScansConfigureRequestCredentials {
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<Box<crate::models::ScansConfigureRequestCredentialsAdd>>,
    /// A scan-specific credentials object you want to modify. The parameters of the object vary based on credential category, credential type, and type-specific settings. For more information, see [Update a Scan](doc:update-scan-tio).   **Note:** This parameter is not supported for use with managed credentials. For more information about editing managed credentials, see [Edit Managed Credentials](doc:edit-managed-credentials-tio).
    #[serde(rename = "edit", skip_serializing_if = "Option::is_none")]
    pub edit: Option<serde_json::Value>,
    /// A list of identifiers for the credentials you want to remove from the scan. For more information, see [Remove Credentials from a Scan](doc:remove-credentials-from-scan-tio).
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<Vec<String>>,
}

impl ScansConfigureRequestCredentials {
    /// An object that specifies credential parameters that enable a scanner to authenticate a connection to a target host.
    pub fn new() -> ScansConfigureRequestCredentials {
        ScansConfigureRequestCredentials {
            add: None,
            edit: None,
            delete: None,
        }
    }
}


