use serde::{Deserialize, Serialize};
use crate::jira::client::Client;
use dotenv;

#[derive(Serialize, Deserialize)]
pub struct Sprint {
    #[serde(rename = "activatedDate")]
    activated_date: Option<String>,

    #[serde(rename = "autoStartStop", default)]
    auto_start_stop: bool,

    #[serde(rename = "completeDate")]
    complete_date: Option<String>,

    #[serde(rename = "endDate")]
    end_date: Option<String>,

    goal: String,
    id: i64,

    #[serde(rename = "incompleteIssuesDestinationId")]
    incomplete_issues_destination_id: Option<i64>,

    name: String,

    #[serde(rename = "originBoardId")]
    origin_board_id: i64,

    #[serde(rename = "self")]
    self_uri: String,

    #[serde(rename = "startDate")]
    start_date: Option<String>,

    state: String,

    #[serde(default)]
    synced: bool,
}

pub(crate) struct Api<'a> {
    client: &'a Client,
}

impl<'a> Api<'a> {
    pub fn new(client: &'a Client) -> Self {
        Api {
            client,
        }
    }

    async fn get(&self, sprint_id: i64) -> Result<Sprint, crate::jira::Error> {
        let res = self.client
            .get(format!("/agile/1.0/sprint/{}", sprint_id).as_str())
            .send()
            .await?;

        println!("{}", res.status().as_str());
        let res = res.text().await?;

        // TODO: handle error response in api ({"error": "Failed to parse Connect Session Auth Token"})
        println!("{}", res.as_str());
        serde_json::from_str::<Sprint>(&res)
            .map_err(|e| crate::jira::Error::Json(e))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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

        let client = crate::jira::Api::build(test_base_uri, test_user, api_key);

        let sprint = client.sprints().get(test_sprint_id).await.unwrap();

        assert_eq!(sprint.id, test_sprint_id);
    }
}
