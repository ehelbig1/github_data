use chrono::{self, prelude::*};
use serde::Deserialize;
use url;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub url: url::Url,
    #[serde(rename = "assets_url")]
    pub assets_url: url::Url,
    #[serde(rename = "upload_url")]
    pub upload_url: url::Url,
    #[serde(rename = "html_url")]
    pub html_url: url::Url,
    pub id: usize,
    pub author: User,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(rename = "published_at")]
    pub published_at: chrono::DateTime<Utc>,
    pub assets: Vec<Asset>,
    #[serde(rename = "tarball_url")]
    pub tarball_url: url::Url,
    #[serde(rename = "zipball_url")]
    pub zipball_url: url::Url,
    pub body: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub login: String,
    pub id: usize,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: url::Url,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: url::Url,
    #[serde(rename = "html_url")]
    pub html_url: url::Url,
    #[serde(rename = "followers_url")]
    pub followers_url: url::Url,
    #[serde(rename = "following_url")]
    pub following_url: url::Url,
    #[serde(rename = "gists_url")]
    pub gists_url: url::Url,
    #[serde(rename = "starred_url")]
    pub starred_url: url::Url,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: url::Url,
    #[serde(rename = "organizations_url")]
    pub organizations_url: url::Url,
    #[serde(rename = "repos_url")]
    pub repos_url: url::Url,
    #[serde(rename = "events_url")]
    pub events_url: url::Url,
    #[serde(rename = "received_events_url")]
    pub received_events_url: url::Url,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub url: url::Url,
    pub id: usize,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub uploader: User,
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub state: String,
    pub size: usize,
    #[serde(rename = "download_count")]
    pub download_count: usize,
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<Utc>,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: url::Url,
}
