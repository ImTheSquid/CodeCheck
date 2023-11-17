use leptos::{*, html::Button};
use leptos_meta::*;
use leptos_router::*;

#[server(CheckSetupStatus)]
async fn check_setup_status() -> Result<bool, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::{Data, Query};
    use crate::server::WebState;

    extract(|data: Data<WebState>| async move {
        data.config.read().expect("Failed to get read lock for config!").setup_complete
    }).await
}

#[component]
pub fn Setup() -> impl IntoView {
    let setup_is_complete = create_resource(|| (), |_| async move {
        check_setup_status().await
    });

    #[cfg(feature = "ssr")]
    {
        if setup_is_complete.with(|sic| sic.as_ref().is_some_and(|val| val.as_ref().is_ok_and(|r| *r))) {
            // this can be done inline because it's synchronous
            // if it were async, we'd use a server function
            let resp = expect_context::<leptos_actix::ResponseOptions>();
            resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
        }
    }

    view! {
        <div>
            <h1>"CodeCheck Setup"</h1>
            <AuthSetup/>
        </div>
    }
}

#[component]
fn AuthSetup() -> impl IntoView {
    view! {
        <div>
            <h2>"Authentication Setup"</h2>
            <AuthSetupInner on_complete=move |_| {}/>
        </div>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "basic_auth")] {
        #[server(AuthenticationSetup)]
        async fn setup_authentication(username: String, password: String) -> Result<Result<(), String>, ServerFnError> {
            use leptos_actix::extract;
            use actix_web::web::Data;
            use crate::server::WebState;

            extract(|data: Data<WebState>| async move {
                todo!()
            }).await
        }

        #[component]
        fn AuthSetupInner(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
            let setup_authentication = create_server_action::<AuthenticationSetup>();

            let (username, set_username) = create_signal("".to_string());
            let (password, set_password) = create_signal("".to_string());
            let button = create_node_ref::<Button>();

            create_effect(move |_| {
                button.get().unwrap().set_disabled(username.with(|u| u.is_empty()) || password.with(|p| p.is_empty()));
            });

            view! {
                <ActionForm action=setup_authentication class="vertical spaced_children">
                    <label>
                        "Enter Root Username"
                        <input type="text" name="username" prop:value=username on:input=move |ev| {
                            set_username(event_target_value(&ev));
                        }/>
                    </label>
                    <label>
                        "Enter Root Password"
                        <input type="password" name="password" prop:value=password on:input=move |ev| {
                            set_password(event_target_value(&ev));   
                        }/>
                    </label>
                    <button type="submit" disabled node_ref=button>"Submit"</button>
                </ActionForm>
            }
        }
    } else {
        #[component]
        fn AuthSetupInner(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
            view! {
                <p style="color: red;">"ERROR! No authentication backend selected."</p>
            }
        }
    }
}