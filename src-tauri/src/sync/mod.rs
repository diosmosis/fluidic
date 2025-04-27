mod sprints;
mod issues;
mod project;
mod paging;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to synchronize sprints, {}", .0)]
    ApiError(String),

    #[error("No boards found in this project.")]
    NoBoardsForProject,

    #[error("Invalid JIRA API response encountered: {}", .0)]
    InvalidApiResponse(String),
}
