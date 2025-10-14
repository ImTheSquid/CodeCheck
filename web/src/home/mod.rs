use crate::{HumanReadableUser, RoleRequirement};
use leptos::{html::Dialog, *};
use leptos_meta::*;
use leptos_router::*;
use styled::style;
pub mod sidebar;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>"Test"<Outlet/></div>
    }
}

#[component]
pub fn UserManagementList(
    users: RwSignal<Vec<HumanReadableUser>>,
    role_requirement: RoleRequirement,
) -> impl IntoView {
    view! {}
}
