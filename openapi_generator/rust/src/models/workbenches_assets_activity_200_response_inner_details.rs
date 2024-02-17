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
pub struct WorkbenchesAssetsActivity200ResponseInnerDetails {
    /// The UUID of the asset.
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// The UUID of your Tenable.io instance.
    #[serde(rename = "containerId", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// The timestamp of the asset creation. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    /// The timestamp of the asset update time. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
    /// Specifies whether the asset has an agent installed.
    #[serde(rename = "hasAgent", skip_serializing_if = "Option::is_none")]
    pub has_agent: Option<bool>,
    /// Specifies whether or not any plugin results match this asset.
    #[serde(rename = "hasPluginResults", skip_serializing_if = "Option::is_none")]
    pub has_plugin_results: Option<bool>,
    /// The timestamp of the completion of the scan that discovered or observed the asset for the first time. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "firstScanTime", skip_serializing_if = "Option::is_none")]
    pub first_scan_time: Option<i32>,
    /// The timestamp of the completion of the last asset scan. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "lastScanTime", skip_serializing_if = "Option::is_none")]
    pub last_scan_time: Option<i32>,
    /// The timestamp of the completion of the last authenticated scan of the asset. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "lastAuthenticatedScanTime", skip_serializing_if = "Option::is_none")]
    pub last_authenticated_scan_time: Option<i32>,
    /// The timestamp of the scan completion time when asset was last scanned and matched license v1 requirements. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "lastLicensedScanTime", skip_serializing_if = "Option::is_none")]
    pub last_licensed_scan_time: Option<i32>,
    /// The timestamp of the scan completion time when asset was last scanned and matched license v2 requirements. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "lastLicensedScanTimeV2", skip_serializing_if = "Option::is_none")]
    pub last_licensed_scan_time_v2: Option<i32>,
    /// An array of source objects representing the entity that logged the event.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<crate::models::WorkbenchesAssetsActivity200ResponseInnerDetailsSourcesInner>>,
    /// If terminated, the timestamp of asset termination. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "terminatedAt", skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<i32>,
    /// The UUID of the user that terminated the asset.
    #[serde(rename = "terminatedBy", skip_serializing_if = "Option::is_none")]
    pub terminated_by: Option<String>,
    /// If deleted, the timestamp of asset deletion. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<i32>,
    /// The UUID of the user that deleted the asset.
    #[serde(rename = "deletedBy", skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    /// Additional asset attributes. For attribute definitions, see [Common Asset Attributes](doc:common-asset-attributes).
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl WorkbenchesAssetsActivity200ResponseInnerDetails {
    pub fn new() -> WorkbenchesAssetsActivity200ResponseInnerDetails {
        WorkbenchesAssetsActivity200ResponseInnerDetails {
            asset_id: None,
            container_id: None,
            created_at: None,
            updated_at: None,
            has_agent: None,
            has_plugin_results: None,
            first_scan_time: None,
            last_scan_time: None,
            last_authenticated_scan_time: None,
            last_licensed_scan_time: None,
            last_licensed_scan_time_v2: None,
            sources: None,
            terminated_at: None,
            terminated_by: None,
            deleted_at: None,
            deleted_by: None,
            properties: None,
        }
    }
}

