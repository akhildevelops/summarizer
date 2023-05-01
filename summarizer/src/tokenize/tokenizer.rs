use super::token::Tokenizer;
use crate::error::Serror;
use log;
use tiktoken_rs::{cl100k_base, CoreBPE};
#[derive(Clone)]
pub struct OpenAI {
    tokenizer: CoreBPE,
}

impl OpenAI {
    pub fn default_params() -> Result<Self, Serror> {
        log::info!("Initializing OpenAI from default params");
        log::debug!("Using cl100k_base as the tokenizer");
        let bpe = cl100k_base()?;
        Ok(Self { tokenizer: bpe })
    }
}

impl Tokenizer for OpenAI {
    const MAX_N_TOKENS: usize = 2000;
    fn bpe(&self) -> &CoreBPE {
        &self.tokenizer
    }
}
