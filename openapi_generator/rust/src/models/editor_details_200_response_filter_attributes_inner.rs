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
pub struct EditorDetails200ResponseFilterAttributesInner {
    /// The short name of the filter.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The long name of the filter.
    #[serde(rename = "readable_name", skip_serializing_if = "Option::is_none")]
    pub readable_name: Option<String>,
    /// The comparison options for the filter.
    #[serde(rename = "operators", skip_serializing_if = "Option::is_none")]
    pub operators: Option<Vec<serde_json::Value>>,
    #[serde(rename = "control", skip_serializing_if = "Option::is_none")]
    pub control: Option<Box<crate::models::EditorDetails200ResponseFilterAttributesInnerControl>>,
}

impl EditorDetails200ResponseFilterAttributesInner {
    pub fn new() -> EditorDetails200ResponseFilterAttributesInner {
        EditorDetails200ResponseFilterAttributesInner {
            name: None,
            readable_name: None,
            operators: None,
            control: None,
        }
    }
}


