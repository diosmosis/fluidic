use jira_openapi_client::apis::board_api::{get_all_boards, get_all_sprints};
use jira_openapi_client::apis::configuration::Configuration;
use crate::stores::sprints::SprintsStore;
use crate::sync::Error;
use crate::sync::project::get_project_board;

pub async fn sync_sprints(
    store: &mut SprintsStore,
    api_config: &Configuration,
    project_key_or_id: &str
) -> Result<(), Error> {
    let board_id = get_project_board(&api_config, project_key_or_id).await?;

    // get sprints for board
    let sprints = get_all_sprints(&api_config, board_id, None, None, None)
        .await
        .map_err(|e| Error::ApiError(format!("could not get sprints for board: {}", e.to_string())))?
        .values
        .unwrap_or_default();

    store.set_sprints(&sprints);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use dotenv;
    use crate::jira::new_client;

    #[tokio::test]
    async fn test_sync_sprints() -> Result<(), Error> {
        dotenv::dotenv().ok();

        let api_key = dotenv::var("JIRA_API_KEY").unwrap();
        let test_user = dotenv::var("JIRA_USER").unwrap();
        let test_project = dotenv::var("JIRA_TEST_PROJECT_ID").unwrap();
        let test_base_uri = dotenv::var("JIRA_TEST_BASE_URI").unwrap();

        let api = new_client(test_base_uri, test_user, api_key).unwrap();

        let mut store = SprintsStore::default();

        assert_eq!(store.sprints().len(), 0);

        let _ = sync_sprints(&mut store, &api, test_project.as_str()).await?;

        assert!(store.sprints().len() > 0);

        Ok(())
    }
}
