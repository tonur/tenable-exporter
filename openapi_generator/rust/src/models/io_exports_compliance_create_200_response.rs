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
pub struct IoExportsComplianceCreate200Response {
    /// The UUID of the compliance export job.
    #[serde(rename = "export_uuid", skip_serializing_if = "Option::is_none")]
    pub export_uuid: Option<String>,
}

impl IoExportsComplianceCreate200Response {
    pub fn new() -> IoExportsComplianceCreate200Response {
        IoExportsComplianceCreate200Response {
            export_uuid: None,
        }
    }
}


