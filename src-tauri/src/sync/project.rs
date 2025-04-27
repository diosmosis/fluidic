use jira_openapi_client::apis::board_api::get_all_boards;
use jira_openapi_client::apis::configuration::Configuration;
use crate::sync::Error;

// TODO: cache board_id
pub async fn get_project_board(
    api_config: &Configuration,
    project_key_or_id: &str
) -> Result<i64, Error> {
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

    Ok(board_id)
}