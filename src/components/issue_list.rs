use leptos::prelude::*;
use jira_openapi_client::models::IssueBean;
use crate::components::test_data::TEST_ISSUES;
use crate::components::*;

// TODO: need the ability to close an issue

#[component]
pub fn IssueList() -> impl IntoView {
    view! {
        <div class="is-flex is-flex-direction-column">
            <div style="border-top: 2px solid #444;">
                <IssueRow></IssueRow>
            </div>
            <div style="border-top: 2px solid #444; border-bottom: 2px solid #444;">
                <IssueRow></IssueRow>
            </div>
        </div>
    }
}
