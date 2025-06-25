use std::fmt::Display;

use clap::Parser;
use eyre::{Ok, Result};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use regex::Regex;

const PROMPT: &str = include_str!("prompt.txt");

struct GenerationOutput {
    codes: Vec<String>,
    topic: String,
}

#[derive(Debug, Clone, Copy)]
enum ContentLength {
    Short,
    Medium,
    Long,
}

impl Display for ContentLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            ContentLength::Long => "long",
            ContentLength::Short => "short",
            ContentLength::Medium => "medium",
        };

        f.write_str(res)
    }
}

fn generate_prompt(content_length: ContentLength, banned_topics: &[String]) -> String {
    let mut prompt = PROMPT.replace("{{topic_length}}", &format!("{content_length}"));

    prompt = format!("{prompt}\n{}", banned_topics.join("\n"));

    prompt
}

async fn generate_code(
    ollama: &Ollama,
    model_name: String,
    banned_topics: &[String],
    content_length: ContentLength,
) -> Result<GenerationOutput> {
    let response = ollama
        .generate(GenerationRequest::new(
            model_name,
            generate_prompt(content_length, banned_topics),
        ))
        .await?;
    println!("{}", response.response);

    let mut response = response.response;

    let topic_regex = Regex::new(r"<topic>([^<]+)</topic>").unwrap();
    let topic = topic_regex.captures(&response).unwrap()[1].to_string();

    response = topic_regex.replace_all(&response, "").to_string();

    Ok(GenerationOutput {
        topic,
        codes: vec![],
    })
}

#[derive(Debug, clap::Parser)]
struct Args {
    model_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let ollama = Ollama::default();
    generate_code(&ollama, args.model_name, &[], ContentLength::Medium).await?;
    println!("Hello, world!");

    Ok(())
}
