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
pub struct IoScansCheckAutoTargetsRequest {
    /// Specify a value as follows:<ul><li>If your scans involve separate environments with overlapping IP ranges, specify the UUID of the [network](doc:manage-networks-tio) you want to associate with the results of the auto-routed scan. This value must match the network where you have assigned the scanner groups that you configured for scan routing.</li><li>Otherwise, specify the default network (00000000-0000-0000-0000-000000000000).</li></ul>
    #[serde(rename = "network_uuid")]
    pub network_uuid: String,
    /// A list of asset tags UUIDs.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// A comma-delimited list of targets to scan. For a full list of supported target formats, see the [Tenable Vulnerability Management User Guide](https://docs.tenable.com/vulnerability-management/Content/Scans/AboutScanTargets.htm).
    #[serde(rename = "target_list", skip_serializing_if = "Option::is_none")]
    pub target_list: Option<String>,
}

impl IoScansCheckAutoTargetsRequest {
    pub fn new(network_uuid: String) -> IoScansCheckAutoTargetsRequest {
        IoScansCheckAutoTargetsRequest {
            network_uuid,
            tags: None,
            target_list: None,
        }
    }
}


