use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::holders::context::compute_if_absent;
use crate::holders::context::CLIENT;
use crate::structs::common::ParsedSpec;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use log::debug;
use openapiv3::ReferenceOr;
use serde::de::DeserializeOwned;
use url::Url;

pub fn resolve_reference<T>(uri: &str, parsed_spec: &ParsedSpec) -> Result<&'static ReferenceOr<T>>
where
    T: DeserializeOwned + Send + Sync,
{
    let cached_value = compute_if_absent(String::from(uri), move || {
        let clean_uri = uri.split_once('#').map(|x| x.0).unwrap_or("/");

        let cached_schema = compute_if_absent(String::from(clean_uri), move || {
            Ok(match &uri {
                u if u.starts_with("#/") => parsed_spec.spec.clone(),
                u if u.starts_with("http://") || u.starts_with("https://") => Arc::new(
                    serde_yaml::from_reader(fetch_url_content(uri)?)
                        .with_context(|| format!("Could not parse yaml url content [{}]", &uri))?,
                ),
                _ => Arc::new(
                    serde_yaml::from_reader(fetch_file_content(&parsed_spec.path, uri)?)
                        .with_context(|| format!("Could not parse yaml file content [{}]", &uri))?,
                ),
            })
        })?;

        let target = cached_schema
            .pointer(&extract_json_pointer(uri))
            .ok_or(anyhow!("Pointer target [{}] not found in content", uri))?;

        Ok(ReferenceOr::Item(
            serde_json::from_value::<T>(target.clone())
                .with_context(|| format!("Could not produce target model [{}]", &uri))?,
        ))
    })?;

    Ok(cached_value)
}

fn extract_json_pointer(uri: &str) -> String {
    let pointer = String::from(uri.split_once('#').map(|x| x.1).unwrap_or("/"));
    debug!("Resolved json pointer [{}]", pointer);
    pointer
}

fn fetch_url_content(uri: &str) -> Result<Box<dyn Read>> {
    let mut url = Url::parse(uri).with_context(|| format!("Could not parse URI [{}]", &uri))?;
    url.set_fragment(None);
    debug!("Resolved external $ref [{}]", url);
    let response = CLIENT
        .get(url)
        .send()
        .with_context(|| format!("Remote request to [{}] error", &uri))?;
    if response.status().is_success() {
        Ok(Box::new(response))
    } else {
        Err(anyhow!("Failed to fetch the content by uri {}", uri))
    }
}

fn fetch_file_content(base_spec_path: &Path, uri: &str) -> Result<Box<dyn Read>> {
    let unparsed_path: Vec<_> = uri.split('#').collect();
    let ref_spec_path = base_spec_path.with_file_name(
        unparsed_path
            .first()
            .ok_or_else(|| anyhow!("Could not parse file path {}", uri))?,
    );

    debug!(
        "Base spec path [{:?}]\nResolved $ref path [{:?}]",
        fs::canonicalize(base_spec_path),
        fs::canonicalize(&ref_spec_path)
    );

    Ok(Box::new(File::open(&ref_spec_path).with_context(|| {
        format!("Could not open file [{:?}]", &ref_spec_path)
    })?))
}
