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
pub struct VulnerabilitiesImportRequestAssetsInnerNetworkInterfacesInner {
    /// A list of IPv4 address that the scan identified as associated with the network interface.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    /// A list of IPv6 addresses that the scan identified as associated with the network interface.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
    /// The MAC address of the network interface.
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// The NETBIOS name of the network interface.
    #[serde(rename = "netbios_name", skip_serializing_if = "Option::is_none")]
    pub netbios_name: Option<String>,
    /// The fully-qualified domain name (FQDN) of the network interface.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}

impl VulnerabilitiesImportRequestAssetsInnerNetworkInterfacesInner {
    pub fn new() -> VulnerabilitiesImportRequestAssetsInnerNetworkInterfacesInner {
        VulnerabilitiesImportRequestAssetsInnerNetworkInterfacesInner {
            ipv4: None,
            ipv6: None,
            mac_address: None,
            netbios_name: None,
            fqdn: None,
        }
    }
}


