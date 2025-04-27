use jira_openapi_client::apis::board_api::{get_all_boards, get_all_sprints};
use jira_openapi_client::apis::configuration::Configuration;
use crate::stores::sprints::SprintsStore;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to synchronize sprints, {}", .0)]
    ApiError(String),

    #[error("No boards found in this project.")]
    NoBoardsForProject,

    #[error("Invalid JIRA API response encountered: {}", .0)]
    InvalidApiResponse(String),
}

pub async fn sync_sprints(
    store: &mut SprintsStore,
    api_config: &Configuration,
    project_key_or_id: &str
) -> Result<(), Error> {
    // get the boards for the project
    let boards =
        get_all_boards(&api_config, None, None, Some(project_key_or_id), None, None)
        .await
        .map_err(|e| Error::ApiError(format!("could not get all boards: {}", e.to_string())))?
        .values
        .take_if(|v| !v.is_empty())
        .ok_or(Error::NoBoardsForProject)?;

    // pick first board
    let board = boards.iter()
        .filter(
            |v| v.location.as_ref().is_some_and(|l| l.project_key == Some(project_key_or_id.to_string()))
        ).take(1)
        .next()
        .ok_or(Error::NoBoardsForProject)?;

    let board_id = board.id.ok_or(Error::InvalidApiResponse("missing board ID".into()))?;

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
