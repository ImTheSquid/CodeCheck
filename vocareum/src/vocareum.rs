//This program contains functions:
//Fetch courses from user vocareum tokens
//Fetch assignments from user's courses
//Fetch students from courses
//Fetch students submissions from course_id and assignment_id
//Vinh version

[dependencies]
reqwest = "0.11"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Course {
    id: u32,
    name: String,
    // Add other relevant fields
}

#[derive(Serialize, Deserialize, Debug)]
struct Assignment {
    id: u32,
    title: String,
    // Add other relevant fields
}

#[derive(Serialize, Deserialize, Debug)]
struct Student {
    id: u32,
    name: String,
    // Add other relevant fields
}

#[derive(Serialize, Deserialize, Debug)]
struct Submission {
    student_id: u32,
    assignment_id: u32,
    grade: String,
    // Add other relevant fields
}
async fn fetch_courses(token: &str) -> Result<Vec<Course>, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.example.com/courses")
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<Course>>()
        .await?;
    Ok(resp)
}


async fn fetch_assignments(token: &str, course_id: u32) -> Result<Vec<Assignment>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.example.com/courses/{}/assignments", course_id);
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<Assignment>>()
        .await?;
    Ok(resp)
}

async fn fetch_students(token: &str, course_id: u32) -> Result<Vec<Student>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.example.com/courses/{}/students", course_id);
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<Student>>()
        .await?;
    Ok(resp)
}

async fn fetch_submissions(token: &str, course_id: u32, assignment_id: u32) -> Result<Vec<Submission>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.example.com/courses/{}/assignments/{}/submissions", course_id, assignment_id);
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<Submission>>()
        .await?;
    Ok(resp)
}


