use async_trait::async_trait;
use reqwest;
use url;

pub mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async fn list_latest_release(
        &self,
        organization: &str,
        repository: &str,
    ) -> Result<model::list_release::Release, error::Error>;

    async fn list_org_repositories(&self, properties: model::list_repositories::Properties) -> Result<model::list_repositories::Response, error::Error>;
}

pub struct GithubDatasource<'a> {
    http_client: &'a reqwest::Client,
    base_url: url::Url,
    personal_access_token: Option<String>
}

impl<'a> GithubDatasource<'a> {
    pub fn new(http_client: &'a reqwest::Client, personal_access_token: Option<String>) -> Self {
        let base_url = url::Url::parse("https://api.github.com").unwrap();

        Self {
            http_client,
            base_url,
            personal_access_token
        }
    }
}

#[async_trait]
impl<'a> Datasource for GithubDatasource<'a> {
    async fn list_latest_release(
        &self,
        organization: &str,
        repository: &str,
    ) -> Result<model::list_release::Release, error::Error> {
        let url = format!(
            "{}repos/{}/{}/releases/latest",
            self.base_url.as_str(),
            organization,
            repository
        );
        let response = self
            .http_client
            .get(url)
            .header("User-Agent", "github_data")
            .send()
            .await;

        let data = match response {
            Ok(response) => response.json::<model::list_release::Release>().await,
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError),
        }
    }

    async fn list_org_repositories(&self, properties: model::list_repositories::Properties) -> Result<model::list_repositories::Response, error::Error> {
        let url = format!("{}orgs/{}/repos", self.base_url.as_str(), properties.organization);
        let mut request = self
            .http_client
            .get(url)
            .header("User-Agent", "github_data")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("Accept", "application/vnd.github+json")
            .query(&[("type", "all"), ("per_page", "100"), ("page", &properties.page.to_string())]);

        request = if let Some(personal_access_token) = &self.personal_access_token {
            request
                .header("Authorization", format!("Bearer {}", personal_access_token))
        } else {
            request
        };

        let response = request
            .send()
            .await;

        let data = match response {
            Ok(response) => response.json::<model::list_repositories::Response>().await,
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError),
        }
    }
}
