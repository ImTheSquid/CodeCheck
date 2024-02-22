use crate::{
    app::{ServerAction, TermSelector, UserSearchBox},
    home::sidebar::{get_course, CourseSidebar, SectionInfo}, RoleRequirement,
};
use auth::ValidatedUser;
use db::Role;
use futures_util::{StreamExt, TryStreamExt};
use leptos::{html::Dialog, logging::*, *};
use leptos_meta::*;
use leptos_router::*;
use std::str::FromStr;
use styled::style;

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <h1>"CodeCheck Admin Interface"</h1>
        <a href="/logout" rel="external">"Logout"</a>
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
    use crate::server::WebState;
    use crate::AuthedUser;
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use db::models::User;
    use goldleaf::AutoCollection;
    use leptos_actix::{extract, ResponseOptions};
    use mongodb::bson::doc;

    let (data, _user): (Data<WebState>, AuthedUser<{ db::Role::Admin }>) = extract().await?;

    let all_users = data
        .database
        .auto_collection::<User>()
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let all_users = all_users
        .into_iter()
        .map(|u| DisplayUser {
            id: u.id.unwrap().to_hex(),
            username: u.username,
            name: u.name,
            role: u.role,
            email: u.email.as_ref().map(|vr| vr.resource.to_owned()),
            email_verified: u.email.is_some_and(|vr| {
                matches!(vr.status, db::models::VerificationStatus::Verified)
            }),
        })
        .collect();

    Ok(all_users)
}

#[server(CreateUser)]
async fn create_user(
    username: String,
    name: String,
    role: String,
    email: String,
    email_verified: Option<bool>,
    password: String,
) -> Result<(), ServerFnError> {
    let role = db::Role::from_str(&role).unwrap();
    let email = if email.is_empty() { None } else { Some(email) };
    let email_verified = email_verified.unwrap_or_default();
    use crate::server::WebState;
    use crate::AuthedUser;
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    #[cfg(feature = "basic_auth")]
    use argon2::{password_hash::rand_core::OsRng, PasswordHasher};
    use db::models::{User, VerificationStatus, VerifiedResource};
    use goldleaf::{AutoCollection, CollectionIdentity};
    use leptos_actix::{extract, ResponseOptions};
    use mongodb::bson::{doc, oid::ObjectId};

    let (data, _user): (Data<WebState>, AuthedUser<{ db::Role::Admin }>) = extract().await?;

    let email = email.map(|e| VerifiedResource {
        resource: e,
        status: if email_verified {
            VerificationStatus::Verified
        } else {
            // todo: send verification email
            todo!()
        },
    });

    #[cfg(feature = "basic_auth")]
    data.database
        .auto_collection::<User>()
        .insert_one(
            User {
                id: None,
                username,
                name,
                role,
                sessions: vec![],
                email,
                password: argon2::Argon2::default()
                    .hash_password(
                        password.as_bytes(),
                        &argon2::password_hash::SaltString::generate(&mut OsRng),
                    )
                    .expect("hash to be successful")
                    .to_string(),
            },
            None,
        )
        .await?;

    #[cfg(not(feature = "basic_auth"))]
    panic!();

    Ok(())
}

#[component]
pub fn Users() -> impl IntoView {
    let users_res = create_blocking_resource(|| (), |_| async move { get_all_users().await });

    let new_user_dialog = create_node_ref::<Dialog>();
    let new_user_action = create_server_action::<CreateUser>();

    create_effect(move |prev| {
        let prev = prev.unwrap_or(0);
        let new = new_user_action.version()();
        if prev != new {
            new_user_dialog.get().expect("new user dialog to be mounted").close();
            users_res.refetch();
            new
        } else {
            prev
        }
    });

    view! {
        <h2>"User Management"</h2>
        <Transition fallback=|| view!{ <p>"Loading users..."</p> }>
            <table class="adminTable">
                <tr>
                    <th scope="col">"ID"</th>
                    <th scope="col">"Username"</th>
                    <th scope="col">"Name"</th>
                    <th scope="col">"Role"</th>
                    <th scope="col">"Email"</th>
                    <th scope="col">"Email Verified?"</th>
                    <th scope="col"><button on:click=move |_| new_user_dialog.get().expect("new user dialog to be mounted").show_modal().expect("Failed to open add dialog")>"Add"</button></th>
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
                <button on:click=move |_| new_user_dialog.get().expect("new user dialog to be mounted").close()>"Cancel"</button>
            </dialog>
        </Transition>
    }
}

// I only did this because Rust got fussy about the styling macro
// The code looks a bit cleaner now too

use crate::HumanReadableUser;
#[component]
fn NewUserForm(new_user_action: ServerAction<CreateUser>) -> impl IntoView {
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
    use crate::server::WebState;
    use crate::AuthedUser;
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use db::models::User;
    use goldleaf::AutoCollection;
    use leptos_actix::{extract, ResponseOptions};
    use mongodb::bson::{doc, oid::ObjectId};

    let (data, _user): (Data<WebState>, AuthedUser<{ db::Role::Admin }>) = extract().await?;

    let user_id = ObjectId::from_str(&user_id).expect("user_id to be a valid objectID");
    let res = data
        .database
        .auto_collection::<User>()
        .delete_one(doc! { "_id": user_id }, None)
        .await?;
    if res.deleted_count != 1 {
        Err(ServerFnError::ServerError("Invalid user".to_string()))
    } else {
        Ok(())
    }
}

#[server(UpdateUser)]
async fn update_user(user: DisplayUser) -> Result<(), ServerFnError> {
    use crate::server::WebState;
    use crate::AuthedUser;
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use db::models::{User, VerificationStatus, VerifiedResource};
    use goldleaf::{AutoCollection, CollectionIdentity};
    use leptos_actix::{extract, ResponseOptions};
    use mongodb::bson::{doc, oid::ObjectId};

    let (data, _user): (Data<WebState>, AuthedUser<{ db::Role::Admin }>) = extract().await?;

    let email = user.email.map(|e| VerifiedResource {
        resource: e,
        status: if user.email_verified {
            VerificationStatus::Verified
        } else {
            // todo: send verification email
            todo!()
        },
    });

    let user_id = ObjectId::parse_str(user.id).unwrap();

    let mut user_db = data
        .database
        .auto_collection::<User>()
        .find_one(doc! { "_id": user_id }, None)
        .await?
        .unwrap();

    user_db.username = user.username;
    user_db.name = user.name;
    user_db.role = user.role;
    user_db.email = email;

    user_db
        .save(&data.database)
        .await?;

    Ok(())
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
        update_user_action.dispatch(UpdateUser {
            user: DisplayUser {
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
            },
        });
    };

    let delete_user_action = create_server_action::<DeleteUser>();

    let user_delete_id = user.id.clone();
    let delete_user = move |_| {
        delete_dialog.get().expect("delete dialog to be mounted").close();
        delete_user_action.dispatch(DeleteUser {
            user_id: user_delete_id.clone(),
        });
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
                            set_role(db::Role::from_str(event_target_value(&ev).as_str()).expect("role to be valid"));
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
                    view! {<button on:click=move |_| set_is_editing(true)>"Edit"</button><button on:click=move |_| delete_dialog.get().expect("delete dialog to be mounted").show_modal().expect("Failed to open removal dialog")>"Remove"</button>}
                }}
            </td>
        </tr>
        <dialog node_ref=delete_dialog>
            <p>"Are you sure you want to delete " {username_str}"?"</p>
            <button on:click=move |_| delete_dialog.get().expect("delete dialog to be mounted").close()>"No, cancel."</button>
            <button on:click=delete_user.clone()>"Yes, delete."</button>
        </dialog>
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TermInfo {
    id: String,
    name: String,
    /// You can only delete terms that don't have any associated courses
    can_delete: bool,
}

#[server(GetTerms)]
async fn get_terms() -> Result<Vec<TermInfo>, ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use db::models::{Course, Term};
    use goldleaf::{AutoCollection, CollectionIdentity};
    use leptos_actix::extract;
    use mongodb::bson::{doc, from_document};

    let (data, _user): (Data<WebState>, crate::AuthedUser<{ db::Role::Admin }>) = extract().await?;

    let stage_attach_num_associated = doc! {
        "$lookup": {
            "from": Course::COLLECTION,
            "localField": "_id",
            "foreignField": "term",
            "as": "associated_courses",
        }
    };

    let stage_set_can_delete = doc! {
        "$set": {
            "can_delete": {
                "$cond": {
                    "if": {
                        "$gt": [
                            {"$size": "$associated_courses"},
                            0,
                        ]
                    },
                    "then": false,
                    "else": true,
                }
            }
        }
    };

    let stage_sort_descending = doc! {
        "$sort": {
            "_id": -1
        }
    };

    let terms = data
        .database
        .auto_collection::<Term>()
        .aggregate(
            vec![
                stage_attach_num_associated,
                stage_set_can_delete,
                stage_sort_descending,
            ],
            None,
        )
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let terms = terms
        .into_iter()
        .map(from_document)
        .try_collect::<Vec<Term>>()?
        .into_iter()
        .map(|t| TermInfo {
            id: t.id.expect("term to have an id").to_hex(),
            name: t.name,
            can_delete: t.can_delete.expect("can delete to be attached to the document"),
        })
        .collect::<Vec<_>>();

    Ok(terms)
}

#[server(CreateTerm)]
async fn create_term(name: String) -> Result<(), ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use db::models::Term;
    use goldleaf::AutoCollection;
    use leptos_actix::extract;

    let (data, _user): (Data<WebState>, crate::AuthedUser<{ db::Role::Admin }>) = extract().await?;

    data.database
        .auto_collection::<Term>()
        .insert_one(
            Term {
                id: None,
                name,
                can_delete: None,
            },
            None,
        )
        .await?;

    Ok(())
}

#[server(DeleteTerm)]
async fn delete_term(id: String) -> Result<(), ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use db::models::{Course, Term};
    use goldleaf::AutoCollection;
    use leptos_actix::extract;
    use mongodb::bson::{doc, oid::ObjectId};

    let (data, _user): (Data<WebState>, crate::AuthedUser<{ db::Role::Admin }>) = extract().await?;

    let id = ObjectId::from_str(&id).expect("id to be valid");

    let num_associated = data
        .database
        .auto_collection::<Course>()
        .count_documents(
            doc! {
                "term": id
            },
            None,
        )
        .await?;

    if num_associated > 0 {
        return Err(ServerFnError::ServerError(
            "Cannot delete term with associated courses".to_string(),
        ));
    }

    data.database
        .auto_collection::<Term>()
        .delete_one(
            doc! {
                "_id": id
            },
            None,
        )
        .await?;

    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BaseCourseInfo {
    id: String,
    name: String,
    owner: HumanReadableUser,
}

#[server(GetCourses)]
async fn get_courses(term_id: String) -> Result<Vec<BaseCourseInfo>, ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use db::models::{Course, User};
    use goldleaf::{AutoCollection, CollectionIdentity};
    use leptos_actix::extract;
    use mongodb::bson::{doc, from_document, oid::ObjectId};

    let (data, _user): (Data<WebState>, crate::AuthedUser<{ db::Role::Admin }>) = extract().await?;

    let stage_match_term_id = doc! {
        "$match": {
            "term": term_id,
        }
    };

    let stage_lookup_owner = doc! {
        "$lookup": {
            "from": User::COLLECTION,
            "foreignField": "_id",
            "localField": "owner",
            "as": "owner_doc",
        }
    };

    let stage_attach_human_readable_owner = doc! {
        "$set": {
            "human_owner": {
                "name": "$owner_doc[0].name",
                "username": "$owner_doc[0].username",
            }
        }
    };

    let stage_sort_name_ascending = doc! {
        "$sort": {
            "name": 1,
        }
    };

    let courses = data
        .database
        .auto_collection::<Course>()
        .aggregate(
            vec![
                stage_match_term_id,
                stage_lookup_owner,
                stage_attach_human_readable_owner,
                stage_sort_name_ascending,
            ],
            None,
        )
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let courses = courses
        .into_iter()
        .map(from_document)
        .try_collect::<Vec<Course>>()?
        .into_iter()
        .map(|c| BaseCourseInfo {
            id: c.id.expect("id to exist on the course").to_hex(),
            name: c.name,
            owner: {
                let human_owner = c.human_owner.expect("human owner to be attaached to the document");
                HumanReadableUser {
                    id: c.owner.to_hex(),
                    name: human_owner.name,
                    username: human_owner.username,
                }
            },
        })
        .collect();

    Ok(courses)
}

#[server(CreateCourse)]
async fn create_course(
    name: String,
    owner_id: String,
    term_id: String,
) -> Result<(), ServerFnError> {
    use crate::{server::WebState, AuthedUser};
    use actix_web::web::Data;
    use db::models::{Course, User};
    use goldleaf::{AutoCollection, CollectionIdentity};
    use leptos_actix::extract;
    use mongodb::bson::{doc, from_document, oid::ObjectId};

    let (data, _user): (Data<WebState>, AuthedUser<{ db::Role::Admin }>) = extract().await?;

    let new_course = Course {
        id: None,
        name,
        owner: ObjectId::from_str(&owner_id)?,
        instructors: vec![],
        graders: vec![],
        sections: vec![],
        term: ObjectId::from_str(&term_id)?,
        ..Default::default()
    };

    data.database
        .auto_collection::<Course>()
        .insert_one(new_course, None)
        .await?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct SelectedTerm(WriteSignal<Option<String>>);

#[component]
pub fn Courses() -> impl IntoView {
    let terms = create_blocking_resource(|| (), |_| async move { get_terms().await });

    let manage_terms_ref = create_node_ref::<Dialog>();
    let create_course_ref = create_node_ref::<Dialog>();

    let (new_term_name, set_new_term_name) = create_signal(String::new());

    let add_term = create_server_action::<CreateTerm>();
    let delete_term = create_server_action::<DeleteTerm>();

    // Refreshes term list when a term is added or deleted
    create_effect(move |prev| {
        let (add_prev, delete_prev) = prev.unwrap_or((0_usize, 0_usize));
        let add_new = add_term.version()();
        let delete_new = delete_term.version()();
        if add_prev != add_new || delete_prev != delete_new {
            set_new_term_name(String::new());
            terms.refetch();
            (add_new, delete_new)
        } else {
            (add_prev, delete_prev)
        }
    });

    let new_course_trigger = create_trigger();
    let course_term = create_rw_signal(None);

    provide_context(SelectedTerm(course_term.write_only()));

    let create_course = create_server_action::<CreateCourse>();

    // Refreshes courses when a new one is created
    create_effect(move |prev| {
        let course_prev = prev.unwrap_or(0_usize);
        let course_new = create_course.version()();
        if course_new != course_prev {
            new_course_trigger.notify();
            create_course_ref.get().expect("create course dialog to be mounted").close();
            course_new
        } else {
            course_prev
        }
    });

    // Removes URL path when term is changed
    // create_effect(move |prev_term: Option<Option<String>>| {
    //     if let Some(term) = prev_term {
    //         if term != course_term.get() {
    //             if term.is_some() {
    //                 leptos_router::use_navigate()("/admin/courses/", Default::default());
    //             }
    //             course_term.get()
    //         } else {
    //             term
    //         }
    //     } else {
    //         course_term.get()
    //     }
    // });

    view! {
        <div>
            <h2>"Course and Term Configuration"</h2>
            <button on:click=move |_| manage_terms_ref.get().expect("manage terms dialog to be mounted").show_modal().expect("Failed to show Term modal!")>"Manage Terms..."</button>
            <button on:click=move |_| create_course_ref.get().expect("create course dialog to be mounted").show_modal().expect("Failed to show course modal!")>"Create Course..."</button>
            <TermSelector selected_term_id=course_term/>
            <CourseSidebar try_get_all=true term_id=course_term refresh=new_course_trigger/>
        </div>
        <dialog node_ref=create_course_ref>
            <h1>"Create Course"</h1>
            <CourseCreator create_course=create_course create_course_ref=create_course_ref/>
        </dialog>
        <dialog node_ref=manage_terms_ref>
            <h1>"Manage Terms"</h1>
            <div class="term-actions">
                <input
                    type="text"
                    placeholder="New Term Name"
                    prop:value=new_term_name
                    on:input=move |ev| set_new_term_name(event_target_value(&ev))
                    on:keypress=move |ev| {
                        if ev.key() == "Enter" && !new_term_name().is_empty() {
                            add_term.dispatch(CreateTerm { name: new_term_name() });
                        }
                    }
                />
                <button
                    disabled=move || new_term_name.with(|ntn| ntn.is_empty())
                    on:click=move |_| add_term.dispatch(CreateTerm { name: new_term_name() })
                >"Add Term"</button>
            </div>
            <p>"Terms may only be deleted if there are no courses associated with it."</p>
            <table class="adminTable">
                <tr>
                    <th scope="col">"ID"</th>
                    <th scope="col">"Name"</th>
                </tr>
                <Transition fallback=||()>
                    {move || terms.get().map(|terms|
                                             view! {
                    <For
                        each=move || terms.clone().expect("terms to not have failed")
                        key=|term| term.id.clone()
                        children=move |term| {
                            let term_id = term.id.clone();

                            view! {
                                <tr>
                                    <th scope="row">{term.id}</th>
                                    <td>{term.name}</td>
                                    <td><button
                                        disabled=move || !term.can_delete
                                        on:click=move |_| delete_term.dispatch(DeleteTerm { id: term_id.clone() })
                                    >"Delete"</button></td>
                                </tr>
                            }
                        }
                    />})}
                </Transition>
            </table>
            <button on:click=move |_| manage_terms_ref.get().expect("manage terms dialog to be mounted").close()>"Close"</button>
        </dialog>
    }
}

#[component]
fn CourseCreator(
    create_course: ServerAction<CreateCourse>,
    create_course_ref: NodeRef<Dialog>,
) -> impl IntoView {
    let (new_course_name, set_new_course_name) = create_signal(String::new());
    let new_course_owner = create_rw_signal(None);
    // let (new_course_term, set_new_course_term) = create_signal(None);
    let new_course_term = create_rw_signal(None);

    let on_click = move |_| {
        create_course.dispatch(CreateCourse {
            name: new_course_name(),
            owner_id: new_course_owner().expect("because submission is blocked until not None"),
            term_id: new_course_term().expect("because submission is blocked until not None"),
        });
        create_course_ref.get().expect("create course dialog to be mounted").close();
    };

    view! {
        <div>
            <input
                type="text"
                name="name"
                placeholder="New Course Name"
                prop:value=new_course_name
                on:input=move |ev| set_new_course_name(event_target_value(&ev))
            />
            <UserSearchBox selected_user_id=new_course_owner placeholder="Search for Course Owner..." name="owner_id"/>
            <TermSelector selected_term_id=new_course_term name="term_id"/>
            <button on:click=move |_| create_course_ref.get().expect("create course dialog to be mounted").close()>"Cancel"</button>
            <button disabled=move || new_course_name.with(String::is_empty) || new_course_owner.with(Option::is_none) || new_course_term.with(Option::is_none)
                on:click=on_click
            >"Create"</button>
        </div>
    }
}

#[server(SaveCourseInfo)]
async fn save_course_info(course_id: String, name: String, owner_id: String) -> Result<(), ServerFnError> {
    use crate::server_prelude::*;
    use db::models::Course;

    let (data, _user): (Data<WebState>, AuthedUser::<{ Role::Admin }>) = extract().await?;

    data.database.auto_collection::<Course>().update_one(doc! {
        "_id": ObjectId::from_str(&course_id)?,
    }, doc! {
        "$set": {
            "name": name,
            "owner": ObjectId::from_str(&owner_id)?,
        }
    }, None).await?;

    Ok(())
}

#[server(DeleteCourse)]
async fn delete_course(course_id: String) -> Result<(), ServerFnError> {
    use crate::server_prelude::*;
    use db::models::Course;
    
    let (data, _user): (Data<WebState>, AuthedUser::<{ Role::Admin }>) = extract().await?;

    data.database.auto_collection::<Course>().delete_one(doc! {
        "_id": ObjectId::from_str(&course_id)?
    }, None).await?;

    Ok(())
}

#[server(CreateSection)]
async fn create_section(course_id: String, section_name: String) -> Result<(), ServerFnError> {
    use crate::server_prelude::*;
    use db::models::{Course, CourseSection};
    
    let (data, _user): (Data<WebState>, AuthedUser::<{ Role::Admin }>) = extract().await?;

    data.database.auto_collection::<Course>().update_one(doc! {
        "_id": ObjectId::from_str(&course_id)?,
    }, doc! {
        "$push": {
            "sections": to_bson(&CourseSection { id: ObjectId::new(), name: section_name })?,
        }
    }, None).await?;

    Ok(())
}

#[server(DeleteSection)]
async fn delete_section(course_id: String, section_id: String) -> Result<(), ServerFnError> {
    use crate::server_prelude::*;
    use db::models::Course;

    let (data, _user): (Data<WebState>, AuthedUser::<{ Role::Admin }>) = extract().await?;

    data.database.auto_collection::<Course>().update_one(doc! {
        "_id": ObjectId::from_str(&course_id)?,
    }, doc! {
        "$pull": {
            "sections": {
                "id": ObjectId::from_str(&section_id)?,
            }
        }
    }, None).await?;

    Ok(())
}

#[server(AddInstructor)]
async fn add_instructor(course_id: String, user_id: String) -> Result<(), ServerFnError> {
    use crate::server_prelude::*;
    use db::models::Course;

    let (data, _user): (Data<WebState>, AuthedUser::<{ Role::Admin }>) = extract().await?;

    data.database.auto_collection::<Course>().update_one(doc! {
        "_id": ObjectId::from_str(&course_id)?,
    }, doc! {
        "$push": {
            "instructors": ObjectId::from_str(&user_id)?,
        }
    }, None).await?;

    Ok(())
}

#[server(RemoveInstructor)]
async fn remove_instructor(course_id: String, user_id: String) -> Result<(), ServerFnError> {
    use crate::server_prelude::*;
    use db::models::Course;

    let (data, _user): (Data<WebState>, AuthedUser::<{ Role::Admin }>) = extract().await?;

    data.database.auto_collection::<Course>().update_one(doc! {
        "_id": ObjectId::from_str(&course_id)?,
    }, doc! {
        "$pull": {
            "instructors": ObjectId::from_str(&user_id)?,
        }
    }, None).await?;

    Ok(())
}

#[server(AddGrader)]
async fn add_grader(course_id: String, user_id: String) -> Result<(), ServerFnError> {
    use crate::server_prelude::*;
    use db::models::Course;

    let (data, _user): (Data<WebState>, AuthedUser::<{ Role::Admin }>) = extract().await?;

    data.database.auto_collection::<Course>().update_one(doc! {
        "_id": ObjectId::from_str(&course_id)?,
    }, doc! {
        "$push": {
            "graders": ObjectId::from_str(&user_id)?,
        }
    }, None).await?;

    Ok(())
}

#[server(RemoveGrader)]
async fn remove_grader(course_id: String, user_id: String) -> Result<(), ServerFnError> {
    use crate::server_prelude::*;
    use db::models::Course;

    let (data, _user): (Data<WebState>, AuthedUser::<{ Role::Admin }>) = extract().await?;

    data.database.auto_collection::<Course>().update_one(doc! {
        "_id": ObjectId::from_str(&course_id)?,
    }, doc! {
        "$pull": {
            "graders": ObjectId::from_str(&user_id)?,
        }
    }, None).await?;

    Ok(())
}

#[component]
pub fn Course() -> impl IntoView {
    let set_selected_term = expect_context::<SelectedTerm>();
    let create_section = create_server_action::<CreateSection>();
    let delete_section = create_server_action::<DeleteSection>();
    let save_course = create_server_action::<SaveCourseInfo>();
    let delete_course = create_server_action::<DeleteCourse>();
    let add_instructor = create_server_action::<AddInstructor>();
    let remove_instructor = create_server_action::<RemoveInstructor>();
    let add_grader = create_server_action::<AddGrader>();
    let remove_grader = create_server_action::<RemoveGrader>();
    let params = use_params_map();
    let course_id = move || params.with(|params| params.get("course").cloned().expect(":course to be in URL params"));

    let course = create_blocking_resource(move || {
        course_id()
    }, |course| async move {
        get_course(true, course).await
    });

    // let styles = style!(
    //     div.fields {
    //         display: inline-block;
    //     }
    //
    //     div.lists {
    //         display: flex;
    //     }
    //
    //     div.lists > div {
    //         flex: 1;
    //         margin: 0 1e0em;
    //     }
    //
    //     div.list-inline {
    //         display: inline-block;
    //     }
    // );

    let (name, set_name) = create_signal(String::new());
    let selected_user_id = create_rw_signal(None);

    create_effect(move |_| {
        if let Some(Ok(course)) = course() {
            set_selected_term.0(Some(course.term_id));
            set_name(course.name);
            selected_user_id.set(Some(course.owner.id));
        }
    });

    create_effect(move |_| {
        create_section.version().track();
        delete_section.version().track();
        save_course.version().track();
        add_instructor.version().track();
        remove_instructor.version().track();
        add_grader.version().track();
        remove_grader.version().track();
        course.refetch();
    });

    let (new_section_name, set_new_section_name) = create_signal(String::new());
    let new_instructor = create_rw_signal(None);
    let new_grader = create_rw_signal(None);

    let create_new_section = move || {
        create_section.dispatch(CreateSection { course_id: course_id(), section_name: new_section_name() });
        set_new_section_name(String::new());
    };

    let add_instructor = move || {
        add_instructor.dispatch(AddInstructor { course_id: course_id(), user_id: new_instructor().expect("instructor to be selected") });
        new_instructor.set(None);
    };

    let add_grader = move || {
        add_grader.dispatch(AddGrader { course_id: course_id(), user_id: new_grader().expect("grader to be selected") });
        new_grader.set(None);
    };

    view! {
        <h1>"Manage "{move || course().map(|c| c.expect("course to load properly").name).unwrap_or("Course".to_string())}</h1>
        <div class="fields">
            <input prop:value=name on:input=move |ev| set_name(event_target_value(&ev)) placeholder="Course Name"/>
            <UserSearchBox selected_user_id=selected_user_id placeholder="Course Owner"/>
            <button disabled=move || name.with(String::is_empty) || selected_user_id.with(Option::is_none) on:click=move |_| save_course.dispatch(SaveCourseInfo { course_id: course_id(), name: name(), owner_id: selected_user_id().expect("owner to be selected")})>"Save"</button>
            <button on:click=move |_| {
                delete_course.dispatch(DeleteCourse { course_id: course_id() });
                use_navigate()("/admin/courses", Default::default());
            }>"Delete (can't undo)"</button>
        </div>
        <div class="lists">
            <div>
                <h2>"Sections"</h2>
                <div class="list-inline">
                    <input type="text" placeholder="New Section Name" prop:value=new_section_name on:input=move |ev| set_new_section_name(event_target_value(&ev)) on:keypress=move |ev| if ev.key() == "Enter" && !new_section_name.with(String::is_empty) { create_new_section() }/>
                    <button disabled=move || new_section_name.with(String::is_empty) on:click=move |_| create_new_section()>"Create"</button>
                </div>
                <Transition>
                    {move || course().map(|c| {
                        view! {
                            <div>
                            <For
                                each=move || c.clone().expect("course to have loaded").sections
                                key=|s| s.id.clone()
                                children=move |s| {
                                    let id = s.id.clone();
                                    view! {
                                        <div class="list-inline">
                                            <p>{s.name}</p>
                                            <button on:click=move |_| delete_section.dispatch(DeleteSection { course_id: course_id(), section_id: id.clone() })>"Remove"</button>
                                        </div>
                                    }
                                }
                            />
                                </div>
                        }}
                    )}
                </Transition>
            </div>
            <div>
                <h2>"Instructors"</h2>
                <div class="list-inline">
                    <UserSearchBox selected_user_id=new_instructor placeholder="New Instructor" role_requirement=RoleRequirement::new(db::Role::Instructor, crate::RoleRequirementCondition::ExactOrGreater)/>
                    <button disabled=move || new_instructor.with(Option::is_none) on:click=move |_| add_instructor()>"Add"</button>
                </div>
                <Transition>
                    {move || course().map(|c|
                        view! {
                            <div>
                            <For
                                each=move || c.clone().expect("course to have loaded").instructors
                                key=|i| i.id.clone()
                                children=move |i| {
                                    view! {
                                        <div>
                                            <p>{format!("{} ({})", i.name, i.username)}</p>
                                            <button on:click=move |_| remove_instructor.dispatch(RemoveInstructor { course_id: course_id(), user_id: i.id.clone() })>"Remove"</button>
                                        </div>
                                    }
                                }
                            />
                                </div>
                        }
                    )}
                </Transition>
            </div>
            <div>
                <h2>"Graders"</h2>
                <div class="list-inline">
                    <UserSearchBox selected_user_id=new_grader placeholder="New Grader"/>
                    <button disabled=move || new_grader.with(Option::is_none) on:click=move |_| add_grader()>"Add"</button>
                </div>
                <Transition>
                    {move || course().map(|c|
                        view! {
                            <div>
                            <For
                                each=move || c.clone().expect("course to have loaded").graders
                                key=|i| i.id.clone()
                                children=move |i| {
                                    view! {
                                        <div>
                                            <p>{format!("{} ({})", i.name, i.username)}</p>
                                            <button on:click=move |_| remove_grader.dispatch(RemoveGrader { course_id: course_id(), user_id: i.id.clone() })>"Remove"</button>
                                        </div>
                                    }
                                }
                            />
                                </div>
                        }
                    )}
                </Transition>
            </div>
        </div>
    }
}

#[component]
fn CourseSectionListRowItem(section: SectionInfo, #[prop(into)] on_remove: Callback<()>) -> impl IntoView {
    // let styles = style!(
    //     div {
    //         display: inline-block;
    //         justify-content: space-between;
    //     }
    // );

    view! {
        <div>
            <p>{section.name}</p>
            <button on:click=move |_| on_remove(())>"Remove"</button>
        </div>
    }
}

#[component]
fn CourseUserListRowItem(user: HumanReadableUser, #[prop(into)] on_remove: Callback<()>) -> impl IntoView {
    // let styles = style!(
    //     div {
    //         display: inline-block;
    //         justify-content: space-between;
    //     }
    // );

    view! {
        <div>
            <p>{format!("{} ({})", user.name, user.username)}</p>
            <button on:click=move |_| on_remove(())>"Remove"</button>
        </div>
    }
}
