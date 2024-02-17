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
pub struct ExportsVulnsRequestExportRequest {
    /// Specifies the number of assets used to chunk the vulnerabilities. The vulnerabilities export is split up by number of asset IDs in a chunk. The exported data of a chunk is the sum of all the vulnerabilities for each asset in that chunk. The range for number of assets in a chunk is a minimum of 50 (the default size) to a maximum of 5,000. If you specify a value outside this range, the system uses the upper or lower-bound value.
    #[serde(rename = "num_assets")]
    pub num_assets: i32,
    /// Specifies whether or not to include unlicensed assets. The default is `false` when no parameter is specified.
    #[serde(rename = "include_unlicensed", skip_serializing_if = "Option::is_none")]
    pub include_unlicensed: Option<bool>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Box<crate::models::ExportsVulnsRequestExportRequestFilters>>,
}

impl ExportsVulnsRequestExportRequest {
    pub fn new(num_assets: i32) -> ExportsVulnsRequestExportRequest {
        ExportsVulnsRequestExportRequest {
            num_assets,
            include_unlicensed: None,
            filters: None,
        }
    }
}

