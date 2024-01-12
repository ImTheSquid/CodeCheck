use leptos::{html::Dialog, *};
use leptos_meta::*;
use leptos_router::*;
use styled::style;
use crate::{HumanReadableUser, RoleRequirement};
pub mod sidebar;

#[component]
pub fn Home() -> impl IntoView {
    let styles = style!(
        div {
            background-color: red;
            color: white;
        }
    );

    styled::view! { styles,
        <div>"Test"<Outlet/></div>
    }
}

#[component]
pub fn UserManagementList(users: RwSignal<Vec<HumanReadableUser>>, role_requirement: RoleRequirement) -> impl IntoView {
    view! {

    }
}
