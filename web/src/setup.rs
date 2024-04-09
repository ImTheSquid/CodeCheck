use leptos::{html::Button, *};
use leptos_meta::*;
use leptos_router::*;

#[server(CheckSetupStatus)]
async fn check_setup_status() -> Result<bool, ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let data: Data<WebState> = extract().await?;

    let complete = data.config
        .read()
        .expect("Failed to get read lock for config!")
        .setup_complete;

    Ok(complete)
}

#[server(CompleteSetup)]
async fn complete_setup() -> Result<(), ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let data: Data<WebState> = extract().await?;

    let mut write = data
        .config
        .write()
        .expect("Failed to get write lock for config!");
    write.setup_complete = true;
    write
        .write().expect("config write to succeed");
    Ok(())
}

#[derive(Debug)]
enum SetupStage {
    Authentication,
    Files,
}

#[component]
pub fn Setup() -> impl IntoView {
    let setup_is_complete = create_resource(|| (), |_| async move { check_setup_status().await });

    let (stage, set_stage) = create_signal(SetupStage::Authentication);

    view! {
            <Suspense fallback=move || view!{<p>"Verifying setup availability..."</p>}>
            {move || if setup_is_complete.with(|sic| sic.as_ref().is_some_and(|val| val.as_ref().is_ok_and(|r| *r)))  {
                    view!{<div><p style="color: red;">"Setup has already been completed!"</p></div>}
                } else {
                    view!{
    <div>
                <h1>"CodeCheck Setup"</h1>
                {move || stage.with(|stage| match stage {
                        SetupStage::Authentication =>
                            view! { <AuthSetup on_complete=move |_| { set_stage(SetupStage::Files) }/> },
                        SetupStage::Files =>
                            view! { <FilesSetup on_complete=move |_| {
                                create_resource(||(), |_| async move {
                                    complete_setup().await
                                });
                                use_navigate()("/login?next=%2Fadmin", Default::default())
                            }/> },
                    })
                }
            </div>
                    }
                }
            }
            </Suspense>
        }
}

#[server(FilesSetup)]
async fn setup_files(vocareum_api_key: String) -> Result<(), ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let data: Data<WebState> = extract().await?;

    let mut write = data
        .config
        .write()
        .expect("Failed to get write lock for config!");
    write.vocareum_key = if vocareum_api_key.is_empty() {
        None
    } else {
        Some(vocareum_api_key)
    };
    write
        .write().expect("write op to succeed");
    Ok(())
}

#[component]
fn FilesSetup(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
    let setup_files_action = create_server_action::<FilesSetup>();

    create_effect(move |_| {
        if setup_files_action
            .value()
            .with(|value| matches!(value, Some(Ok(()))))
        {
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

            extract(|data: Data<WebState>| async move {
                auth::basic::enroll_root(&data.database, username, password).await.map_err(|e| ServerFnError::ServerError(e.to_string()))
            }).await?
        }

        #[component]
        fn AuthSetupInner(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
            let auth_action = create_server_action::<AuthenticationSetup>();

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
                        <input type="text" name="username" required/>
                    </label>
                    <label>
                        "Enter Root Password"
                        <input type="password" name="password" required/>
                    </label>
                    <button type="submit">"Submit"</button>
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
