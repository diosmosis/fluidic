use crate::stores::signal::Signal;
use jira_openapi_client::models::IssueBean;

struct IssuesStore {
    issues: Signal<Vec<IssueBean>>,
}

impl IssuesStore {
    fn default() -> Self {
        Self {
            issues: Signal::new(Vec::<IssueBean>::default()),
        }
    }
}
