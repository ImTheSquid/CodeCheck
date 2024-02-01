#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum JobType {
    Ast,
    StringTokenization,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Job {
    assignment_id: String,
    ty: JobType,
    flag_threshold: f64,
}

pub struct JobManager {
    
}

pub async fn job_main(data: Job) {
    match data.ty {
        JobType::Ast => ast_main(data.assignment_id, data.flag_threshold).await,
        JobType::StringTokenization => string_tokenization_main().await,
    }
}

pub async fn ast_main(assignment_id: String, flag_threshold: f64) {
    todo!()
}

pub async fn string_tokenization_main() {
    todo!()
}
