# ScansPluginOutput200ResponseInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host_fqdn** | Option<**String**> | The host's fully qualified domain name; optional. | [optional]
**host_fqdn** | Option<**String**> | The FQDN of the host. Normally, this is populated with the value used to scan the host in the target list of the last scan that was ran where it was seen; always present. | [optional]
**host_ip** | Option<**String**> | The host's IPv4 address; optional. | [optional]
**host_uuid** | Option<**String**> | The host's UUID generated by Tenable.io for identification purposes; always present. | [optional]
**host_start** | Option<**String**> | The last time a scan was started for this host as an ISO 8601 timestamp; always present. | [optional]
**host_end** | Option<**String**> | The last time a scan was completed for this host as an ISO 8601 timestamp; always present. | [optional]
**mac_address** | Option<**String**> | The host's mac addresses in a newline-separated list; optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

