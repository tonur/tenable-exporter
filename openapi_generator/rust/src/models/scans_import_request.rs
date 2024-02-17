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
pub struct ScansImportRequest {
    /// The name of the file to import as provided by the response from [Upload File](ref:file-upload) endpoint.
    #[serde(rename = "file")]
    pub file: String,
    /// The ID of the destination folder. If you omit this parameter, Tenable.io stores the imported scan in the default folder.
    #[serde(rename = "folder_id", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i32>,
    /// The password for the file to import (required for nessus.db).
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl ScansImportRequest {
    pub fn new(file: String) -> ScansImportRequest {
        ScansImportRequest {
            file,
            folder_id: None,
            password: None,
        }
    }
}


