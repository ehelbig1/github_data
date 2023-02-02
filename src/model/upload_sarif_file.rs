use serde::Deserialize;
use url;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Response {
    id: String,
    url: url::Url
}

#[derive(Debug, PartialEq, Clone)]
pub struct Properties {
    pub owner: String,
    pub repo: String,
    pub commit_sha: String,
    pub r#ref: String,
    pub sarif: String
}

impl Properties {
    pub fn new(owner: String, repo: String, commit_sha: String, r#ref: String, sarif: String) -> Self {
        Self {
            owner,
            repo,
            commit_sha,
            r#ref,
            sarif
        }
    }
}