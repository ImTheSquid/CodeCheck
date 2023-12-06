use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use goldleaf::CollectionIdentity;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub last_use: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, CollectionIdentity)]
#[db(name = "user")]
pub struct User {
    #[db(native_id_field)]
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<UserId>,
    #[db(indexing(index = 1, unique))]
    pub username: String,
    pub name: String,
    pub role: Role,
    #[cfg(feature = "basic_auth")]
    pub password: String,
    pub sessions: Vec<Session>,
    pub email: Option<VerifiedResource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedResource {
    pub resource: String,
    pub status: VerificationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationStatus {
    Unverified {
        token: String,
        expiration: DateTime<Utc>,
    },
    /// Resource is verified
    Verified,
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
    // TODO: Term string
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
    pub last_ast_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Span {
    start_line: usize,
    end_line: usize,
    file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpanPair {
    row: Span,
    column: Span,
    similarity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionCell {
    similarity: f64,
    span_pairs: Vec<SpanPair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionResult {
    matrix: Vec<DetectionCell>,
    size: usize,
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
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
}
