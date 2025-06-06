/*
 * iikoCloud API
 *
 * <h3>Description of common data formats:</h3><b>uuid</b> - string in UUID(universally unique identifier).<br/>Examples: <i>550e8400-e29b-41d4-a716-446655440000, b090de0b-8550-6e17-70b2-bbba152bcbd3</i><br/><br/><b>date-time</b> - date and time string in custom string format <b>yyyy-MM-dd HH:mm:ss.fff</b>.<br/>Examples: <i>2017-04-29 20:45:00.000, 2018-01-01 01:01:30.123</i>
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`api1_access_token_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Api1AccessTokenPostError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status408(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

pub async fn api1_access_token_post(
    configuration: &configuration::Configuration,
    timeout: Option<i32>,
    get_access_token_request: Option<models::GetAccessTokenRequest>,
) -> Result<models::GetAccessTokenResponse, Error<Api1AccessTokenPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timeout = timeout;
    let p_get_access_token_request = get_access_token_request;

    let uri_str = format!("{}/api/1/access_token", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_timeout {
        req_builder = req_builder.header("Timeout", param_value.to_string());
    }
    req_builder = req_builder.json(&p_get_access_token_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetAccessTokenResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetAccessTokenResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Api1AccessTokenPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
