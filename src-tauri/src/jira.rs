use jira_openapi_client::apis::configuration::Configuration;
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid configuration in {0} property: {1}")]
    InvalidConfiguration(String, String),
}

pub fn new_client(base_uri: String, user: String, api_key: String) -> Result<Configuration, Error> {
    let mut c = Configuration::default();
    c.base_path = Url::parse(&base_uri)
        .map_err(|e| Error::InvalidConfiguration("base_uri".to_string(), e.to_string()))?
        .join("rest")
        .unwrap()
        .to_string();
    c.basic_auth = Some((user, Some(api_key)));
    Ok(c)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use dotenv;
    use jira_openapi_client::apis::sprint_api::get_sprint;

    #[tokio::test]
    async fn test_get() {
        dotenv::dotenv().ok();

        let api_key = dotenv::var("JIRA_API_KEY").unwrap();
        let test_user = dotenv::var("JIRA_USER").unwrap();
        let test_sprint_id = dotenv::var("JIRA_TEST_SPRINT_ID")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let test_base_uri = dotenv::var("JIRA_TEST_BASE_URI").unwrap();

        let api = new_client(test_base_uri, test_user, api_key).unwrap();

        let sprint = get_sprint(&api, test_sprint_id).await.unwrap();

        assert_eq!(sprint.id, Some(test_sprint_id));
    }
}
