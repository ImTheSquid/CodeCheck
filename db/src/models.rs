use goldleaf::CollectionIdentity;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Instructor,
    Assistant,
}

pub type UserId = ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    token: String,
    expiration: DateTime<Utc>,
    last_use: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, CollectionIdentity)]
#[db(name = "user")]
pub struct User {
    #[db(native_id_field)]
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<UserId>,
    pub username: String,
    pub role: Role,
    #[cfg(feature = "basic_auth")]
    pub password: String,
    #[cfg(feature = "basic_auth")]
    pub salt: String,
    pub sessions: Vec<Session>,
}

pub type CourseId = ObjectId;
pub type CourseSectionId = ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseSection {
    pub id: CourseSectionId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, CollectionIdentity)]
#[db(name = "course")]
pub struct Course {
    #[db(native_id_field)]
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<CourseId>,
    pub name: String,
    pub owner: UserId,
    pub instructors: Vec<UserId>,
    pub graders: Vec<UserId>,
    pub sections: Vec<CourseSection>,
}

pub type AssignmentId = ObjectId;

#[derive(Debug, Serialize, Deserialize, CollectionIdentity)]
#[db(name = "assignment")]
pub struct Assignment {
    #[db(native_id_field)]
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<AssignmentId>,
    pub name: String,
    pub course: CourseSectionId,
    pub last_ast: Option<Vec<f64>>,
}

pub type SubmissionId = ObjectId;

#[derive(Debug, Serialize, Deserialize, CollectionIdentity)]
#[db(name = "submission")]
pub struct Submission {
    #[db(native_id_field)]
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<SubmissionId>,
    pub assignment: AssignmentId,
    pub author: String,
}