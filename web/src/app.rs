use futures_util::{StreamExt, TryStreamExt};
use leptos::{logging::*, *};
use leptos_meta::*;
use leptos_router::*;
use rand::distributions::{Alphanumeric, DistString};

use crate::admin::Admin;
use crate::home::{sidebar::CourseSidebar, Home};
use crate::login::{LoggedIn, Login};
use crate::setup::Setup;
use crate::{admin, HumanReadableUser, RoleRequirement};

pub type ServerAction<T> = Action<T, Result<<T as server_fn::ServerFn<()>>::Output, ServerFnError>>;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/codecheck.css"/>

        // sets the document title
        <Title text="CodeCheck"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="setup" view=Setup/>
                    <Route path="login" view=Login/>
                    <Route path="admin" view=|| view! {
                        <LoggedIn>
                            <Admin/>
                        </LoggedIn>
                    }>
                        <Route path="users" view=admin::Users/>
                        <Route path="courses" view=admin::Courses>
                            <Route path=":course" view=admin::Course/>
                            <Route path="" view=|| view! { <p>"Select a course."</p> }/>
                        </Route>
                    </Route>
                    <Route path="home" view=|| view! {
                        <LoggedIn minimum_role=db::Role::Assistant>
                            //<CourseSidebar try_get_all=false/>
                            <Outlet/>
                        </LoggedIn>
                    }>
                        <Route path=":course" view=Home>
                            <Route path=":section" view=Home>
                                <Route path=":assignment" view=Home/>
                            </Route>
                        </Route>
                        <Route path="" view=Home/>
                    </Route>
                    <Route path="*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let query = use_query_map();
    let did_logout = query.with(|q| q.get("logout").is_some());

    view! {
        <Show when=move || did_logout fallback=|| ()>
            <p style="color: green;"><strong>"Logout Successful!"</strong></p>
        </Show>
        <h1>"Welcome to CodeCheck!"</h1>
        <h2>"A research project by Jack Hogan, Vidit Patel, and Ava Lyall"</h2>
        <h3><a href="https://github.com/ImTheSquid/CodeCheck" target="_blank" rel="noopener">"Source Repository"</a></h3>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server(LookupUsers)]
async fn lookup_users(query: String, role_requirement: RoleRequirement) -> Result<Vec<HumanReadableUser>, ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use db::models::User;
    use goldleaf::AutoCollection;
    use leptos_actix::extract;
    use mongodb::bson::{doc, from_document};

    let data = extract!(Data<WebState>);

    let query = query.to_lowercase();

    let found_users = data
        .database
        .auto_collection::<User>()
        .aggregate(
            vec![doc! {
                "$match": { "$expr": {
                "$or": [
                    {
                        "$ne": [
                            {"indexOfCP": [{"$toLower": "$username"}, &query]},
                            -1
                        ]
                    },
                    {
                        "$ne": [
                            {"indexOfCP": [{"$toLower": "$name"}, &query]},
                            -1
                        ]
                    },
                    {
                        "_id": &query,
                    }
                ]
            }}}],
            None,
        )
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let found_users = found_users
        .into_iter()
        .map(from_document)
        .try_collect::<Vec<User>>()?
        .into_iter()
        .filter_map(|user| 
            if role_requirement.includes_role(user.role) {
                Some(
                    HumanReadableUser {
                        id: user.id.expect("id to exist").to_hex(),
                        username: user.username,
                        name: user.name,
                })
            } else {
                None
            }
        )
        .collect();

    Ok(found_users)
}

#[component]
pub fn UserSearchBox(
    selected_user_id: RwSignal<Option<String>>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] name: String,
    #[prop(optional)] role_requirement: RoleRequirement,
) -> impl IntoView {
    let (selected_user_id, set_selected_user_id) = selected_user_id.split();
    let (query, set_query) = create_signal(String::new());
    let found_users = create_resource(query, move |query| async move {
        if query.is_empty() {
            Ok(vec![])
        } else {
            lookup_users(query, role_requirement).await
        }
    });
    let identifier = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    let identifier_clone = identifier.clone();
    // Tracks when the user backspaces without needing a ReadSignal from the parent
    let (has_valid, set_has_valid) = create_signal(false);
    let placeholder = if placeholder.is_empty() {
        "Search Users...".to_string()
    } else {
        placeholder
    };

    // Jank-ass solution to stop stuff from conflicting
    let (lock, set_lock) = create_signal(false);

    // Allows for the parent to set the ID
    create_effect(move |_| {
        if let Some(Ok(users)) = found_users() {
            if let Some(id) = selected_user_id() {
                if let Some(user) = users.iter().find(|u| u.id == id) {
                    set_query(format!("{} ({})", user.name, user.username));

                    set_has_valid(true);
                } else {
                    set_query(id);
                    set_has_valid(false);
                }
            } else {
                // Damn this is dirty
                if lock.get_untracked() {
                    set_lock.set_untracked(false);
                    return;
                }
                set_query(String::new());
                set_has_valid(false);
            }
        }
    });

    view! {
        <input type="text" placeholder=placeholder list=move || format!("user-search-{identifier}") name=name prop:value=query on:input=move |ev| {
            set_lock(true);
            set_query(event_target_value(&ev));
            if let Some(Ok(users_list)) = found_users() {
                if let Some(user) = users_list.iter().find(|u| u.id == event_target_value(&ev)) {
                    set_query(format!("{} ({})", user.name, user.username));

                    set_lock(false);
                    set_selected_user_id(Some(user.id.clone()));
                    set_has_valid(true);
                } else if has_valid() {
                    set_selected_user_id(None);
                    set_query(String::new());
                    set_has_valid(false);
                }
            }
        }/>
        <datalist id=move || format!("user-search-{identifier_clone}")>
            <Transition fallback=|| ()>
                {move ||
                    found_users().map(|found_users| {
                        view! {
                            <For
                                each=move || found_users.clone().expect("users to be valid")
                                key=|found_user| found_user.id.clone()
                                children=|found_user| {
                                    view! {
                                        <option value={found_user.id}>{format!("{} ({})", found_user.name, found_user.username)}</option>
                                    }
                                }
                            />
                        }
                    })
                }
            </Transition>
        </datalist>
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TermInfo {
    id: String,
    name: String,
}

#[server(GetTerms)]
async fn get_terms() -> Result<Vec<TermInfo>, ServerFnError> {
    use crate::server::WebState;
    use actix_web::web::Data;
    use db::models::Term;
    use goldleaf::AutoCollection;
    use leptos_actix::extract;
    use mongodb::bson::{doc, from_document};

    let data = extract!(Data<WebState>);

    let stage_sort_by_name = doc! {
        "$sort": {
            "name": 1
        }
    };

    let terms = data
        .database
        .auto_collection::<Term>()
        .aggregate(vec![stage_sort_by_name], None)
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let terms = terms
        .into_iter()
        .map(from_document)
        .try_collect::<Vec<Term>>()?
        .into_iter()
        .map(|t| TermInfo {
            id: t.id.expect("id to be valid").to_hex(),
            name: t.name,
        })
        .collect();

    Ok(terms)
}

#[component]
pub fn TermSelector(
    selected_term_id: RwSignal<Option<String>>,
    #[prop(optional, into)] name: String,
) -> impl IntoView {
    let terms = create_blocking_resource(|| (), |()| async move { get_terms().await });

    let (selected_term_id, set_selected_term_id) = selected_term_id.split();

    let select = create_node_ref::<leptos::html::Select>();

    // Sets the initial value for the selector once terms have loaded
    let (did_initial, set_did_initial) = create_signal(false);

    create_effect(move |_| {
        if let Some(Ok(terms)) = terms() {
            if let Some(term_id) = selected_term_id() {
                select().expect("select to be mounted").set_value(&term_id);
            } else {
                // Due to how the HTML works it always chooses the last item on load
                if let Some(term) = terms.last() {
                    if !did_initial() {
                        set_selected_term_id(Some(term.id.clone()));
                        set_did_initial(true);
                    }
                }
            }
        }
    });

    view! {
        <select name=name node_ref=select on:input=move |ev| {
            if let Some(Ok(terms)) = terms.get() {
                if let Some(term) = terms.iter().find(|t| t.id == event_target_value(&ev)) {
                    set_selected_term_id(Some(term.id.clone()));
                }
            }
        }>
            <Transition fallback=|| view! {
                <option selected disabled>"Term..."</option>
            }>
                {move ||
                    terms().map(|terms| {
                        let terms = store_value(terms);
                        let has_terms = !terms().as_ref().expect("terms to be loaded").is_empty();

                        view! {
                            <Show
                                when=move || has_terms
                                fallback=|| view! { <option selected disabled>"Term..."</option> }
                            >
                            <For
                                each=move || terms().clone().expect("terms to be valid")
                                key=|term| term.id.clone()
                                children=move |term| {
                                    view! {
                                        <option value={term.id}>{term.name}</option>
                                    }
                                }
                            />
                            </Show>
                        }
                    })
                }
            </Transition>
        </select>
    }
}
