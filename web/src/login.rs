use db::models::Role;
use leptos::{*, IntoView, component};

#[server(ValidateLogin)]
async fn validate_login() -> Result<Role, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::web::Data;
    use crate::server::WebState;
    todo!()
}

/// Requires the user to be logged in to see the inner view, otherwise
/// redirects user to log in page.
#[component]
pub fn LoggedIn(#[prop(optional)] required_role: Option<Role>, children: Children) -> impl IntoView {
    // Default to requiring Admin rights to see a view, makes sure I don't
    // mess up and expose something I shouldn't through negligence
    let required_role = required_role.unwrap_or(Role::Admin);

    let logged_in = create_resource(|| (), |()| async move {  
        validate_login().await
    });

    todo!()
}

#[server(Login)]
async fn login() -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use actix_web::web::Data;
    use crate::server::WebState;
    todo!()
}

#[component]
pub fn Login() -> impl IntoView {
    todo!()
}
