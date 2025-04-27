use std::sync::Arc;
use leptos::prelude::{Get, Set};
use crate::stores::signal::Signal;
use jira_openapi_client::models::IssueBean;

pub struct IssuesStore {
    pub issues: Signal<Vec<Arc<IssueBean>>>,
}

impl IssuesStore {
    pub fn default() -> Self {
        Self {
            issues: Signal::new(Vec::<Arc<IssueBean>>::default()),
        }
    }

    pub fn issues(&self) -> Vec<Arc<IssueBean>> {
        self.issues.read.get()
    }

    pub fn move_issues_from(&mut self, issues: &mut Vec<IssueBean>) {
        let mut arc_issues = vec![];
        while let Some(v) = issues.pop() {
            arc_issues.push(Arc::new(v));
        }

        self.issues.write.set(arc_issues);
    }
}
