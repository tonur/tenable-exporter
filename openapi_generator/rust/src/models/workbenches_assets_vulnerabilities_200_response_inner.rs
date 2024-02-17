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
pub struct WorkbenchesAssetsVulnerabilities200ResponseInner {
    /// The UUID of the asset.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A count of vulnerabilities by severity.
    #[serde(rename = "severities", skip_serializing_if = "Option::is_none")]
    pub severities: Option<Vec<crate::models::WorkbenchesAssetsVulnerabilities200ResponseInnerSeveritiesInner>>,
    /// The total number of vulnerabilities detected on the asset.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// A list of fully-qualified domain names (FQDNs) for the asset.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<Vec<String>>,
    /// A list of ipv4 addresses for the asset.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    /// A list of ipv6 addresses for the asset.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
    /// The ISO timestamp of the scan that most recently detected the asset.
    #[serde(rename = "last_seen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// The NetBIOS name for the asset.
    #[serde(rename = "netbios_name", skip_serializing_if = "Option::is_none")]
    pub netbios_name: Option<Vec<String>>,
    /// The names of any Nessus agents that scanned and identified the asset.
    #[serde(rename = "agent_name", skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<Vec<String>>,
}

impl WorkbenchesAssetsVulnerabilities200ResponseInner {
    pub fn new() -> WorkbenchesAssetsVulnerabilities200ResponseInner {
        WorkbenchesAssetsVulnerabilities200ResponseInner {
            id: None,
            severities: None,
            total: None,
            fqdn: None,
            ipv4: None,
            ipv6: None,
            last_seen: None,
            netbios_name: None,
            agent_name: None,
        }
    }
}


