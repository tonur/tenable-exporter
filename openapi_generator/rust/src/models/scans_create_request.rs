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
pub struct ScansCreateRequest {
    /// The UUID for the Tenable-provided scan template to use. Use the [GET /editor/scan/templates](ref:editor-list-templates) endpoint to find the template UUID.  **Caution:** The defaults listed for a template via the [GET /editor/{type}/templates/{template_uuid}](ref:editor-template-details) endpoint apply to the user interface only. When you create a scan via the API you must include the settings in the request even if the setting is listed as a default in the editor. For example, even if `host_tagging` is set to `yes` by default in the editor, you still need to include `\"host_tagging\": \"yes\"` in the `settings` object for the scan.
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "settings")]
    pub settings: Box<crate::models::ScansCreateRequestSettings>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Box<crate::models::ScansCreateRequestCredentials>>,
    #[serde(rename = "plugins", skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Box<crate::models::ScansCreateRequestPlugins>>,
}

impl ScansCreateRequest {
    pub fn new(uuid: String, settings: crate::models::ScansCreateRequestSettings) -> ScansCreateRequest {
        ScansCreateRequest {
            uuid,
            settings: Box::new(settings),
            credentials: None,
            plugins: None,
        }
    }
}

