use serde::{Deserialize, Serialize};
use crate::jira::client::Client;

#[derive(Serialize, Deserialize)]
pub struct Sprint {
    // TODO
}

pub(crate) struct Api<'a> {
    client: &'a Client,
}

impl<'a> Api<'a> {
    pub fn new(client: &'a Client) -> Self {
        Api {
            client,
        }
    }

    fn get(&self, sprintId: u64) -> Sprint {
        Sprint {}
    }
}
