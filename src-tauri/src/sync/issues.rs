use jira_openapi_client::apis::board_api::get_issues_for_board;
use jira_openapi_client::apis::configuration::Configuration;
use jira_openapi_client::models::SearchResultsBean;
use crate::stores::issues::IssuesStore;
use crate::sync::Error;
use crate::sync::paging::paged_for_all;
use crate::sync::project::get_project_board;

// TODO: need to add exponential back off (https://docs.rs/reqwest-retry/latest/reqwest_retry/index.html)

pub async fn sync_issues(
    store: &mut IssuesStore,
    api_config: &Configuration,
    project_key_or_id: &str
) -> Result<(), Error> {
    let board_id = get_project_board(&api_config, project_key_or_id).await?;

    // get issues for board
    let mut issues = paged_for_all::<SearchResultsBean>(
        500,
        async |start_at: usize, page_size: usize| {
            get_issues_for_board(&api_config, board_id, None, None, Some(page_size as i32), None, None, Some(start_at as i64))
                .await
                .map_err(|e| Error::ApiError(
                    format!("failed to get issues for board: {}", e.to_string())
                ))
        }).await?;

    store.move_issues_from(&mut issues);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use dotenv;
    use crate::jira::new_client;

    #[tokio::test]
    async fn test_sync_issues() -> Result<(), Error> {
        dotenv::dotenv().ok();

        let api_key = dotenv::var("JIRA_API_KEY").unwrap();
        let test_user = dotenv::var("JIRA_USER").unwrap();
        let test_project = dotenv::var("JIRA_TEST_PROJECT_ID").unwrap();
        let test_base_uri = dotenv::var("JIRA_TEST_BASE_URI").unwrap();

        let api = new_client(test_base_uri, test_user, api_key).unwrap();

        let mut store = IssuesStore::default();

        assert_eq!(store.issues().len(), 0);

        let _ = sync_issues(&mut store, &api, test_project.as_str()).await?;

        assert!(store.issues().len() > 600);

        Ok(())
    }
}
