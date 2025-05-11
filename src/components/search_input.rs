use leptos::prelude::*;

#[component]
pub fn SearchInput() -> impl IntoView {
    view! {
        <div class="m-4 control has-icons-right">
            <input class="input" placeholder="Type to search..." />
            <span class="icon is-small is-right">
              <i class="fas fa-magnifying-glass"></i>
            </span>
        </div>
    }
}
