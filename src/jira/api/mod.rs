use ouroboros::self_referencing;
use crate::jira::client::Client;
use sprint::Api as SprintApi;

pub mod sprint;

#[self_referencing]
pub struct Api {
    client: Client,

    #[borrows(client)]
    #[covariant]
    streams: SprintApi<'this>,
}

impl Api {
    fn build(api_key: String) -> Self {
        ApiBuilder {
            client: Client::new(api_key),
            streams_builder: |c| SprintApi::new(c),
        }.build()
    }
}
