use std::path::Path;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel, Special};
use llama_cpp_2::sampling::LlamaSampler;
use thiserror::Error;

/// A message in the conversation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Error)]
pub enum InferenceError {
    #[error("Model not loaded")]
    ModelNotLoaded,
    #[error("Failed to initialize backend: {0}")]
    BackendInitError(String),
    #[error("Failed to load model: {0}")]
    ModelLoadError(String),
    #[error("Failed to create context: {0}")]
    ContextError(String),
    #[error("Failed to tokenize: {0}")]
    TokenizeError(String),
    #[error("Failed during inference: {0}")]
    InferenceError(String),
}

pub struct LlamaInference {
    backend: LlamaBackend,
    model: Option<LlamaModel>,
}

impl LlamaInference {
    /// Initialize the llama.cpp backend
    pub fn new() -> Result<Self, InferenceError> {
        let backend =
            LlamaBackend::init().map_err(|e| InferenceError::BackendInitError(e.to_string()))?;

        Ok(Self {
            backend,
            model: None,
        })
    }

    /// Load a model from a file path
    pub fn load_model(&mut self, path: &Path) -> Result<(), InferenceError> {
        let model_params = LlamaModelParams::default();

        let model = LlamaModel::load_from_file(&self.backend, path, &model_params)
            .map_err(|e| InferenceError::ModelLoadError(e.to_string()))?;

        self.model = Some(model);
        Ok(())
    }

    /// Check if a model is loaded
    pub fn is_model_loaded(&self) -> bool {
        self.model.is_some()
    }

    /// Generate a response given the conversation history
    pub fn generate(&self, messages: &[Message], max_tokens: u32) -> Result<String, InferenceError> {
        let model = self.model.as_ref().ok_or(InferenceError::ModelNotLoaded)?;

        // Configure context with explicit parameters for Qwen2.5 models
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(std::num::NonZeroU32::new(2048))
            .with_n_batch(512);

        let mut ctx = model
            .new_context(&self.backend, ctx_params)
            .map_err(|e| InferenceError::ContextError(e.to_string()))?;

        // Format conversation history as chat prompt using ChatML format
        let mut formatted_prompt = String::from(
            "<|im_start|>system\nYou are a helpful AI assistant running locally on the user's computer. Be concise and helpful.<|im_end|>\n"
        );

        for msg in messages {
            formatted_prompt.push_str(&format!(
                "<|im_start|>{}\n{}<|im_end|>\n",
                msg.role, msg.content
            ));
        }

        // Add the assistant turn start
        formatted_prompt.push_str("<|im_start|>assistant\n");

        // Tokenize the prompt
        let tokens = model
            .str_to_token(&formatted_prompt, AddBos::Always)
            .map_err(|e| InferenceError::TokenizeError(e.to_string()))?;

        // Use dynamic batch size to accommodate longer conversations
        let batch_size = std::cmp::max(1024, tokens.len() + 256);
        let mut batch = LlamaBatch::new(batch_size, 1);
        for (i, token) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            batch
                .add(*token, i as i32, &[0], is_last)
                .map_err(|e| InferenceError::InferenceError(e.to_string()))?;
        }

        // Process prompt
        ctx.decode(&mut batch)
            .map_err(|e| InferenceError::InferenceError(e.to_string()))?;

        // Generate tokens - use chain_simple with dist + greedy as per official examples
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::dist(1234),
            LlamaSampler::greedy(),
        ]);
        let mut output = String::new();
        let mut n_cur = batch.n_tokens();

        for _ in 0..max_tokens {
            // Safety check: ensure batch has tokens before sampling
            let n_tokens = batch.n_tokens();
            if n_tokens == 0 {
                return Err(InferenceError::InferenceError("Batch is empty".to_string()));
            }
            let new_token = sampler.sample(&ctx, n_tokens - 1);
            sampler.accept(new_token);

            // Check for end-of-generation
            if model.is_eog_token(new_token) {
                break;
            }

            // Convert token to string
            if let Ok(token_str) = model.token_to_str(new_token, Special::Tokenize) {
                // Stop if we hit the end-of-turn marker
                if token_str.contains("<|im_end|>") {
                    break;
                }
                output.push_str(&token_str);
            }

            // Add token to batch for next iteration
            batch.clear();
            batch
                .add(new_token, n_cur, &[0], true)
                .map_err(|e| InferenceError::InferenceError(e.to_string()))?;

            n_cur += 1;

            // Decode
            ctx.decode(&mut batch)
                .map_err(|e| InferenceError::InferenceError(e.to_string()))?;
        }

        Ok(output.trim().to_string())
    }

    /// Generate a response with tool definitions in the system prompt
    pub fn generate_with_tools(
        &self,
        messages: &[Message],
        tool_definitions: &str,
        max_tokens: u32,
    ) -> Result<String, InferenceError> {
        let model = self.model.as_ref().ok_or(InferenceError::ModelNotLoaded)?;

        // Configure context with explicit parameters - larger for tool definitions
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(std::num::NonZeroU32::new(4096))
            .with_n_batch(512);

        let mut ctx = model
            .new_context(&self.backend, ctx_params)
            .map_err(|e| InferenceError::ContextError(e.to_string()))?;

        // Format with tool-aware system prompt
        let mut formatted_prompt = format!(
            "<|im_start|>system\nYou are a helpful AI assistant running locally on the user's computer. Be concise and helpful.\n\n{}<|im_end|>\n",
            tool_definitions
        );

        for msg in messages {
            formatted_prompt.push_str(&format!(
                "<|im_start|>{}\n{}<|im_end|>\n",
                msg.role, msg.content
            ));
        }

        formatted_prompt.push_str("<|im_start|>assistant\n");

        let tokens = model
            .str_to_token(&formatted_prompt, AddBos::Always)
            .map_err(|e| InferenceError::TokenizeError(e.to_string()))?;

        // Use batch size that accommodates the prompt (tool definitions can be large)
        let batch_size = std::cmp::max(2048, tokens.len() + 512);
        let mut batch = LlamaBatch::new(batch_size, 1);
        for (i, token) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            batch
                .add(*token, i as i32, &[0], is_last)
                .map_err(|e| InferenceError::InferenceError(e.to_string()))?;
        }

        ctx.decode(&mut batch)
            .map_err(|e| InferenceError::InferenceError(e.to_string()))?;

        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::dist(1234),
            LlamaSampler::greedy(),
        ]);
        let mut output = String::new();
        let mut n_cur = batch.n_tokens();

        for _ in 0..max_tokens {
            // Safety check: ensure batch has tokens before sampling
            let n_tokens = batch.n_tokens();
            if n_tokens == 0 {
                return Err(InferenceError::InferenceError("Batch is empty".to_string()));
            }
            let new_token = sampler.sample(&ctx, n_tokens - 1);
            sampler.accept(new_token);

            if model.is_eog_token(new_token) {
                break;
            }

            if let Ok(token_str) = model.token_to_str(new_token, Special::Tokenize) {
                if token_str.contains("<|im_end|>") {
                    break;
                }
                output.push_str(&token_str);
            }

            batch.clear();
            batch
                .add(new_token, n_cur, &[0], true)
                .map_err(|e| InferenceError::InferenceError(e.to_string()))?;

            n_cur += 1;

            ctx.decode(&mut batch)
                .map_err(|e| InferenceError::InferenceError(e.to_string()))?;
        }

        Ok(output.trim().to_string())
    }
}
