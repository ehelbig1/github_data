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
    ) -> Result<model::release::Release, error::Error>;
}

pub struct GithubDatasource<'a> {
    http_client: &'a reqwest::Client,
    base_url: url::Url,
}

impl<'a> GithubDatasource<'a> {
    pub fn new(http_client: &'a reqwest::Client) -> Self {
        let base_url = url::Url::parse("https://api.github.com").unwrap();

        Self {
            http_client,
            base_url,
        }
    }
}

#[async_trait]
impl<'a> Datasource for GithubDatasource<'a> {
    async fn list_latest_release(
        &self,
        organization: &str,
        repository: &str,
    ) -> Result<model::release::Release, error::Error> {
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
            Ok(response) => response.json::<model::release::Release>().await,
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError),
        }
    }
}
