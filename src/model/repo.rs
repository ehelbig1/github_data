use chrono::{self, prelude::*};
use serde::Deserialize;
use url;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Repo {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    pub html_url: url::Url,
    pub description: String,
    pub fork: bool,
    pub url: url::Url,
    pub archive_url: url::Url,
    pub assignees_url: url::Url,
    pub blobs_url: url::Url,
    pub branches_url: url::Url,
    pub collaborators_url: url::Url,
    pub comments_url: url::Url,
    pub commits_url: url::Url,
    pub compare_url: url::Url,
    pub contents_url: url::Url,
    pub contributors_url: url::Url,
    pub deployments_url: url::Url,
    pub downloads_url: url::Url,
    pub events_url: url::Url,
    pub forks_url: url::Url,
    pub git_commits_url: url::Url,
    pub git_refs_url: url::Url,
    pub git_tags_url: url::Url,
    pub git_url: url::Url,
    pub issue_comment_url: url::Url,
    pub issue_events_url: url::Url,
    pub issues_url: url::Url,
    pub keys_url: url::Url,
    pub labels_url: url::Url,
    pub languages_url: url::Url,
    pub merges_url: url::Url,
    pub milestones_url: url::Url,
    pub notifications_url: url::Url,
    pub pulls_url: url::Url,
    pub releases_url: url::Url,
    pub ssh_url: url::Url,
    pub stargazers_url: url::Url,
    pub statuses_url: url::Url,
    pub subscribers_url: url::Url,
    pub subscription_url: url::Url,
    pub tags_url: url::Url,
    pub teams_url: url::Url,
    pub trees_url: url::Url,
    pub clone_url: url::Url,
    pub mirror_url: url::Url,
    pub hooks_url: url::Url,
    pub svn_url: url::Url,
    pub homepage: url::Url,
    pub language: Option<String>,
    pub forks_count: u32,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub size: u32,
    pub default_branch: String,
    pub open_issues_count: u32,
    pub is_template: bool,
    pub topics: Vec<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub has_discussions: bool,
    pub archived: bool,
    pub disabled: bool,
    pub visibility: String,
    pub pushed_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub permissions: Permissions,
    pub security_and_analysis: SecurityAndAnalysis,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Owner {
    pub login: String,
    pub id: u32,
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
pub struct Permissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SecurityAndAnalysis {
    pub advanced_security: AdvancedSecurity,
    pub secret_scanning: SecretScanning,
    pub secret_scanning_push_protection: SecretScanningPushProtection,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AdvancedSecurity {
    pub status: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SecretScanning {
    pub status: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SecretScanningPushProtection {
    pub status: String,
}