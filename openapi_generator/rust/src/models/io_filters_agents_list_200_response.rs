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
pub struct IoFiltersAgentsList200Response {
    /// Array of strings which represent each field which supports \"wildcard\" search. Wildcard search is a mechanism where multiple fields of a record are filtered against one specific filter string. If any one of the supported fields' values matches against the filter string, then the record matches the wildcard filter. Note that for a record to be returned, it must pass the wildcard filter (if there is one) AND the set of standard filters.
    #[serde(rename = "wildcard_fields", skip_serializing_if = "Option::is_none")]
    pub wildcard_fields: Option<Vec<String>>,
    /// A list of filters available for the record type.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::models::IoFiltersAgentsList200ResponseFiltersInner>>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Box<crate::models::IoFiltersAgentsList200ResponseSort>>,
}

impl IoFiltersAgentsList200Response {
    pub fn new() -> IoFiltersAgentsList200Response {
        IoFiltersAgentsList200Response {
            wildcard_fields: None,
            filters: None,
            sort: None,
        }
    }
}


