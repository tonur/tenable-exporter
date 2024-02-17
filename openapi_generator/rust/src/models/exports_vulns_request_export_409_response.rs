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
pub struct ExportsVulnsRequestExport409Response {
    /// The UUID of ongoing vulnerabilities export job that was already submitted.
    #[serde(rename = "active_job_id", skip_serializing_if = "Option::is_none")]
    pub active_job_id: Option<String>,
    /// A description of the conflict.
    #[serde(rename = "failure_reason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

impl ExportsVulnsRequestExport409Response {
    pub fn new() -> ExportsVulnsRequestExport409Response {
        ExportsVulnsRequestExport409Response {
            active_job_id: None,
            failure_reason: None,
        }
    }
}


