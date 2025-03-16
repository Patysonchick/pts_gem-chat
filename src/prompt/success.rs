use crate::prompt::Part;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub candidates: Vec<Candidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: UsageMetadata,
    #[serde(rename = "modelVersion")]
    model_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Content,
    #[serde(rename = "finishReason")]
    finish_reason: String,
    index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageMetadata {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: usize,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: usize,
    #[serde(rename = "totalTokenCount")]
    total_token_count: usize,
    #[serde(rename = "promptTokensDetails")]
    prompt_tokens_details: Vec<PromptTokensDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTokensDetail {
    modality: String,
    #[serde(rename = "tokenCount")]
    token_count: usize,
}
