use chrono::{self, prelude::*};
use serde::Deserialize;
use url;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Release {
    pub url: url::Url,
    pub assets_url: url::Url,
    pub upload_url: url::Url,
    pub html_url: url::Url,
    pub id: usize,
    pub author: User,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub published_at: chrono::DateTime<Utc>,
    pub assets: Vec<Asset>,
    pub tarball_url: url::Url,
    pub zipball_url: url::Url,
    pub body: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct User {
    pub login: String,
    pub id: usize,
    pub node_id: String,
    pub avatar_url: url::Url,
    pub gravatar_id: String,
    pub url: url::Url,
    pub html_url: url::Url,
    pub followers_url: url::Url,
    pub following_url: url::Url,
    pub gists_url: url::Url,
    pub starred_url: url::Url,
    pub subscriptions_url: url::Url,
    pub organizations_url: url::Url,
    pub repos_url: url::Url,
    pub events_url: url::Url,
    pub received_events_url: url::Url,
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Asset {
    pub url: url::Url,
    pub id: usize,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub uploader: User,
    pub content_type: String,
    pub state: String,
    pub size: usize,
    pub download_count: usize,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub browser_download_url: url::Url,
}
