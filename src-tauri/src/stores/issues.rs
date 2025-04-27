use std::sync::Arc;
use crate::stores::signal::Signal;
use jira_openapi_client::models::IssueBean;

pub struct IssuesStore {
    pub issues: Signal<Vec<Arc<IssueBean>>>,
}

impl IssuesStore {
    fn default() -> Self {
        Self {
            issues: Signal::new(Vec::<Arc<IssueBean>>::default()),
        }
    }
}
