/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoExportsComplianceStatus200Response : Information about the compliance export job, including the current status of the export and the total number of chunks.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoExportsComplianceStatus200Response {
    /// The status of the compliance export request. Possible values include:  - PROCESSING—Tenable Vulnerability Management has started processing the compliance export request.  - FINISHED—Tenable Vulnerability Management has completed processing the export request, the list of chunks is complete, and all chunks are available for download.  - READY—Some chunks are now available for download, but Tenable Vulnerability Management is still processing the export request.  - CANCELLED—An administrator has canceled the export request.  - ERROR—Tenable Vulnerability Management encountered an error while processing the export request. Tenable recommends that you retry the request. If the status persists on retry, contact Support.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// A comma-separated list of completed chunks available for download.
    #[serde(rename = "chunks_available", skip_serializing_if = "Option::is_none")]
    pub chunks_available: Option<Vec<i32>>,
}

impl IoExportsComplianceStatus200Response {
    /// Information about the compliance export job, including the current status of the export and the total number of chunks.
    pub fn new() -> IoExportsComplianceStatus200Response {
        IoExportsComplianceStatus200Response {
            status: None,
            chunks_available: None,
        }
    }
}


