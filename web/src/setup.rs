use leptos::{*, html::Button};
use leptos_meta::*;
use leptos_router::*;

#[server(CheckSetupStatus)]
async fn check_setup_status() -> Result<bool, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::web::Data;
    use crate::server::WebState;

    extract(|data: Data<WebState>| async move {
        data.config.read().expect("Failed to get read lock for config!").setup_complete
    }).await
}

#[derive(Debug)]
enum SetupStage {
    Authentication,
    Files,
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

    let (stage, set_stage) = create_signal(SetupStage::Authentication);

    view! {
        <div>
            <h1>"CodeCheck Setup"</h1>
            {move || stage.with(|stage| match stage {
                    SetupStage::Authentication => 
                        view! { <AuthSetup on_complete=move |_| { set_stage(SetupStage::Files) }/> },
                    SetupStage::Files =>
                        view! { <FilesSetup on_complete=move |_| { use_navigate()("/login?next=%2Fadmin", Default::default()) }/> },
                })
            }
        </div>
    }
}

#[server(FilesSetup)]
async fn setup_files(vocareum_api_key: String) -> Result<(), ServerFnError> {
    todo!()
}

#[component]
fn FilesSetup(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
    let setup_files_action = create_server_action::<FilesSetup>();

    create_effect(move |_| {
        if setup_files_action.value().with(|value| matches!(value, Some(Ok(())))) {
            on_complete(());
        }
    });

    view! {
        <div>
            <h2>"Files Setup"</h2>
            <p>"Files are stored at "<span class="code">{storage::MAIN_STORAGE_DIRECTORY}</span>". Connect an external volume to mount this directory in your real file system if necessary. CodeCheck will automatically create directories as needed in this directory " <strong>"WITHOUT CHECKING IF FILES ALREADY EXIST!"</strong>" Make sure the directory is clear!"</p>
            <ActionForm action=setup_files_action class="vertical spaced_children">
                <label>
                    "Vocareum API key (optional)"
                    <input type="text" name="vocareum_api_key"/>
                </label>

                <button type="submit">"Submit"</button>
            </ActionForm>
        </div>
    }
}

#[component]
fn AuthSetup(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
    view! {
        <div>
            <h2>"Authentication Setup"</h2>
            <AuthSetupInner on_complete=on_complete/>
        </div>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "basic_auth")] {
        #[server(AuthenticationSetup)]
        async fn setup_authentication(username: String, password: String) -> Result<(), ServerFnError> {
            use leptos_actix::extract;
            use actix_web::web::Data;
            use crate::server::WebState;
            use db::models::User;
            use goldleaf::AutoCollection;
            use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

            extract(|data: Data<WebState>| async move {
                let salt = SaltString::generate(&mut OsRng);
                let password = Argon2::default().hash_password(password.as_bytes(), &salt).map_err(|e| ServerFnError::ServerError(e.to_string()))?.to_string();

                data.database.auto_collection::<User>().insert_one(User {
                    id: None,
                    username,
                    role: db::models::Role::Admin,
                    password,
                    salt: salt.to_string(),
                    sessions: Vec::new(),
                }, None).await?;

                Ok(())
            }).await?
        }

        #[component]
        fn AuthSetupInner(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
            let auth_action = create_server_action::<AuthenticationSetup>();

            let (username, set_username) = create_signal("".to_string());
            let (password, set_password) = create_signal("".to_string());
            let button = create_node_ref::<Button>();

            create_effect(move |_| {
                button.get().unwrap().set_disabled(username.with(|u| u.is_empty()) || password.with(|p| p.is_empty()));
            });

            create_effect(move |_| {
                if auth_action.value().with(|value| value.is_some()) {
                    let val = auth_action.value().get().unwrap();
                    match val {
                        Ok(_) => on_complete(()),
                        Err(e) => println!("Leptos: {}", e),
                    }
                }
            });

            view! {
                <ActionForm action=auth_action class="vertical spaced_children">
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
