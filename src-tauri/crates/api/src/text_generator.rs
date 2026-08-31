use promoforge_core::*;
use crate::llm::LlmClient;

pub struct TextGenerator<'a> {
    llm: &'a LlmClient,
}

impl<'a> TextGenerator<'a> {
    pub fn new(llm: &'a LlmClient) -> Self {
        Self { llm }
    }

    pub async fn generate_social_posts(
        &self,
        pkb: &ProductKnowledgeBase,
        platform: &str,
        count: usize,
    ) -> Result<Vec<String>> {
        let mut posts = Vec::new();

        let messages = vec![
            LlmMessage {
                role: "user".into(),
                content: format!(
                    "Generate {} {} posts for {} ({}).\n\n\
                     Features: {}\n\
                     Target users: {}\n\
                     Brand voice: {:?}\n\n\
                     Requirements:\n\
                     - Platform-native formatting\n\
                     - Engaging hooks\n\
                     - Clear CTAs\n\
                     - Varied angles and tones\n\
                     - Each post should be unique\n\n\
                     Return each post separated by '---POST---'",
                    count,
                    platform,
                    pkb.identity.inferred_name,
                    pkb.identity.category,
                    pkb.features.iter().take(3).map(|f| f.name.as_str()).collect::<Vec<_>>().join(", "),
                    pkb.personas.iter().take(2).map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
                    pkb.brand.inferred_voice.tone
                ),
            },
        ];

        let resp = self.llm.complete(&messages, 2048).await?;

        let raw_posts: Vec<&str> = resp.content.split("---POST---").collect();
        for post in raw_posts {
            let trimmed = post.trim().to_string();
            if !trimmed.is_empty() {
                posts.push(trimmed);
            }
        }

        Ok(posts)
    }

    pub async fn generate_store_metadata(
        &self,
        pkb: &ProductKnowledgeBase,
        platform: &str,
    ) -> Result<serde_json::Value> {
        let messages = vec![
            LlmMessage {
                role: "user".into(),
                content: format!(
                    "Generate {} store listing metadata for {}.\n\n\
                     Category: {}\n\
                     Features: {}\n\
                     Target users: {}\n\n\
                     Return as JSON with:\n\
                     - title: keyword-optimized title\n\
                     - subtitle: benefit-focused subtitle\n\
                     - description: feature-rich description with AIDA framework\n\
                     - keywords: array of 10 relevant keywords/tags\n\
                     - short_description: 1-2 sentence summary",
                    platform,
                    pkb.identity.inferred_name,
                    pkb.identity.category,
                    pkb.features.iter().map(|f| f.name.as_str()).collect::<Vec<_>>().join(", "),
                    pkb.personas.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
                ),
            },
        ];

        let resp = self.llm.complete(&messages, 2048).await?;

        let metadata: serde_json::Value = serde_json::from_str(&resp.content)
            .unwrap_or_else(|_| {
                serde_json::json!({
                    "raw_response": resp.content
                })
            });

        Ok(metadata)
    }

    pub async fn generate_press_kit(
        &self,
        pkb: &ProductKnowledgeBase,
    ) -> Result<String> {
        let messages = vec![
            LlmMessage {
                role: "user".into(),
                content: format!(
                    "Create a comprehensive press kit for {}.\n\n\
                     Product: {} ({})\n\
                     Features: {}\n\
                     Tech Stack: {}\n\n\
                     Include:\n\
                     1. Boilerplate (3 lengths: 1-sentence, 1-paragraph, detailed)\n\
                     2. Fact sheet\n\
                     3. Key features list\n\
                     4. Screenshots/visual descriptions\n\
                     5. Contact information template\n\
                     6. Social media links template",
                    pkb.identity.inferred_name,
                    pkb.identity.inferred_name,
                    pkb.identity.category,
                    pkb.features.iter().map(|f| format!("- {}: {}", f.name, f.description)).collect::<Vec<_>>().join("\n"),
                    pkb.technical.tech_stack.join(", ")
                ),
            },
        ];

        let resp = self.llm.complete(&messages, 4096).await?;
        Ok(resp.content)
    }

    pub async fn generate_aeo_content(
        &self,
        pkb: &ProductKnowledgeBase,
    ) -> Result<serde_json::Value> {
        let messages = vec![
            LlmMessage {
                role: "user".into(),
                content: format!(
                    "Generate Answer Engine Optimization (AEO) content for {}.\n\n\
                     Product: {} ({})\n\
                     Features: {}\n\
                     Category: {}\n\n\
                     Create:\n\
                     1. llms.txt content (machine-readable product description)\n\
                     2. 10 FAQ questions and answers\n\
                     3. JSON-LD Product schema\n\
                     4. 3 competitor comparison points\n\n\
                     Return as JSON.",
                    pkb.identity.inferred_name,
                    pkb.identity.inferred_name,
                    pkb.identity.category,
                    pkb.features.iter().map(|f| f.name.as_str()).collect::<Vec<_>>().join(", "),
                    pkb.identity.category
                ),
            },
        ];

        let resp = self.llm.complete(&messages, 4096).await?;

        let content: serde_json::Value = serde_json::from_str(&resp.content)
            .unwrap_or_else(|_| serde_json::json!({"raw": resp.content}));

        Ok(content)
    }
}
