# ExportsAssetsExportStatus200ResponseStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The status of the export request. Possible values include:  - QUEUED—Tenable Vulnerability Management has queued the export request until it completes other requests currently in process.  - PROCESSING—Tenable Vulnerability Management has started processing the export request.  - FINISHED—Tenable Vulnerability Management has completed processing the export request. The list of chunks is complete.  - CANCELLED—An administrator has cancelled the export request.  - ERROR—Tenable Vulnerability Management encountered an error while processing the export request. Tenable recommends that you retry the request. If the status persists on retry, contact Support. | [optional]
**chunks_available** | Option<**Vec<i32>**> | A list of completed chunks available for download. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

