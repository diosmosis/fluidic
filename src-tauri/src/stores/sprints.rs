use std::sync::Arc;
use leptos::prelude::{Get, Set};
use crate::stores::signal::Signal;
use jira_openapi_client::models::SprintBean;

pub struct SprintsStore {
    sprints: Signal<Vec<Arc<SprintBean>>>,
}

impl SprintsStore {
    pub fn default() -> Self {
        Self {
            sprints: Signal::new(Vec::<Arc<SprintBean>>::default()),
        }
    }

    pub fn sprints(&self) -> Vec<Arc<SprintBean>> {
        self.sprints.read.get()
    }

    pub fn set_sprints(&mut self, sprints: &Vec<SprintBean>) {
        let arc_sprints = sprints
            .iter()
            .map(|x| Arc::new(x.clone()))
            .collect();

        self.sprints.write.set(arc_sprints);
    }
}
