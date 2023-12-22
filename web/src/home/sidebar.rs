use leptos::{*, html::Dialog};
use leptos_meta::*;
use leptos_router::*;
use stylist::style;

#[component]
pub fn OutletSidebar() -> impl IntoView {
    view! {
        <div id="sidebar">
            <CourseSidebar/>
        </div>

        <div id="content">
            <Outlet/>
        </div>
    }
}

#[server(GetCourses)]
async fn get_courses() -> Result<(), ServerFnError> {
    todo!()
}

#[component]
pub fn CourseSidebar() -> impl IntoView {
    let courses = create_blocking_resource(|| (), |()| async move { get_courses().await });

    let styles = style!(
        .sidebar {
            top: 0;
            left: -15 em;
            width: 15 em;
        }
    );

    styled::view! { styles, 
        <div class="sidebar">
            <p>"Test"</p>
        </div>
        <div class="content">
            <Outlet/>
        </div>
    }
}

#[component]
fn SectionSidebar() -> impl IntoView {
    let styles = style!();

    styled::view! { styles,

    }
}
