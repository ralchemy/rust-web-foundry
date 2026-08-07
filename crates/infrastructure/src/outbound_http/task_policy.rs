use application::{TaskPolicy, TaskPolicyError};
use domain::TaskTitle;
use fastrace::{future::FutureExt, local::LocalSpan, prelude::Span};
use reqwest::{Client, StatusCode, redirect::Policy, retry};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct HttpTaskPolicy {
    client: Client,
    url: reqwest::Url,
}

#[derive(Serialize)]
struct PolicyRequest<'a> {
    title: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PolicyResponse {
    allowed: bool,
}

impl HttpTaskPolicy {
    pub fn new(
        url: String,
        timeout: Duration,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if timeout.is_zero() {
            return Err(std::io::Error::other("task policy timeout must be positive").into());
        }
        let url = reqwest::Url::parse(&url)?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err(std::io::Error::other("task policy URL must use http or https").into());
        }
        let client = Client::builder()
            .timeout(timeout)
            // A timeout leaves a POST's downstream outcome uncertain; never retry it.
            .retry(retry::never())
            .redirect(Policy::none())
            .build()?;
        Ok(Self { client, url })
    }
}

impl TaskPolicy for HttpTaskPolicy {
    async fn is_allowed(&self, title: &TaskTitle) -> Result<bool, TaskPolicyError> {
        let span = Span::enter_with_local_parent("task_policy.check").with_properties(|| {
            [
                ("span.kind", "client"),
                ("http.request.method", "POST"),
                ("server.address", "task-policy"),
                ("http.route", "/check"),
            ]
        });
        async {
            let response = self
                .client
                .post(self.url.clone())
                .headers(fastrace_reqwest::traceparent_headers())
                .json(&PolicyRequest {
                    title: title.as_str(),
                })
                .send()
                .await
                .map_err(|_| {
                    mark_error("task_policy_unavailable");
                    TaskPolicyError::Unavailable
                })?;

            let status = response.status();
            LocalSpan::add_property(|| ("http.response.status_code", status.as_u16().to_string()));
            if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
                mark_error("task_policy_unavailable");
                return Err(TaskPolicyError::Unavailable);
            }
            if status != StatusCode::OK {
                mark_error("task_policy_bad_response");
                return Err(TaskPolicyError::BadResponse);
            }
            response
                .json::<PolicyResponse>()
                .await
                .map(|response| response.allowed)
                .map_err(|_| {
                    mark_error("task_policy_bad_response");
                    TaskPolicyError::BadResponse
                })
        }
        .in_span(span)
        .await
    }
}

fn mark_error(category: &'static str) {
    LocalSpan::add_properties(|| [("span.status_code", "error"), ("error.type", category)]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_policy_client_settings_at_startup() {
        assert!(
            HttpTaskPolicy::new("http://localhost/check".into(), Duration::from_secs(1)).is_ok()
        );
        assert!(
            HttpTaskPolicy::new("ftp://localhost/check".into(), Duration::from_secs(1)).is_err()
        );
        assert!(HttpTaskPolicy::new("http://localhost/check".into(), Duration::ZERO).is_err());
    }
}
