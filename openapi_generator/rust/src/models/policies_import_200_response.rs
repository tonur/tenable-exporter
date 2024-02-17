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
pub struct PoliciesImport200Response {
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<i32>,
    #[serde(rename = "no_target", skip_serializing_if = "Option::is_none")]
    pub no_target: Option<String>,
    #[serde(rename = "template_uuid", skip_serializing_if = "Option::is_none")]
    pub template_uuid: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<i32>,
    #[serde(rename = "user_permissions", skip_serializing_if = "Option::is_none")]
    pub user_permissions: Option<i32>,
    #[serde(rename = "last_modification_date", skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<i32>,
    #[serde(rename = "creation_date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<i32>,
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl PoliciesImport200Response {
    pub fn new() -> PoliciesImport200Response {
        PoliciesImport200Response {
            private: None,
            no_target: None,
            template_uuid: None,
            description: None,
            name: None,
            owner: None,
            shared: None,
            user_permissions: None,
            last_modification_date: None,
            creation_date: None,
            owner_id: None,
            id: None,
        }
    }
}


