/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoExportsComplianceDownload200ResponseInner : 



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoExportsComplianceDownload200ResponseInner {
    /// The UUID of the asset on which the compliance check was executed.
    #[serde(rename = "asset_uuid", skip_serializing_if = "Option::is_none")]
    pub asset_uuid: Option<String>,
    /// The Unix timestamp when a compliance scan first assessed the asset with the compliance check.
    #[serde(rename = "first_seen", skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<i64>,
    /// The Unix timestamp when a compliance scan last assessed the asset with the compliance check.
    #[serde(rename = "last_seen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<i64>,
    /// The name of the audit file containing the compliance check.
    #[serde(rename = "audit_file", skip_serializing_if = "Option::is_none")]
    pub audit_file: Option<String>,
    /// The unique identifier for the compliance finding.
    #[serde(rename = "check_id", skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    /// The descriptive name of the compliance check.
    #[serde(rename = "check_name", skip_serializing_if = "Option::is_none")]
    pub check_name: Option<String>,
    /// Full text description of the compliance check.
    #[serde(rename = "check_info", skip_serializing_if = "Option::is_none")]
    pub check_info: Option<String>,
    /// The desired value (integer or string) for the compliance check. For example, if a password length compliance check requires passwords to be 8 characters long then `8` is the expected value. For manual checks, this field will contain the command used for the compliance check.
    #[serde(rename = "expected_value", skip_serializing_if = "Option::is_none")]
    pub expected_value: Option<String>,
    /// The actual value (integer, string, or table) evaluated from the compliance check. For example, if a password length compliance check requires passwords to be 8 characters long, but the evaluated value was 7 then `7` is the actual value. For manual checks, this field will contain the output of the command that was executed.
    #[serde(rename = "actual_value", skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<String>,
    /// Indicates the result of the compliance check for the given asset.   Possible values include:  - PASSED—Returned if the asset has passed the compliance check.  - FAILED—Returned if the asset has failed the compliance check.  - WARNING—Returned in cases where there is no definable passing criteria; for example, an audit where you must verify that members of the administrator group are appropriate for your organization.  - SKIPPED—Returned if the plugin determines that the check is not applicable to the asset. It can also be returned in other various cases; for example, when a check requires that a direct command be run to gather data on an offline network device or if a check contains commands that won't run on the specified OS.  - UNKNOWN—Returned when a status cannot be determined for the OVAL check. This status is set by the OVAL engine.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Industry references for the compliance check.
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<Vec<crate::models::IoExportsComplianceDownload200ResponseInnerReferenceInner>>,
    /// Links to external websites that contain reference information about the compliance check.
    #[serde(rename = "see_also", skip_serializing_if = "Option::is_none")]
    pub see_also: Option<String>,
    /// Remediation information for the compliance check.
    #[serde(rename = "solution", skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
    /// An error message if the compliance evaluation fails.
    #[serde(rename = "check_error", skip_serializing_if = "Option::is_none")]
    pub check_error: Option<String>,
    /// The name of the profile for the benchmark standard.
    #[serde(rename = "profile_name", skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// The type of database if the compliance check assessed a database.
    #[serde(rename = "db_type", skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// The unique ID of the compliance plugin.
    #[serde(rename = "plugin_id", skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<i32>,
    /// Indicates whether or not the finding applies to the asset based on the last assessment. This field is `NULL` for findings last seen before December 2021.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl IoExportsComplianceDownload200ResponseInner {
    /// 
    pub fn new() -> IoExportsComplianceDownload200ResponseInner {
        IoExportsComplianceDownload200ResponseInner {
            asset_uuid: None,
            first_seen: None,
            last_seen: None,
            audit_file: None,
            check_id: None,
            check_name: None,
            check_info: None,
            expected_value: None,
            actual_value: None,
            status: None,
            reference: None,
            see_also: None,
            solution: None,
            check_error: None,
            profile_name: None,
            db_type: None,
            plugin_id: None,
            state: None,
        }
    }
}


