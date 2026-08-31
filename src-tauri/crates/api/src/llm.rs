use promoforge_core::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct LlmClient {
    http: Client,
    provider: String,
    api_key: String,
    model: String,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    usage: Option<AnthropicUsage>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<OpenAiMessage>,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
    usage: Option<OpenAiUsage>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiRespMessage,
}

#[derive(Deserialize)]
struct OpenAiRespMessage {
    content: String,
}

#[derive(Deserialize)]
struct OpenAiUsage {
    total_tokens: u32,
}

impl LlmClient {
    pub fn new(provider: &str, api_key: &str, model: &str) -> Self {
        Self {
            http: Client::new(),
            provider: provider.to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }

    pub async fn complete(
        &self,
        messages: &[LlmMessage],
        max_tokens: u32,
    ) -> Result<LlmResponse> {
        match self.provider.as_str() {
            "anthropic" => self.complete_anthropic(messages, max_tokens).await,
            "openai" => self.complete_openai(messages, max_tokens).await,
            _ => Err(Error::Llm(format!("Unsupported provider: {}", self.provider))),
        }
    }

    async fn complete_anthropic(
        &self,
        messages: &[LlmMessage],
        max_tokens: u32,
    ) -> Result<LlmResponse> {
        let anthropic_messages: Vec<AnthropicMessage> = messages
            .iter()
            .map(|m| AnthropicMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect();

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens,
            messages: anthropic_messages,
        };

        let response = self
            .http
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Llm(format!("Request failed: {}", e)))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| Error::Llm(format!("Failed to read response: {}", e)))?;

        if !status.is_success() {
            return Err(Error::Llm(format!("API error {}: {}", status, body)));
        }

        let resp: AnthropicResponse =
            serde_json::from_str(&body).map_err(|e| Error::Llm(format!("Parse error: {}", e)))?;

        let content = resp
            .content
            .into_iter()
            .next()
            .map(|c| c.text)
            .unwrap_or_default();

        let tokens = resp.usage.as_ref().map(|u| u.input_tokens + u.output_tokens).unwrap_or(0);

        Ok(LlmResponse {
            content,
            model: self.model.clone(),
            tokens_used: tokens,
            cost: 0.0, // TODO: calculate based on model pricing
        })
    }

    async fn complete_openai(
        &self,
        messages: &[LlmMessage],
        max_tokens: u32,
    ) -> Result<LlmResponse> {
        let openai_messages: Vec<OpenAiMessage> = messages
            .iter()
            .map(|m| OpenAiMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect();

        let request = OpenAiRequest {
            model: self.model.clone(),
            max_tokens,
            messages: openai_messages,
        };

        let response = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Llm(format!("Request failed: {}", e)))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| Error::Llm(format!("Failed to read response: {}", e)))?;

        if !status.is_success() {
            return Err(Error::Llm(format!("API error {}: {}", status, body)));
        }

        let resp: OpenAiResponse =
            serde_json::from_str(&body).map_err(|e| Error::Llm(format!("Parse error: {}", e)))?;

        let content = resp
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .unwrap_or_default();

        let tokens = resp.usage.map(|u| u.total_tokens).unwrap_or(0);

        Ok(LlmResponse {
            content,
            model: self.model.clone(),
            tokens_used: tokens,
            cost: 0.0,
        })
    }

    pub async fn analyze_code(
        &self,
        code: &str,
        file_path: &str,
        language: &str,
    ) -> Result<String> {
        let messages = vec![
            LlmMessage {
                role: "user".into(),
                content: format!(
                    "Analyze this {} code from {} and provide:\n1. A brief summary of what this module/file does\n2. Key features and capabilities it implements\n3. Any notable patterns or architecture decisions\n4. Marketing-relevant aspects (what would a user care about?)\n\nCode:\n```{}\n{}\n```",
                    language, file_path, language, code
                ),
            },
        ];

        let resp = self.complete(&messages, 1024).await?;
        Ok(resp.content)
    }

    pub async fn generate_positioning(
        &self,
        pkb: &ProductKnowledgeBase,
    ) -> Result<String> {
        let feature_list: Vec<&str> = pkb.features.iter().take(5).map(|f| f.name.as_str()).collect();
        let persona_list: Vec<&str> = pkb.personas.iter().take(3).map(|p| p.name.as_str()).collect();

        let messages = vec![
            LlmMessage {
                role: "user".into(),
                content: format!(
                    "Create a compelling positioning statement and messaging hierarchy for:\n\n\
                     Product: {} ({})\n\
                     Category: {}\n\
                     Features: {}\n\
                     Target Users: {}\n\
                     Tech Stack: {}\n\n\
                     Provide:\n\
                     1. Positioning statement (one paragraph)\n\
                     2. Primary value proposition\n\
                     3. 3 supporting messages\n\
                     4. 3 tagline options\n\
                     5. Elevator pitch (30 seconds)",
                    pkb.identity.inferred_name,
                    pkb.identity.category,
                    pkb.identity.category,
                    feature_list.join(", "),
                    persona_list.join(", "),
                    pkb.technical.tech_stack.join(", ")
                ),
            },
        ];

        let resp = self.complete(&messages, 2048).await?;
        Ok(resp.content)
    }
}
