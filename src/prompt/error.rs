use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "promptFeedback")]
    pub prompt_feedback: PromptFeedback,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: UsageMetadata,
    #[serde(rename = "modelVersion")]
    pub model_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptFeedback {
    #[serde(rename = "blockReason")]
    pub block_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageMetadata {
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: usize,
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: usize,
    #[serde(rename = "promptTokensDetails")]
    pub prompt_tokens_details: Vec<PromptTokensDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTokensDetail {
    pub modality: String,
    #[serde(rename = "tokenCount")]
    pub token_count: usize,
}
