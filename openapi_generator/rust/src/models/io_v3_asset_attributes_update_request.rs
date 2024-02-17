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
pub struct IoV3AssetAttributesUpdateRequest {
    /// The new or updated description for the custom asset attribute.   **Note:** Currently `description` is the only non-primary key attribute that can be updated. If the `name` field needs a new value then you should create a new custom asset attribute via the [POST /api/v3/assets/attributes](ref:io-v3-asset-attributes-create) endpoint.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl IoV3AssetAttributesUpdateRequest {
    pub fn new() -> IoV3AssetAttributesUpdateRequest {
        IoV3AssetAttributesUpdateRequest {
            description: None,
        }
    }
}


