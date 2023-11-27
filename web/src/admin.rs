use auth::ValidatedUser;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::login::LoggedIn;

#[component]
pub fn Admin() -> impl IntoView {
    let ctx = expect_context::<ValidatedUser>();

    view! {
        <p>"protected endpoint! User accessed with ID " {ctx.id.to_hex()}</p>
    }
}
