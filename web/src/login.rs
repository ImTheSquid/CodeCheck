use auth::ValidatedUser;
use leptos::{*, IntoView, component};
use db::Role;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server(ValidateLogin)]
async fn validate_login(required_role: Role) -> Result<ValidatedUser, ServerFnError> {
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::HttpRequest;
    use actix_web::web::Data;
    use crate::server::WebState;

    let response = expect_context::<ResponseOptions>();
 
    extract(move |data: Data<WebState>, req: HttpRequest| async move {
        let cookie = match req.cookie("session") {
            None => {
                leptos_actix::redirect(&format!("/login?next={}", urlencoding::encode(req.path())));
                return Err(ServerFnError::ServerError("No cookie present!".to_string()));
            },
            Some(cookie) => cookie,
        };

        let res = match auth::validate_token(&data.database, cookie.value()).await.map_err(|e| ServerFnError::ServerError(e.to_string())) {
            Err(e) => {
                leptos_actix::redirect(&format!("/login?next={}", urlencoding::encode(req.path())));
                return Err(ServerFnError::ServerError(e.to_string()));
            },
            Ok(res) => res,
        };
        
        if res.role < required_role {
            response.set_status(actix_web::http::StatusCode::FORBIDDEN);
            return Err(ServerFnError::ServerError("Forbidden".to_string()));
        }

        Ok(res)
    }).await?
}

/// Requires the user to be logged in to see the inner view, otherwise
/// redirects user to log in page.
#[component]
pub fn LoggedIn(#[prop(optional)] required_role: Option<Role>, children: ChildrenFn) -> impl IntoView {
    // Default to requiring Admin rights to see a view, makes sure I don't
    // mess up and expose something I shouldn't through negligence
    let required_role = required_role.unwrap_or(Role::Admin);

    let logged_in = create_blocking_resource(move || required_role, |role| async move {  
        validate_login(role).await
    });

    let children = store_value(children);

    view! {
        <Transition fallback=move || view! { <p>"Validating session data..."</p> }>
            {move || {
                if let Some(li) = logged_in.get() {
                    match li {
                        Ok(li) => {
                            provide_context(li);
                            children.with_value(|c| c()).into_view()
                        },
                        Err(e) => {
                            view! {
                                <p style="color: red;">{e.to_string()}</p>
                            }.into_view()
                        }
                    }
                } else {
                    ().into_view()
                }
                
            }}
        </Transition>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "basic_auth")] {
        #[server(Login)]
        async fn login(username: String, password: String, next: String) -> Result<(), ServerFnError> {
            use leptos_actix::{extract, ResponseOptions};
            use actix_web::{web::Data, cookie::Cookie};
            use actix_web::cookie::time::OffsetDateTime;
            use actix_web::http::header::HeaderValue;
            use crate::server::WebState;
            use actix_web::HttpRequest;
            
            let response = expect_context::<ResponseOptions>();

            extract(|data: Data<WebState>| async move {
                let token = auth::basic::authenticate(&data.database, &username, &password).await.map_err(|e| ServerFnError::ServerError(e.to_string()))?;

                let cookie = Cookie::build("session", &token).expires(Some(OffsetDateTime::now_utc() + auth::TOKEN_LIFETIME)).path("/").finish();
                response.insert_header(actix_web::http::header::SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).expect("Failed to write cookie"));
                leptos_actix::redirect(&next);
                Ok(())
            }).await?
        }

        #[component]
        pub fn Login() -> impl IntoView {
            let login_action = create_server_action::<Login>();

            #[derive(Debug, Params, PartialEq, Clone)]
            struct LoginQuery {
                next: Option<String>
            }
            
            let next = use_query::<LoginQuery>().get().unwrap().next.unwrap_or("/".to_string());

            view! {
                <div>
                    <h1>"CodeCheck Login"</h1>
                    <ActionForm action=login_action class="vertical spaced_children">
                        <label>
                            "Username"
                            <input type="text" name="username" required/>
                        </label>
                        <label>
                            "Password"
                            <input type="password" name="password" required/>
                        </label>
                        <input type="text" hidden name="next" value=next/>
                        <button type="submit">"Submit"</button>
                    </ActionForm>
                </div>
            }
        }
    } else {
        #[component]
        pub fn Login() -> impl IntoView {
            view! { <p style="color: red;">"No auth backend selected!"</p> }
        }
    }
}
