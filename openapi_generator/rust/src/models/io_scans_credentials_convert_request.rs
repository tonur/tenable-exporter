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
pub struct IoScansCredentialsConvertRequest {
    /// The name of the managed credentials. This name must be unique within your Tenable.io instance.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "settings")]
    pub settings: Box<crate::models::IoScansCredentialsConvertRequestSettings>,
    /// The system name that uniquely identifies the credentials type, for example, `Windows`.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The system name that uniquely identifies the credentials category, for example `Host`.
    #[serde(rename = "category")]
    pub category: String,
    /// A value specifying whether the credentials are scan-specific (`true`) or managed (`false`). You must use `false` for this request body attribute to convert scan-specific to managed credentials.
    #[serde(rename = "ad_hoc")]
    pub ad_hoc: bool,
    /// A list of user permissions for the managed credentials. If a request message omits this parameter, Tenable.io automatically creates a `permissions` object for the user account that submits the request.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<crate::models::IoScansCredentialsConvertRequestPermissionsInner>>,
}

impl IoScansCredentialsConvertRequest {
    pub fn new(name: String, settings: crate::models::IoScansCredentialsConvertRequestSettings, r#type: String, category: String, ad_hoc: bool) -> IoScansCredentialsConvertRequest {
        IoScansCredentialsConvertRequest {
            name,
            settings: Box::new(settings),
            r#type,
            category,
            ad_hoc,
            permissions: None,
        }
    }
}


