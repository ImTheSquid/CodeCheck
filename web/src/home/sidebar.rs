use std::str::FromStr;

use futures_util::TryStreamExt;
use leptos::{html::Dialog, *};
use leptos_meta::*;
use leptos_router::*;
use stylist::style;

use crate::HumanReadableUser;

#[component]
pub fn OutletSidebar() -> impl IntoView {
    view! {
        <div id="sidebar">
            //<CourseSidebar try_get_all=false/>
        </div>

        <div id="content">
            <Outlet/>
        </div>
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CourseInfo {
    id: String,
    name: String,
    owner: HumanReadableUser,
}

#[server(GetCourses)]
async fn get_courses(
    try_get_all: String,
    term_id: String,
) -> Result<Vec<CourseInfo>, ServerFnError> {
    use crate::server::WebState;
    use crate::AuthedUser;
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use db::models::{Course, User};
    use goldleaf::{AutoCollection, CollectionIdentity};
    use leptos_actix::{extract, ResponseOptions};
    use mongodb::bson::{doc, from_document, oid::ObjectId};

    let term_id = ObjectId::from_str(&term_id)?;

    let data = extract!(Data<WebState>);

    let mut aggregation = if bool::from_str(&try_get_all)? {
        let _user = extract!(AuthedUser<{ db::Role::Admin }>);

        Vec::new()
    } else {
        let user = extract!(AuthedUser<{ db::Role::Instructor }>);

        vec![doc! {
            "$match": {
                "$or": [
                    {
                        "owner": user.id,
                    },
                    {
                        "instructors": user.id,
                    },
                    {
                        "graders": user.id,
                    },
                ]
            }
        }]
    };

    aggregation.append(&mut vec![
        doc! {
            "$match": {
                "term": term_id,
            }
        },
        // Lookup owner
        doc! {
            "$lookup": {
                "from": User::COLLECTION,
                "foreignField": "_id",
                "localField": "owner",
                "as": "owner_doc",
            }
        },
        // Select first in lookup
        doc! {
            "$set": {
                "owner_doc": {
                    "$arrayElemAt": ["$owner_doc", 0]
                }
            }
        },
        // Attach human readable
        doc! {
            "$set": {
                "human_owner": {
                    "name": "$owner_doc.name",
                    "username": "$owner_doc.username",
                }
            }
        },
        // Sort by name ascending
        doc! {
            "$sort": {
                "name": 1,
            }
        },
    ]);

    let courses = data
        .database
        .auto_collection::<Course>()
        .aggregate(aggregation, None)
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let courses = courses
        .into_iter()
        .map(from_document)
        .try_collect::<Vec<Course>>()?
        .into_iter()
        .map(|c| CourseInfo {
            id: c.id.unwrap().to_hex(),
            name: c.name,
            owner: {
                let human_owner = c.human_owner.unwrap();
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

#[component]
pub fn CourseSidebar(
    try_get_all: bool,
    #[prop(into)] term_id: Signal<Option<String>>,
    #[prop(optional)] refresh: Option<Trigger>,
) -> impl IntoView {
    let courses = create_blocking_resource(
        move || {
            if let Some(refresh) = refresh {
                refresh();
            }
            term_id()
        },
        move |term_id| async move {
            if let Some(term_id) = term_id {
                Some(get_courses(try_get_all.to_string(), term_id).await)
            } else {
                None
            }
        },
    );

    view! {
        <div class="sidebar-container">
            <div class="sidebar">
                <Transition fallback=|| view! { <p>"No courses found."</p> }>
                    {move || courses().map(|courses| {
                        match courses {
                            Some(courses) =>
                        view! {
                            <For
                                each=move || courses.clone().unwrap()
                                key=|course| course.id.clone()
                                children=|course| {
                                    view! {
                                        <CourseLink info=course/>
                                    }
                                }
                            />
                        }.into_view(),
                            None => view! { <p>"No term selected."</p> }.into_view(),
                        }
                    })}
                </Transition>
            </div>
            <div class="content">
                <Outlet/>
            </div>
        </div>
    }
}

#[component]
fn CourseLink(info: CourseInfo) -> impl IntoView {
    let styles = style!(
        div {
            display: block;
            text-decoration: none;

        }
    );

    styled::view! { styles,
        <A href={info.id}>
            <strong><p class="name">{info.name}</p></strong>
            <p class="owner">{move || format!("{} ({})", info.owner.name, info.owner.username)}</p>
        </A>
    }
}
