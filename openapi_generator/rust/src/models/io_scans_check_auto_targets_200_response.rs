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
pub struct IoScansCheckAutoTargets200Response {
    /// The list of targets that did not match a route in any scanner group.
    #[serde(rename = "missed_targets", skip_serializing_if = "Option::is_none")]
    pub missed_targets: Option<Vec<String>>,
    /// The total count of missed targets, before being truncated by the optional `limit` parameter.
    #[serde(rename = "total_missed_targets", skip_serializing_if = "Option::is_none")]
    pub total_missed_targets: Option<i32>,
    /// A list of UUIDs for scanner groups where configured scan routes matched at least one of the specified targets.
    #[serde(rename = "matched_resource_uuids", skip_serializing_if = "Option::is_none")]
    pub matched_resource_uuids: Option<Vec<String>>,
    /// The count of matched resource UUIDs, before being truncated by the optional `matched_resource_limit` parameter.
    #[serde(rename = "total_matched_resource_uuids", skip_serializing_if = "Option::is_none")]
    pub total_matched_resource_uuids: Option<i32>,
}

impl IoScansCheckAutoTargets200Response {
    pub fn new() -> IoScansCheckAutoTargets200Response {
        IoScansCheckAutoTargets200Response {
            missed_targets: None,
            total_missed_targets: None,
            matched_resource_uuids: None,
            total_matched_resource_uuids: None,
        }
    }
}


