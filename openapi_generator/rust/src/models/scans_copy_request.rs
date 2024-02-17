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
pub struct ScansCopyRequest {
    /// The ID of the destination folder. If you don't specify a folder ID, Tenable.io creates the copy in the same folder as the original.
    #[serde(rename = "folder_id", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i32>,
    /// The name of the copied scan. If you don't specify a name, Tenable.io uses the same name as the original with \"Copy of\" prefix.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ScansCopyRequest {
    pub fn new() -> ScansCopyRequest {
        ScansCopyRequest {
            folder_id: None,
            name: None,
        }
    }
}


