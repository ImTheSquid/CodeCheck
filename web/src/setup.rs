use leptos::*;
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
            <AuthSetupInner/>
        </div>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "basic_auth")] {
        #[component]
        fn AuthSetupInner() -> impl IntoView {
            view! {
                <p>"basic auth"</p>
            }
        }
    } else {
        #[component]
        fn AuthSetupInner() -> impl IntoView {
            view! {
                <p style="color: red;">"ERROR! No authentication backend selected."</p>
            }
        }
    }
}