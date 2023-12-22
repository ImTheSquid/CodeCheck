use leptos::{*, html::Dialog};
use leptos_meta::*;
use leptos_router::*;
use styled::style;
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
