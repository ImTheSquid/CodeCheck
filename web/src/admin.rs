use auth::ValidatedUser;
use db::Role;
use futures_util::TryStreamExt;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <h1>"CodeCheck Admin Interface"</h1>
        <Outlet/>
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DisplayUser {
    id: String,
    username: String,
    name: String,
    role: Role,
    email: Option<String>,
    email_verified: bool,
}

#[server(GetAllUsers)]
async fn get_all_users() -> Result<Vec<DisplayUser>, ServerFnError> {
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::HttpRequest;
    use actix_web::web::Data;
    use crate::server::WebState;
    use goldleaf::AutoCollection;
    use db::models::User;
    use mongodb::bson::doc;

    extract(|data: Data<WebState>| async move {
        let all_users = data.database.auto_collection::<User>().find(doc!{}, None).await?.try_collect::<Vec<_>>().await?;

        let all_users = all_users.into_iter().map(|u| DisplayUser {
            id: u.id.unwrap().to_hex(),
            username: u.username,
            name: u.name,
            role: u.role,
            email: u.email.map(|vr| vr.resource),
            email_verified: u.email.is_some_and(|vr| matches!(vr.status, db::models::VerificationStatus::Verified)),
        }).collect();
        
        Ok(all_users)
    }).await?
}

#[component]
pub fn Users() -> impl IntoView {
    view! {
        <h2>"User Management"</h2>
    }
}

#[component]
fn UserListRowItem() -> impl IntoView {
    todo!()
}
