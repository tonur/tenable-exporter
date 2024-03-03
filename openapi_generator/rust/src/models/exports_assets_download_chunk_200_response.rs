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
pub struct ExportsAssetsDownloadChunk200Response {
    /// The UUID of the asset in Tenable Vulnerability Management. Use this value as the unique key for the asset.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Specifies whether a Nessus agent scan identified the asset.
    #[serde(rename = "has_agent", skip_serializing_if = "Option::is_none")]
    pub has_agent: Option<bool>,
    /// Specifies whether the asset has plugin results associated with it.
    #[serde(rename = "has_plugin_results", skip_serializing_if = "Option::is_none")]
    pub has_plugin_results: Option<bool>,
    /// The time and date when Tenable Vulnerability Management created the asset record.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time and date when a user terminated the Amazon Web Service (AWS) virtual machine instance of the asset.
    #[serde(rename = "terminated_at", skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
    /// The user who terminated the AWS instance of the asset.
    #[serde(rename = "terminated_by", skip_serializing_if = "Option::is_none")]
    pub terminated_by: Option<String>,
    /// The time and date when the asset record was last updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The time and date when a user deleted the asset record. When a user deletes an asset record, Tenable Vulnerability Management retains the record until the asset ages out of the license count.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// The user who deleted the asset record.
    #[serde(rename = "deleted_by", skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    /// The time and date when a scan first identified the asset.
    #[serde(rename = "first_seen", skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    /// The time and date of the scan that most recently identified the asset.
    #[serde(rename = "last_seen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// The time and date of the first scan run against the asset.
    #[serde(rename = "first_scan_time", skip_serializing_if = "Option::is_none")]
    pub first_scan_time: Option<String>,
    /// The time and date of the last scan run against the asset.
    #[serde(rename = "last_scan_time", skip_serializing_if = "Option::is_none")]
    pub last_scan_time: Option<String>,
    /// The time and date of the last credentialed scan run on the asset.
    #[serde(rename = "last_authenticated_scan_date", skip_serializing_if = "Option::is_none")]
    pub last_authenticated_scan_date: Option<String>,
    /// The time and date of the last scan that identified the asset as licensed. Tenable Vulnerability Management categorizes an asset as licensed if a scan of that asset has returned results from a non-discovery plugin within the last 90 days.
    #[serde(rename = "last_licensed_scan_date", skip_serializing_if = "Option::is_none")]
    pub last_licensed_scan_date: Option<String>,
    /// The UUID of the scan configuration used during the last scan of the asset.
    #[serde(rename = "last_scan_id", skip_serializing_if = "Option::is_none")]
    pub last_scan_id: Option<String>,
    /// The `schedule_uuid` for the last scan of the asset.
    #[serde(rename = "last_schedule_id", skip_serializing_if = "Option::is_none")]
    pub last_schedule_id: Option<String>,
    /// The unique identifier of the Microsoft Azure virtual machine instance. For more information, see \"Accessing and Using Azure VM Unique ID\" in the Microsoft Azure documentation.
    #[serde(rename = "azure_vm_id", skip_serializing_if = "Option::is_none")]
    pub azure_vm_id: Option<String>,
    /// The unique identifier of the resource in the Azure Resource Manager. For more information, see the Azure Resource Manager Documentation.
    #[serde(rename = "azure_resource_id", skip_serializing_if = "Option::is_none")]
    pub azure_resource_id: Option<String>,
    /// The unique identifier of the virtual machine instance in Google Cloud Platform (GCP).
    #[serde(rename = "gcp_project_id", skip_serializing_if = "Option::is_none")]
    pub gcp_project_id: Option<String>,
    /// The customized name of the project to which the virtual machine instance belongs in GCP. For more information see \"Creating and Managing Projects\" in the GCP documentation.
    #[serde(rename = "gcp_zone", skip_serializing_if = "Option::is_none")]
    pub gcp_zone: Option<String>,
    /// The zone where the virtual machine instance runs in GCP. For more information, see \"Regions and Zones\" in the GCP documentation.
    #[serde(rename = "gcp_instance_id", skip_serializing_if = "Option::is_none")]
    pub gcp_instance_id: Option<String>,
    /// The unique identifier of the Linux AMI image in Amazon Elastic Compute Cloud (Amazon EC2). For more information, see the Amazon Elastic Compute Cloud Documentation.
    #[serde(rename = "aws_ec2_instance_ami_id", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_instance_ami_id: Option<String>,
    /// The unique identifier of the Linux instance in Amazon EC2. For more information, see the Amazon Elastic Compute Cloud Documentation.
    #[serde(rename = "aws_ec2_instance_id", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_instance_id: Option<String>,
    /// This property represents the `tenable_uuid`. This identifier can originate from either an agent or a credentialed remote Nessus scan. If no agent is present on the asset, a UUID is assigned by Tenable Vulnerability Management during a credentialed scan when the [Create unique identifier on hosts scanned with credentials](https://docs.tenable.com/vulnerability-management/Content/Scans/AdvancedSettings.htm) option is enabled. Note that no UUID is set for an uncredentialed non-agent scans.
    #[serde(rename = "agent_uuid", skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The BIOS UUID of the asset.
    #[serde(rename = "bios_uuid", skip_serializing_if = "Option::is_none")]
    pub bios_uuid: Option<String>,
    /// The ID of the network object associated with scanners that identified the asset. The default network ID is `00000000-0000-0000-0000-000000000000`. For more information about network objects, see [Manage Networks](doc:manage-networks-tio).
    #[serde(rename = "network_id", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// The ID of the network object associated with scanners that identified the asset. The default network name is `Default`. All other network names are user-defined. For more information about network objects, see [Manage Networks](doc:manage-networks-tio).
    #[serde(rename = "network_name", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// The canonical user identifier for the AWS account associated with the virtual machine instance. For example, `79a59df900b949e55d96a1e698fbacedfd6e09d98eacf8f8d5218e7cd47ef2be`. For more information, see AWS Account Identifiers in the AWS documentation.
    #[serde(rename = "aws_owner_id", skip_serializing_if = "Option::is_none")]
    pub aws_owner_id: Option<String>,
    /// The availability zone where Amazon Web Services hosts the virtual machine instance, for example, `us-east-1a``. Availability zones are subdivisions of AWS regions. For more information, see \"Regions and Availability Zones\" in the AWS documentation.
    #[serde(rename = "aws_availability_zone", skip_serializing_if = "Option::is_none")]
    pub aws_availability_zone: Option<String>,
    /// The region where AWS hosts the virtual machine instance, for example, `us-east-1`. For more information, see \"Regions and Availability Zones\" in the AWS documentation.
    #[serde(rename = "aws_region", skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// The unique identifier for the virtual public cloud that hosts the AWS virtual machine instance. For more information, see the Amazon Virtual Private Cloud User Guide.
    #[serde(rename = "aws_vpc_id", skip_serializing_if = "Option::is_none")]
    pub aws_vpc_id: Option<String>,
    /// The virtual machine instance's group in AWS.
    #[serde(rename = "aws_ec2_instance_group_name", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_instance_group_name: Option<String>,
    /// The state of the virtual machine instance in AWS at the time of the scan.
    #[serde(rename = "aws_ec2_instance_state_name", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_instance_state_name: Option<String>,
    /// The type of instance in AWS EC2.
    #[serde(rename = "aws_ec2_instance_type", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_instance_type: Option<String>,
    /// The unique identifier of the AWS subnet where the virtual machine instance was running at the time of the scan.
    #[serde(rename = "aws_subnet_id", skip_serializing_if = "Option::is_none")]
    pub aws_subnet_id: Option<String>,
    /// The product code associated with the AMI used to launch the virtual machine instance in AWS EC2.
    #[serde(rename = "aws_ec2_product_code", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_product_code: Option<String>,
    /// The name of the virtual machine instance in AWS EC2.
    #[serde(rename = "aws_ec2_name", skip_serializing_if = "Option::is_none")]
    pub aws_ec2_name: Option<String>,
    /// The unique identifier of the asset in McAfee ePolicy Orchestrator (ePO). For more information, see the McAfee documentation.
    #[serde(rename = "mcafee_epo_guid", skip_serializing_if = "Option::is_none")]
    pub mcafee_epo_guid: Option<String>,
    /// The unique identifier of the McAfee ePO agent that identified the asset. For more information, see the McAfee documentation.
    #[serde(rename = "mcafee_epo_agent_guid", skip_serializing_if = "Option::is_none")]
    pub mcafee_epo_agent_guid: Option<String>,
    /// The unique record identifier of the asset in ServiceNow. For more information, see the ServiceNow documentation.
    #[serde(rename = "servicenow_sysid", skip_serializing_if = "Option::is_none")]
    pub servicenow_sysid: Option<String>,
    /// The unique identifiers of the asset in HCL BigFix. For more information, see the HCL BigFix documentation.
    #[serde(rename = "bigfix_asset_id", skip_serializing_if = "Option::is_none")]
    pub bigfix_asset_id: Option<String>,
    /// The names of any Nessus agents that scanned and identified the asset.
    #[serde(rename = "agent_names", skip_serializing_if = "Option::is_none")]
    pub agent_names: Option<Vec<String>>,
    /// A list of Common Platform Enumeration (CPE) values that represent software applications a scan identified as present on an asset. This attribute supports the CPE 2.2 format. For more information, see the \"Component Syntax\" section of the [CPE Specification, Version 2.2](https://cpe.mitre.org/files/cpe-specification_2.2.pdf). For assets identified in Tenable scans, this attribute contains data only if a scan using [Nessus Plugin ID 45590](https://www.tenable.com/plugins/nessus/45590) has evaluated the asset.  **Note:** If no scan detects an application within 30 days of the scan that originally detected the application, Tenable Vulnerability Management considers the detection of that application expired. As a result, the next time a scan evaluates the asset, Tenable Vulnerability Management removes the expired application from the installed_software attribute. This activity is logged as a `remove` type of `attribute_change` update in the asset activity log.
    #[serde(rename = "installed_software", skip_serializing_if = "Option::is_none")]
    pub installed_software: Option<Vec<String>>,
    /// The IPv4 addresses that scans have associated with the asset record.
    #[serde(rename = "ipv4s", skip_serializing_if = "Option::is_none")]
    pub ipv4s: Option<Vec<String>>,
    /// The IPv6 addresses that scans have associated with the asset record.
    #[serde(rename = "ipv6s", skip_serializing_if = "Option::is_none")]
    pub ipv6s: Option<Vec<String>>,
    /// The fully-qualified domain names that scans have associated with the asset record.
    #[serde(rename = "fqdns", skip_serializing_if = "Option::is_none")]
    pub fqdns: Option<Vec<String>>,
    /// The MAC addresses that scans have associated with the asset record.
    #[serde(rename = "mac_addresses", skip_serializing_if = "Option::is_none")]
    pub mac_addresses: Option<Vec<String>>,
    /// The NetBIOS names that scans have associated with the asset record.
    #[serde(rename = "netbios_names", skip_serializing_if = "Option::is_none")]
    pub netbios_names: Option<Vec<String>>,
    /// The operating systems that scans have associated with the asset record.
    #[serde(rename = "operating_systems", skip_serializing_if = "Option::is_none")]
    pub operating_systems: Option<Vec<String>>,
    /// The system types as reported by Plugin ID 54615. Possible values include `router`, `general-purpose`, `scan-host`, and `embedded`.
    #[serde(rename = "system_types", skip_serializing_if = "Option::is_none")]
    pub system_types: Option<Vec<String>>,
    /// The hostnames that scans have associated with the asset record.
    #[serde(rename = "hostnames", skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
    /// The SSH key fingerprints that scans have associated with the asset record.
    #[serde(rename = "ssh_fingerprints", skip_serializing_if = "Option::is_none")]
    pub ssh_fingerprints: Option<Vec<String>>,
    /// The Asset ID of the asset in Qualys. For more information, see the Qualys documentation.  **Note:** Tenable is enabling Qualys asset import for customers in a rolling fashion. For more information, contact your Tenable representative.
    #[serde(rename = "qualys_asset_ids", skip_serializing_if = "Option::is_none")]
    pub qualys_asset_ids: Option<Vec<String>>,
    /// The Host ID of the asset in Qualys. For more information, see the Qualys documentation.  **Note:** Tenable is enabling Qualys asset import for customers in a rolling fashion. For more information, contact your Tenable representative.
    #[serde(rename = "qualys_host_ids", skip_serializing_if = "Option::is_none")]
    pub qualys_host_ids: Option<Vec<String>>,
    /// The manufacturer's unique identifiers of the Trusted Platform Module (TPM) associated with the asset.
    #[serde(rename = "manufacturer_tpm_ids", skip_serializing_if = "Option::is_none")]
    pub manufacturer_tpm_ids: Option<Vec<String>>,
    /// The hardware keys for the asset in Symantec Endpoint Protection.
    #[serde(rename = "symantec_ep_hardware_keys", skip_serializing_if = "Option::is_none")]
    pub symantec_ep_hardware_keys: Option<Vec<String>>,
    /// The sources of the scans that identified the asset. An asset source is the entity that reported the asset details. Sources can include sensors, connectors, and API imports. If your request specifies multiple sources, Tenable Vulnerability Management returns all assets seen by any of the specified sources.  The items in the `sources` array must correspond to the names of the sources as defined in your organization's implementation of Tenable Vulnerability Management. Commonly used names include:  - AWS—The asset data was obtained from an Amazon Web Services connector.  - NESSUS_AGENT—The asset data was obtained from a Tenable Nessus Agent scan.  - PVS—The asset data from a Tenable Nessus Network Monitor (NNM) scan.  - NESSUS_SCAN—The asset data was obtained from a Tenable Nessus scan.  - WAS—The asset data was obtained from a Tenable Web App Scanning scan.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<crate::models::ExportsAssetsDownloadChunk200ResponseSourcesInner>>,
    /// Category tags assigned to the asset in Tenable Vulnerability Management.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ExportsAssetsDownloadChunk200ResponseTagsInner>>,
    /// The network interfaces that scans identified on the asset.
    #[serde(rename = "network_interfaces", skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<crate::models::ExportsAssetsDownloadChunk200ResponseNetworkInterfacesInner>>,
    /// The Asset Criticality Rating (ACR) for the asset. With Lumin, Tenable assigns an ACR to each asset on your network to represent the asset's relative risk as an integer from 1 to 10. For more information, see [Lumin Metrics](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminMetrics.htm) in the *Tenable Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license.
    #[serde(rename = "acr_score", skip_serializing_if = "Option::is_none")]
    pub acr_score: Option<String>,
    /// The Asset Exposure Score (AES) for the asset. For more information, see [Lumin Metrics](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminMetrics.htm) in the *Tenable Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license.
    #[serde(rename = "exposure_score", skip_serializing_if = "Option::is_none")]
    pub exposure_score: Option<String>,
    /// An array of open ports and their services as reported by the info-level plugins. For more information about open ports reported by info-level plugins, see [Relocate Open Port Findings](https://docs.tenable.com/vulnerability-management/Content/Settings/General/General.htm#Relocate-Open-Port-Findings) in the Tenable Vulnerability Management User Guide and the [Relocate Open Port Findings Quick Reference Guide](https://docs.tenable.com/quick-reference/relocate-open-port-findings/).
    #[serde(rename = "open_ports", skip_serializing_if = "Option::is_none")]
    pub open_ports: Option<Vec<crate::models::ExportsAssetsDownloadChunk200ResponseOpenPortsInner>>,
}

impl ExportsAssetsDownloadChunk200Response {
    pub fn new() -> ExportsAssetsDownloadChunk200Response {
        ExportsAssetsDownloadChunk200Response {
            id: None,
            has_agent: None,
            has_plugin_results: None,
            created_at: None,
            terminated_at: None,
            terminated_by: None,
            updated_at: None,
            deleted_at: None,
            deleted_by: None,
            first_seen: None,
            last_seen: None,
            first_scan_time: None,
            last_scan_time: None,
            last_authenticated_scan_date: None,
            last_licensed_scan_date: None,
            last_scan_id: None,
            last_schedule_id: None,
            azure_vm_id: None,
            azure_resource_id: None,
            gcp_project_id: None,
            gcp_zone: None,
            gcp_instance_id: None,
            aws_ec2_instance_ami_id: None,
            aws_ec2_instance_id: None,
            agent_uuid: None,
            bios_uuid: None,
            network_id: None,
            network_name: None,
            aws_owner_id: None,
            aws_availability_zone: None,
            aws_region: None,
            aws_vpc_id: None,
            aws_ec2_instance_group_name: None,
            aws_ec2_instance_state_name: None,
            aws_ec2_instance_type: None,
            aws_subnet_id: None,
            aws_ec2_product_code: None,
            aws_ec2_name: None,
            mcafee_epo_guid: None,
            mcafee_epo_agent_guid: None,
            servicenow_sysid: None,
            bigfix_asset_id: None,
            agent_names: None,
            installed_software: None,
            ipv4s: None,
            ipv6s: None,
            fqdns: None,
            mac_addresses: None,
            netbios_names: None,
            operating_systems: None,
            system_types: None,
            hostnames: None,
            ssh_fingerprints: None,
            qualys_asset_ids: None,
            qualys_host_ids: None,
            manufacturer_tpm_ids: None,
            symantec_ep_hardware_keys: None,
            sources: None,
            tags: None,
            network_interfaces: None,
            acr_score: None,
            exposure_score: None,
            open_ports: None,
        }
    }
}


