use jira_openapi_client::apis::configuration::Configuration;
use crate::jira::Error;
use crate::stores::issues::IssuesStore;

pub async fn sync_issues(issues: &mut IssuesStore, api_config: &Configuration) -> Result<(), Error> {
    todo!()
}
