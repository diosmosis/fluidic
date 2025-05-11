use leptos::prelude::*;
use jira_openapi_client::models::IssueBean;

// TODO: handle grouping of subtasks
// TODO: the current filter should show all done items in the current sprint (crossed out)

#[component]
pub fn IssueRow() -> impl IntoView {
    view! {
        <div class="is-flex is-flex-direction-row is-align-items-center pl-4 pr-4 pt-3 pb-3">
            <div class="mr-4" style="border-radius: 4px; background-color: skyblue; color: #666; padding: 2px 4px;">MWP</div>
            <div class="is-flex-grow-1">
                <a href="#">scrape builtwith.com to get better usage analytics per country</a>
            </div>
            <div>
                <button class="button is-info is-small is-dark">
                    <span class="icon is-small is-right">
                      <i class="fas fa-list"></i>
                    </span>
                </button>

                <button class="button is-info is-small ml-2 is-dark">
                    <span class="icon is-small is-right">
                      <i class="fas fa-plus"></i>
                    </span>
                </button>

                <button class="button is-info is-small ml-2 is-dark">
                    <span class="icon is-small is-right">
                      <i class="fas fa-check"></i>
                    </span>
                </button>
            </div>
        </div>
    }
}
