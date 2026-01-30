use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

const CLOUDFLARE_GATEWAY_URL: &str = 
    "https://gateway.ai.cloudflare.com/v1/2d4b81ed42312401410d8ab4cd8c5dcf/northstars-industries/compat/chat/completions";

#[derive(Debug, Clone)]
pub struct DeepSeekClient {
    client: Client,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

impl DeepSeekClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub fn from_env() -> Option<Self> {
        std::env::var("CLOUDFLARE_AI_TOKEN")
            .ok()
            .map(|key| Self::new(key))
    }

    pub fn with_default_key() -> Self {
        Self::new("75pX0slf0zE2EF6Kf0H-MjauYQosat8-wzqXP0eF".to_string())
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let request = ChatRequest {
            model: "deepseek/deepseek-chat".to_string(),
            messages,
            stream: false,
        };

        let response = self.client
            .post(CLOUDFLARE_GATEWAY_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, text);
        }

        let chat_response: ChatResponse = response.json().await?;
        
        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from AI"))
    }

    pub fn get_system_prompt() -> ChatMessage {
        ChatMessage {
            role: "system".to_string(),
            content: r#"You are QHub, an AI assistant specialized in quantum computing. 
You help users design and implement quantum algorithms and circuits.

When a user describes a computation they want to perform:
1. Explain what quantum approach would be suitable
2. Generate Python code using Qiskit that implements the quantum circuit
3. Explain the expected output/results

Keep responses concise but informative. Use code blocks with ```python for code.
Focus on practical, runnable quantum circuits for IBM Quantum backends."#.to_string(),
        }
    }
}
