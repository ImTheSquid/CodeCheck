use auth::ValidatedUser;
use db::Role;
use futures_util::TryStreamExt;
use leptos::{*, html::Dialog};
use leptos_meta::*;
use leptos_router::*;
use stylist::style;
use std::str::FromStr;

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <h1>"CodeCheck Admin Interface"</h1>
        <Outlet/>
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
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
    use crate::AuthedUser;

    extract(|data: Data<WebState>, _user: AuthedUser<{db::Role::Admin}>| async move {
        let all_users = data.database.auto_collection::<User>().find(doc!{}, None).await?.try_collect::<Vec<_>>().await?;

        let all_users = all_users.into_iter().map(|u| DisplayUser {
            id: u.id.unwrap().to_hex(),
            username: u.username,
            name: u.name,
            role: u.role,
            email: u.email.as_ref().map(|vr| vr.resource.to_owned()),
            email_verified: u.email.is_some_and(|vr| matches!(vr.status, db::models::VerificationStatus::Verified)),
        }).collect();
        
        Ok(all_users)
    }).await?
}

#[server(CreateUser)]
async fn create_user(username: String, name: String, role: String, email: String, email_verified: Option<bool>, password: String) -> Result<(), ServerFnError> {
    let role = db::Role::from_str(&role).unwrap();
    let email = if email.is_empty() {
        None
    } else {
        Some(email)
    };
    let email_verified = email_verified.unwrap_or_default();
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::HttpRequest;
    use actix_web::web::Data;
    use crate::server::WebState;
    use goldleaf::{AutoCollection, CollectionIdentity};
    use db::models::{User, VerifiedResource, VerificationStatus};
    use mongodb::bson::{doc, oid::ObjectId};
    use crate::AuthedUser;
    #[cfg(feature = "basic_auth")]
    use argon2::{PasswordHasher, password_hash::rand_core::OsRng};

    extract(move |data: Data<WebState>, _user: AuthedUser<{db::Role::Admin}>| async move {
        let email = email.map(|e| VerifiedResource {
            resource: e,
            status: if email_verified {
                VerificationStatus::Verified
            } else {
                // todo: send verification email
                todo!()
            }
        });

        #[cfg(feature = "basic_auth")]
        data.database.auto_collection::<User>().insert_one(User {
            id: None,
            username,
            name,
            role,
            sessions: vec![],
            email,
            password: argon2::Argon2::default().hash_password(password.as_bytes(), &argon2::password_hash::SaltString::generate(&mut OsRng)).map_err(|e| ServerFnError::ServerError(e.to_string()))?.to_string(),
        }, None).await?;

        #[cfg(not(feature = "basic_auth"))]
        panic!();

        Ok(())
    }).await?
}

#[component]
pub fn Users() -> impl IntoView {
    let users_res = create_blocking_resource(|| (), |_| async move {
        get_all_users().await
    });

    let new_user_dialog = create_node_ref::<Dialog>();
    let new_user_action = create_server_action::<CreateUser>();

    create_effect(move |prev| {
        let prev = prev.unwrap_or(0);
        let new = new_user_action.version()();
        if prev != new {
            new_user_dialog.get().unwrap().close();
            users_res.refetch();
            new
        } else {
            prev
        }
    });

    let styles = style!(
        table {
            border-collapse: collapse;
            border: 2px solid rgb(200, 200, 200);
            margin-left: auto;
            margin-right: auto;
        }

        td, th {
            padding: 10px;
            border: 2px solid rgb(190, 190, 190);
        }
    );

    styled::view! { styles,
        <h2>"User Management"</h2>
        <Transition fallback=|| view!{ <p>"Loading users..."</p> }>
            <table>
                <tr>
                    <th scope="col">"ID"</th>
                    <th scope="col">"Username"</th>
                    <th scope="col">"Name"</th>
                    <th scope="col">"Role"</th>
                    <th scope="col">"Email"</th>
                    <th scope="col">"Email Verified?"</th>
                    <th scope="col"><button on:click=move |_| new_user_dialog.get().unwrap().show_modal().expect("Failed to open add dialog")>"Add"</button></th>
                </tr>

                {move || {
                    users_res.get().map(|users| {
                        view! {
                            <For each=move|| users.clone().expect("Failed to load users") key=move |user| user.id.clone() children=move |user: DisplayUser| {
                                view! {
                                    <UserListRowItem user=user refresh=move |()| users_res.refetch()/> 
                                }
                            }/>
                        }
                    })
                }}
            </table>

            <dialog node_ref=new_user_dialog>
                <NewUserForm new_user_action=new_user_action/>
                <button on:click=move |_| new_user_dialog.get().unwrap().close()>"Cancel"</button>
            </dialog>
        </Transition>
    }
}

// I only did this because Rust got fussy about the styling macro
// The code looks a bit cleaner now too
use server_fn::ServerFn;
#[component]
fn NewUserForm(new_user_action: Action<CreateUser, Result<<CreateUser as ServerFn<()>>::Output, ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action=new_user_action>
            <label>
                "Username"
                <input type="text" name="username" required/>
            </label>
            <label>
                "Name"
                <input type="text" name="name" required/>
            </label>
            <label>
                "Role"
                <select name="role">
                    <option value="admin">"Admin"</option>
                    <option value="instructor">"Instructor"</option>
                    <option value="assistant" selected>"Assistant"</option>
                </select>
            </label>
            <label>
                "Email"
                <input type="email" name="email"/>
            </label>
            <label>
                "Email Verified"
                <input type="checkbox" name="email_verified" value="true"/>
            </label>
            <label>
                "Password"
                <input type="password" name="password" required/>
            </label>
            <button type="submit">"Submit"</button>
        </ActionForm>
    }
}

#[server(DeleteUser)]
async fn delete_user(user_id: String) -> Result<(), ServerFnError> {
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::HttpRequest;
    use actix_web::web::Data;
    use crate::server::WebState;
    use goldleaf::AutoCollection;
    use db::models::User;
    use mongodb::bson::{doc, oid::ObjectId};
    use crate::AuthedUser;

    extract(move |data: Data<WebState>, _user: AuthedUser<{db::Role::Admin}>| async move {
        let user_id = ObjectId::from_str(&user_id).unwrap();
        let res = data.database.auto_collection::<User>().delete_one(doc! { "_id": user_id }, None).await?;
        if res.deleted_count != 1 {
            Err(ServerFnError::ServerError("Invalid user".to_string()))
        } else {
            Ok(())
        }
    }).await?
}

#[server(UpdateUser)]
async fn update_user(user: DisplayUser) -> Result<(), ServerFnError> {
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::HttpRequest;
    use actix_web::web::Data;
    use crate::server::WebState;
    use goldleaf::{AutoCollection, CollectionIdentity};
    use db::models::{User, VerifiedResource, VerificationStatus};
    use mongodb::bson::{doc, oid::ObjectId};
    use crate::AuthedUser;

    extract(move |data: Data<WebState>, _user: AuthedUser<{db::Role::Admin}>| async move {
        let email = user.email.map(|e| VerifiedResource {
            resource: e,
            status: if user.email_verified {
                VerificationStatus::Verified
            } else {
                // todo: send verification email
                todo!()
            }
        });

        let user_id = ObjectId::parse_str(user.id).unwrap();

        let mut user_db = data.database.auto_collection::<User>().find_one(doc! { "_id": user_id }, None).await.map_err(|e| ServerFnError::ServerError(e.to_string()))?.unwrap();

        user_db.username = user.username;
        user_db.name = user.name;
        user_db.role = user.role;
        user_db.email = email;

        user_db.save(&data.database).await.map_err(|e| ServerFnError::ServerError(e.to_string()))?;

        Ok(())
    }).await?
}

#[component]
fn UserListRowItem(user: DisplayUser, #[prop(into)] refresh: Callback<()>) -> impl IntoView {
    let (is_editing, set_is_editing) = create_signal(false);
    let delete_dialog = create_node_ref::<Dialog>();

    let (username, set_username) = create_signal(user.username.clone());
    let (name, set_name) = create_signal(user.name.clone());
    let (role, set_role) = create_signal(user.role);
    let (email, set_email) = create_signal(user.email.clone().unwrap_or("".to_string()));
    let (email_verified, set_email_verified) = create_signal(user.email_verified);

    let user_cancel = user.clone();
    let cancel_editing = move |_| {
        // Reset all values
        set_username(user_cancel.username.clone());
        set_name(user_cancel.name.clone());
        set_role(user_cancel.role);
        set_email(user_cancel.email.clone().unwrap_or("".to_string()));
        set_email_verified(user_cancel.email_verified);
        set_is_editing(false);
    };

    let update_user_action = create_server_action::<UpdateUser>();

    let update_user_id = user.id.clone();
    let save = move |_| {
        update_user_action.dispatch(UpdateUser { user: DisplayUser {
            id: update_user_id.clone(),
            username: username(),
            name: name(),
            email: if email().is_empty() {
                None
            } else {
                Some(email())
            },
            email_verified: email_verified(),
            role: role(),
        }});
    };

    let delete_user_action = create_server_action::<DeleteUser>();

    let user_delete_id = user.id.clone();
    let delete_user = move |_| {
        delete_dialog.get().unwrap().close();
        delete_user_action.dispatch(DeleteUser{ user_id: user_delete_id.clone() });
    };

    create_effect(move |prev| {
        let (delete_ver, update_ver): (usize, usize) = prev.unwrap_or((0, 0));
        let new_delete_ver = delete_user_action.version()();
        let new_update_ver = update_user_action.version()();
        if new_delete_ver != delete_ver || new_update_ver != update_ver {
            refresh(());
            (new_delete_ver, new_update_ver)
        } else {
            (delete_ver, update_ver)
        }
    });

    let username_str = user.username.clone();

    view! {
        <tr>
            <th scope="row">{user.id}</th>
            <td>
                {move || if is_editing() {
                    view! {<input type="text" prop:value=username on:input=move |ev| set_username(event_target_value(&ev))/>}.into_view()
                } else {
                    view! {<span>{user.username.clone()}</span>}.into_view()
                }}
            </td>
            <td>
                {move || if is_editing() {
                    view! {<input type="text" prop:value=name on:input=move |ev| set_name(event_target_value(&ev))/>}.into_view()
                } else {
                    view! {<span>{user.name.clone()}</span>}.into_view()
                }}
            </td>
            <td>
                {move || if is_editing() {
                    view! {
                        <select prop:value=move|| role().to_string().to_lowercase() on:input=move |ev| {
                            set_role(db::Role::from_str(event_target_value(&ev).as_str()).unwrap());
                        }>
                            <option selected disabled hidden>"Change..."</option>
                            <option value="admin">"Admin"</option>
                            <option value="instructor">"Instructor"</option>
                            <option value="assistant">"Assistant"</option>
                        </select>
                    }.into_view()
                } else {
                    view! {<span>{user.role.to_string()}</span>}.into_view()
                }}
            </td>
            <td>
                {move || if is_editing() {
                    view! {<input type="email" prop:value=email on:input=move |ev| set_email(event_target_value(&ev))/>}.into_view()
                } else {
                    view! {<span>{user.email.clone()}</span>}.into_view()
                }}
            </td>
            <td>
                {move || if is_editing() {
                    view! {<input type="checkbox" prop:checked=email_verified on:input=move |ev| set_email_verified(event_target_checked(&ev))/>}.into_view()
                } else {
                    view! {<span>{user.email_verified}</span>}.into_view()
                }}
            </td>
            <td>
                {move || if is_editing() {
                    view! {<button on:click=save.clone()>"Save"</button><button on:click=cancel_editing.clone()>"Cancel"</button>}
                } else {
                    view! {<button on:click=move |_| set_is_editing(true)>"Edit"</button><button on:click=move |_| delete_dialog.get().unwrap().show_modal().expect("Failed to open removal dialog")>"Remove"</button>}
                }}
            </td>
        </tr>
        <dialog node_ref=delete_dialog>
            <p>"Are you sure you want to delete " {username_str}"?"</p>
            <button on:click=move |_| delete_dialog.get().unwrap().close()>"No, cancel."</button>
            <button on:click=delete_user.clone()>"Yes, delete."</button>
        </dialog>
    }
}
