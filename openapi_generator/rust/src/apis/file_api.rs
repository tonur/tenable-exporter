/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`file_upload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileUploadError {
    Status429(),
    Status500(),
    UnknownValue(serde_json::Value),
}


/// Uploads a file.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>
pub async fn file_upload(configuration: &configuration::Configuration, no_enc: Option<i32>, filedata: Option<std::path::PathBuf>) -> Result<crate::models::FileUpload200Response, Error<FileUploadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/file/upload", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = no_enc {
        local_var_req_builder = local_var_req_builder.query(&[("no_enc", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-ApiKeys", local_var_value);
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'Filedata' parameter
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FileUploadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

