//! A server lokraft can talk to.

use serde::{Deserialize, Serialize};

/// A named server address, with the key it asks for when it asks for one.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Connection {
    pub name: String,
    pub url: String,
    /// Blank is read as absent: an untouched field attaches no credential.
    pub api_key: Option<String>,
}

impl Connection {
    pub fn new(name: &str, url: &str, api_key: Option<&str>) -> Self {
        Self {
            name: name.trim().to_string(),
            url: url.trim().trim_end_matches('/').to_string(),
            api_key: api_key
                .map(str::trim)
                .filter(|k| !k.is_empty())
                .map(str::to_string),
        }
    }
}
