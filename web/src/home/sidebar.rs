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
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::HttpRequest;
    use actix_web::web::Data;
    use crate::server::WebState;
    use goldleaf::AutoCollection;
    use db::models::User;
    use mongodb::bson::doc;
    use crate::AuthedUser;

    extract(|data: Data<WebState>, _user: AuthedUser<{db::Role::Assistant}>| async move {
        todo!()
    }).await?
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
