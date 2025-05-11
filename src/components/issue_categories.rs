use leptos::prelude::*;

#[component]
pub fn IssueCategories() -> impl IntoView {
    view! {
        <div class="is-flex is-flex-direction-row is-align-content-center m-4">
            <button class="button mr-4 is-primary is-dark">Current</button>
            <button class="button mr-4 is-primary">Backlog</button>
            <button class="button is-primary">All</button>
        </div>
    }
}
