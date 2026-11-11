use std::path::Path;
use std::time::Duration;

use axum::body::Body;
use axum::http::Response as HttpResponse;
use axum::http::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::response::Response;
use loco_rs::app::AppContext;
use loco_rs::prelude::{Error, Result};
use loco_rs::storage::strategies::single::SingleStrategy;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use url::Url;

const SIGNED_URL_TTL: Duration = Duration::from_secs(300);

pub struct Streamable<'a> {
    pub disk: &'a str,
    pub path: &'a str,
    pub basename: &'a str,
    pub mimetype: &'a str,
    pub nginx_redirect: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub enum Disposition {
    Inline,
    Attachment,
}

impl Disposition {
    fn as_str(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Attachment => "attachment",
        }
    }
}

pub struct NginxStream;

impl NginxStream {
    pub async fn stream(
        ctx: &AppContext,
        file: &Streamable<'_>,
        disposition: Disposition,
    ) -> Result<Response> {
        let strategy = SingleStrategy::new(file.disk);

        let signed = ctx
            .storage
            .presign_get_with_policy(Path::new(file.path), SIGNED_URL_TTL, &strategy)
            .await?;

        if signed.headers.keys().any(|name| name.as_str() != "host") {
            return Err(Error::Message(
                "Signed GET requires additional headers".into(),
            ));
        }

        let signed_url = signed.url();

        let url = Url::parse(&signed_url).map_err(|e| Error::Message(e.to_string()))?;

        if url.scheme() != "https" || url.port().is_some() || url.fragment().is_some() {
            return Err(Error::Message("Unsupported signed URL".into()));
        }

        let host = url
            .host_str()
            .ok_or_else(|| Error::Message("Signed URL without host".into()))?;

        let mut path_query = url.path().to_owned();

        if let Some(query) = url.query() {
            path_query.push('?');
            path_query.push_str(query);
        }

        let nginx_redirect = file.nginx_redirect;

        let redirect = format!("{nginx_redirect}/https/{host}{path_query}");

        let encoded = utf8_percent_encode(file.basename, NON_ALPHANUMERIC);

        let ascii_filename: String = file
            .basename
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        let content_disposition = format!(
            "{}; filename=\"{}\"; filename*=UTF-8''{}",
            disposition.as_str(),
            ascii_filename,
            encoded,
        );

        HttpResponse::builder()
            .header(ACCEPT_RANGES, "bytes")
            .header(CONTENT_TYPE, file.mimetype)
            .header(CONTENT_DISPOSITION, content_disposition)
            .header("X-Accel-Redirect", redirect)
            .body(Body::empty())
            .map_err(|e| Error::Message(e.to_string()))
    }
}
