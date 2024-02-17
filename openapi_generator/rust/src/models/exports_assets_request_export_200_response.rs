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
pub struct ExportsAssetsRequestExport200Response {
    /// The UUID of the assets export job.
    #[serde(rename = "export_uuid", skip_serializing_if = "Option::is_none")]
    pub export_uuid: Option<String>,
}

impl ExportsAssetsRequestExport200Response {
    pub fn new() -> ExportsAssetsRequestExport200Response {
        ExportsAssetsRequestExport200Response {
            export_uuid: None,
        }
    }
}


