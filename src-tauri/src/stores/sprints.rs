use crate::stores::signal::Signal;
use jira_openapi_client::models::SprintBean;

struct SprintsStore {
    sprints: Signal<Vec<SprintBean>>,
}

impl SprintsStore {
    fn default() -> Self {
        Self {
            sprints: Signal::new(Vec::<SprintBean>::default()),
        }
    }
}
