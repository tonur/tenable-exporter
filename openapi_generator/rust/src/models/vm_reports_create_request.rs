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
pub struct VmReportsCreateRequest {
    /// A name for the report.  If this parameter is omitted, Tenable Vulnerability Management uses the default name `Vulnerabilities_Export_Report` with a timestamp in ISO 8601 format appended to the end to create a unique name. For example, `Vulnerabilities_Export_Report_2023-11-30T00:23:13.199227748Z`.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of template to use for the report. The following templates are available:  - `host_vulns_summary`—An executive summary report that provides operations teams a snapshot of risk based on vulnerable assets.  - `host_vulns_by_plugins`—A report that provides a summary of the plugins that detected vulnerabilities on affected assets. Plugins are sorted by severity and the assets are sorted by the Asset Criticality Rating (ACR).  - `host_vulns_by_assets`—A summary of the most vulnerable assets.
    #[serde(rename = "template_name")]
    pub template_name: TemplateName,
}

impl VmReportsCreateRequest {
    pub fn new(template_name: TemplateName) -> VmReportsCreateRequest {
        VmReportsCreateRequest {
            name: None,
            template_name,
        }
    }
}

/// The type of template to use for the report. The following templates are available:  - `host_vulns_summary`—An executive summary report that provides operations teams a snapshot of risk based on vulnerable assets.  - `host_vulns_by_plugins`—A report that provides a summary of the plugins that detected vulnerabilities on affected assets. Plugins are sorted by severity and the assets are sorted by the Asset Criticality Rating (ACR).  - `host_vulns_by_assets`—A summary of the most vulnerable assets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateName {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "host_vulns_summary")]
    HostVulnsSummary,
    #[serde(rename = "host_vulns_by_plugins")]
    HostVulnsByPlugins,
    #[serde(rename = "host_vulns_by_assets")]
    HostVulnsByAssets,
}

impl Default for TemplateName {
    fn default() -> TemplateName {
        Self::Empty
    }
}
