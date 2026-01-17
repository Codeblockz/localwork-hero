use std::path::Path;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel, Special};
use llama_cpp_2::sampling::LlamaSampler;
use thiserror::Error;

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

    /// Generate a response for the given prompt
    pub fn generate(&self, prompt: &str, max_tokens: u32) -> Result<String, InferenceError> {
        let model = self.model.as_ref().ok_or(InferenceError::ModelNotLoaded)?;

        // Create context with reasonable defaults
        let ctx_params = LlamaContextParams::default();

        let mut ctx = model
            .new_context(&self.backend, ctx_params)
            .map_err(|e| InferenceError::ContextError(e.to_string()))?;

        // Format as chat prompt
        let formatted_prompt = format!(
            "<|im_start|>system\nYou are a helpful AI assistant running locally on the user's computer. Be concise and helpful.<|im_end|>\n<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n",
            prompt
        );

        // Tokenize the prompt
        let tokens = model
            .str_to_token(&formatted_prompt, AddBos::Always)
            .map_err(|e| InferenceError::TokenizeError(e.to_string()))?;

        // Create batch and add tokens
        let mut batch = LlamaBatch::new(512, 1);
        for (i, token) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            batch
                .add(*token, i as i32, &[0], is_last)
                .map_err(|e| InferenceError::InferenceError(e.to_string()))?;
        }

        // Process prompt
        ctx.decode(&mut batch)
            .map_err(|e| InferenceError::InferenceError(e.to_string()))?;

        // Generate tokens
        let mut sampler = LlamaSampler::greedy();
        let mut output = String::new();
        let mut n_cur = batch.n_tokens();

        for _ in 0..max_tokens {
            // Sample next token
            let new_token = sampler.sample(&ctx, n_cur - 1);
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
}
