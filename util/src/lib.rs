use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct MarkSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct Mark {
    pub a: MarkSpan,
    pub b: MarkSpan,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Pair {
    pub a: String,
    pub b: String,
    pub marks: Vec<Mark>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Dataset {
    pub pairs: HashMap<usize, Pair>,
}
