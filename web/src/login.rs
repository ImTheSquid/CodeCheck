use auth::ValidatedUser;
use leptos::{*, IntoView, component};
use db::Role;
use leptos::*;

#[server(ValidateLogin)]
async fn validate_login(required_role: Role) -> Result<ValidatedUser, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::web::Data;
    use crate::server::WebState;
    
    extract(|data: Data<WebState>| async move {
        todo!()
    }).await?
}

/// Requires the user to be logged in to see the inner view, otherwise
/// redirects user to log in page.
#[component]
pub fn LoggedIn(#[prop(optional)] required_role: Option<Role>, children: ChildrenFn) -> impl IntoView {
    // Default to requiring Admin rights to see a view, makes sure I don't
    // mess up and expose something I shouldn't through negligence
    // let required_role = required_role.unwrap_or(Role::Admin);

    let logged_in = create_resource(move || required_role.unwrap_or(Role::Admin), |role| async move {  
        validate_login(role).await
    });

    view! {
        <Transition fallback=move || view! { <p>"Validating session data..."</p> }>
            {move || {
                provide_context(logged_in.get().unwrap());

                view! { children.with_value(|c| c()) }
            }}
        </Transition>
    }
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
