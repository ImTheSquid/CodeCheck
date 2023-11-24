use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::login::LoggedIn;

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <LoggedIn>
            <p>"protected endpoint"</p>
        </LoggedIn>
    }
}
