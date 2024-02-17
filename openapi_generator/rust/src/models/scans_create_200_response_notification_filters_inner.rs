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
pub struct ScansCreate200ResponseNotificationFiltersInner {
    /// The attribute value Tenable.io filters on. For example, when filtering on severity, this attribute might specify `Critical`.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The operator Tenable.io applies to the filter value, for example, `eq`.
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// The attribute name. For example, use the `risk_factor` attribute if you want to filter on vulnerability severity.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

impl ScansCreate200ResponseNotificationFiltersInner {
    pub fn new() -> ScansCreate200ResponseNotificationFiltersInner {
        ScansCreate200ResponseNotificationFiltersInner {
            value: None,
            quality: None,
            filter: None,
        }
    }
}


