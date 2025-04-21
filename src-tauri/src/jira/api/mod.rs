use ouroboros::self_referencing;
use crate::jira::client::Client;
use sprint::Api as SprintApi;

pub mod sprint;

#[self_referencing]
pub struct Api {
    client: Client,

    #[borrows(client)]
    #[covariant]
    sprints_api: SprintApi<'this>,
}

impl<'a> Api {
    fn build(base_uri: String, user: String, api_key: String) -> Self {
        ApiBuilder {
            client: Client::new(base_uri, user, api_key),
            sprints_api_builder: |c| SprintApi::new(c),
        }.build()
    }

    fn sprints(&'a self) -> &SprintApi<'a> {
        &self.borrow_sprints_api()
    }
}
