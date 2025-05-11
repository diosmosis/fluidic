use leptos::prelude::*;
use crate::components::*;

#[component]
pub fn IssueManager() -> impl IntoView {
    view! {
        <div>
            <IssueCategories>
            </IssueCategories>

            <SearchInput>
            </SearchInput>

            <IssueList>
            </IssueList>
        </div>
    }
}
