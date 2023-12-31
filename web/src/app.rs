use futures_util::{StreamExt, TryStreamExt};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rand::distributions::{Alphanumeric, DistString};

use crate::setup::Setup;
use crate::login::{Login, LoggedIn};
use crate::admin::Admin;
use crate::{admin, HumanReadableUser};
use crate::home::{sidebar::CourseSidebar, Home};

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
                        <Route path="courses" view=admin::Courses/>
                    </Route>
                    <Route path="home" view=|| view! {
                        <LoggedIn minimum_role=db::Role::Assistant>
                            <CourseSidebar/>
                        </LoggedIn>
                    }>
                        <Route path=":course" view=Home>
                            <Route path=":section" view=Home>
                                <Route path=":assignment" view=Home/>
                            </Route>
                        </Route>
                        <Route path="" view=|| view! { <p>"Select a course"</p> }/>
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
async fn lookup_users(query: String) -> Result<Vec<HumanReadableUser>, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::web::Data;
    use crate::server::WebState;
    use goldleaf::AutoCollection;
    use db::models::User;
    use mongodb::bson::{from_document, doc};

    let data = extract!(Data<WebState>);

    let query = query.to_lowercase();

    let found_users = data.database.auto_collection::<User>().aggregate(vec![doc! {
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
            }
        ]
    }}}], None).await?.try_collect::<Vec<_>>().await?;

    let found_users = found_users.into_iter().map(from_document).try_collect::<Vec<User>>()?.into_iter().map(|user| HumanReadableUser {
        id: user.id.unwrap().to_hex(),
        username: user.username,
        name: user.name,
    }).collect();

    Ok(found_users)
}

#[component]
pub fn UserSearchBox(selected_user_id: WriteSignal<Option<String>>, #[prop(optional)] placeholder: String) -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let found_users = create_resource(query, |query| async move {
        lookup_users(query).await
    });
    let identifier = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    let identifier_clone = identifier.clone();
    // Tracks when the user backspaces without needing a ReadSignal from the parent
    let (has_valid, set_has_valid) = create_signal(false);
    let placeholder = if placeholder.is_empty() { "Search Users...".to_string() } else { placeholder };

    view! {
        <input type="text" placeholder=placeholder list=move || format!("user-search-{identifier}") prop:value=query on:input=move |ev| {
            if let Some(Ok(users_list)) = found_users() {
                if let Some(user) = users_list.iter().find(|u| u.id == event_target_value(&ev)) {
                    set_query(format!("{} ({})", user.name, user.username));

                    selected_user_id(Some(event_target_value(&ev)));
                    set_has_valid(true);
                } else if has_valid() {
                    selected_user_id(None);
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
    use leptos_actix::extract;
    use actix_web::web::Data;
    use crate::server::WebState;
    use mongodb::bson::{doc, from_document};
    use goldleaf::AutoCollection;
    use db::models::Term;

    let data = extract!(Data<WebState>);

    let stage_sort_by_name = doc! {
        "$sort": {
            "name": 1
        }
    };

    let terms = data.database.auto_collection::<Term>().aggregate(vec![
        stage_sort_by_name,
    ], None).await?.try_collect::<Vec<_>>().await?;

    let terms = terms.into_iter().map(from_document).try_collect::<Vec<Term>>()?.into_iter().map(|t| TermInfo {
        id: t.id.unwrap().to_hex(),
        name: t.name,
    }).collect();
    
    Ok(terms)
}

#[component]
pub fn TermSelector(selected_term_id: WriteSignal<Option<String>>) -> impl IntoView {
    let terms = create_blocking_resource(|| (), |()| async move {
        get_terms().await
    });

    view! {
        <select on:input=move |ev| {
            if let Some(Ok(terms)) = terms.get() {
                if let Some(term) = terms.iter().find(|t| t.id == event_target_value(&ev)) {
                    selected_term_id(Some(term.id.clone()));
                }
            }
        }>
            <option selected disabled>"Term..."</option>
            <Transition>
                {move ||
                    terms.get().map(|terms| {
                        view! {
                            <For
                                each=move || terms.clone().unwrap()
                                key=|term| term.id.clone()
                                children=move |term| {
                                    view! {
                                        <option value={term.id}>{term.name}</option>
                                    }
                                }
                            />
                        }
                    })             
                }
            </Transition>
        </select>
    }
}
