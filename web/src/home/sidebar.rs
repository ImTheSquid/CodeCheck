use std::str::FromStr;

use futures_util::TryStreamExt;
use leptos::{html::Dialog, logging::*, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::*;
use stylist::style;

use crate::{HumanReadableUser};

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
    try_get_all: bool,
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

    let mut aggregation = if try_get_all {
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct SectionInfo {
    id: String,
    name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct FullCourseInfo {
    pub id: String,
    pub name: String,
    pub owner: HumanReadableUser,
    pub instructors: Vec<HumanReadableUser>,
    pub graders: Vec<HumanReadableUser>,
    pub sections: Vec<SectionInfo>,
    pub term_id: String,
}

#[server(GetCourse)]
pub async fn get_course(try_override_membership: bool, id: String) -> Result<FullCourseInfo, ServerFnError> {
    use crate::server_prelude::*;
    use db::models::{User, Course};

    let id = ObjectId::from_str(&id)?;
    let data = extract!(Data<WebState>);

    let aggregation = if try_override_membership {
        let _user = extract!(AuthedUser<{ Role::Admin }>);

        doc! {
            "$match": {
                "_id": id,
            }
        }
    } else {
        let user = extract!(AuthedUser<{ Role::Instructor }>);

        doc! {
            "$match": {
                "_id": id,
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
        }
    };

    let aggregation = vec![
        aggregation,
        // Lookup owner
        doc! {
            "$lookup": {
                "from": User::COLLECTION,
                "localField": "owner",
                "foreignField": "_id",
                "as": "human_owner",
            }
        },
        // Select first for owner
        doc! {
            "$set": {
                "human_owner": {
                    "$arrayElemAt": ["$human_owner", 0],
                }
            }
        },        
        // Attach human readable
        doc! {
            "$set": {
                "human_owner": {
                    "name": "$human_owner.name",
                    "username": "$human_owner.username",
                }
            }
        },
        // Lookup instructors
        doc! {
            "$lookup": {
                "from": User::COLLECTION,
                "localField": "instructors",
                "foreignField": "_id",
                "as": "user_instructors",
            }
        },
        // Lookup graders
        doc! {
            "$lookup": {
                "from": User::COLLECTION,
                "localField": "graders",
                "foreignField": "_id",
                "as": "user_graders",
            }
        },
    ];
    
    let course = data.database.auto_collection::<Course>().aggregate(aggregation, None).await?.try_collect::<Vec<_>>().await?.pop();

    match course {
        None => Err(ServerFnError::ServerError("Invalid course ID".to_string())),
        Some(course) => {
            let course: Course = from_document(course)?;
            let human_owner = course.human_owner.unwrap();
            let human_instructors = course.user_instructors.unwrap();
            let human_graders = course.user_graders.unwrap();
            Ok(FullCourseInfo {
                id: course.id.unwrap().to_hex(),
                name: course.name,
                owner: HumanReadableUser { id: course.owner.to_hex(), name: human_owner.name, username: human_owner.username },
                instructors: human_instructors.into_iter().map(|inst| HumanReadableUser {
                    id: inst.id.unwrap().to_hex(),
                    name: inst.name,
                    username: inst.username,
                }).collect(),
                graders: human_graders.into_iter().map(|grad| HumanReadableUser {
                    id: grad.id.unwrap().to_hex(),
                    name: grad.name,
                    username: grad.username,
                }).collect(),
                term_id: course.term.to_hex(),
                sections: course.sections.into_iter().map(|sect| SectionInfo {
                    id: sect.id.to_hex(),
                    name: sect.name,
                }).collect(),
            })
        }
    }
}

#[component]
pub fn CourseSidebar(
    try_get_all: bool,
    term_id: RwSignal<Option<String>>,
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
                Some(get_courses(try_get_all, term_id).await)
            } else {
                None
            }
        },
    );

    // Sidebar resizing code
    let container = create_node_ref::<html::Div>();
    let (_, set_sidebar_height) = use_css_var("--sidebar-height");
    let resize_listener = window_event_listener(ev::resize, move |ev| {
        let target = event_target::<web_sys::Window>(&ev);
        let height = target.inner_height().expect("window height to be valid").as_f64().expect("window height to be a float");
        let top = container().unwrap().get_bounding_client_rect().y() as f64;
        set_sidebar_height(format!("{}px", height - top));
    });
    on_cleanup(move || resize_listener.remove());

    create_effect(move |_| {
        let height = window().inner_height().expect("window height to be valid").as_f64().expect("window height to be a float");
        let top = container().unwrap().get_bounding_client_rect().y();
        set_sidebar_height(format!("{}px", height - top));
    });

    view! {
        <div node_ref=container class="sidebar-container">
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
