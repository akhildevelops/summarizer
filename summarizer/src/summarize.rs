use crate::default;
use crate::error::Serror;
use crate::tokenize::{OpenAI, Tokenizer};
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::future::join_all;
use log;

pub trait Summarize {
    fn description(&self) -> &str;
}

impl Summarize for String {
    fn description(&self) -> &str {
        &self
    }
}

#[derive(Clone)]
pub struct Summarizer {
    openai: OpenAI,
    client: Client,
}

impl Summarizer {
    pub fn default_params() -> Result<Self, Serror> {
        log::info!("Initializing Summarizer from default params");
        Ok(Self {
            openai: OpenAI::default_params()?,
            client: Client::new(),
        })
    }
}

impl Summarizer {
    pub async fn summarize(&self, x: &impl Summarize) -> Result<String, Serror> {
        log::info!("Summarizing the Content");
        let client = &self.client;
        let chat = client.chat();
        let mut interm: String;
        let mut content = x.description();
        log::debug!("Length of content: {}", content.len());
        loop {
            log::debug!(
                "Breaking the content into segments with a max token limit of {}",
                OpenAI::MAX_N_TOKENS
            );
            let segments = self
                .openai
                .tokenize_in_max_tokenlimit(content)?
                .detokenize_inarray()?;
            log::debug!("Got {} segments", segments.len());

            log::debug!("Creating Chat request for each segment");
            let all_requests: Vec<Result<_, Serror>> = segments
                .iter()
                .map(|x| {
                    let request = CreateChatCompletionRequestArgs::default()
                        .model(default::GPT_MODEL)
                        .messages([
                            ChatCompletionRequestMessageArgs::default()
                                .role(Role::System)
                                .content(default::SYSTEM_PROMPT)
                                .build()?,
                            ChatCompletionRequestMessageArgs::default()
                                .role(Role::User)
                                .content(x.description())
                                .build()?,
                        ])
                        .build()?;
                    Ok(chat.create(request))
                })
                .collect::<Vec<_>>();
            if all_requests.len() == 1 {
                log::debug!("Got only one segment!, summarizing for that content");
                return Ok(all_requests
                    .into_iter()
                    .next()
                    .ok_or(Serror::Other("cannot find first request".to_string()))??
                    .await?
                    .choices
                    .into_iter()
                    .next()
                    .ok_or(Serror::Other("cannot find summary".to_string()))?
                    .message
                    .content);
            } else {
                log::debug!(
                    "For the {} segments, summarizing individually and finally merging them to a single string.",
                    all_requests.len()
                );
                let futures = all_requests.into_iter().filter_map(|x| x.ok());
                interm = join_all(futures)
                    .await
                    .into_iter()
                    .filter_map(|x| {
                        let x = x.unwrap().choices.into_iter().next()?.message.content;
                        Some(x)
                    })
                    .collect::<String>();
                content = &interm;
                log::debug!("Looping again with the content to be this summarized content")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Youtube;
    use env_logger;
    fn log_init() {
        env_logger::builder().is_test(true).try_init().unwrap();
    }
    struct DUMMY;
    impl Summarize for DUMMY {
        fn description(&self) -> &str {
            "The thread initiating the shutdown blocks until all spawned work has been stopped. This can take an indefinite amount of time. The Drop implementation waits forever for this.
            
            shutdown_background and shutdown_timeout can be used if waiting forever is undesired. When the timeout is reached, spawned work that did not stop in time and threads running it are leaked. The work continues to run until one of the stopping conditions is fulfilled, but the thread initiating the shutdown is unblocked.
            
            Once the runtime has been dropped, any outstanding I/O resources bound to it will no longer function. Calling any method on them will result in an error."
        }
    }
    #[tokio::test]
    #[ignore = "Requires mocking openai response"]
    async fn summarize() {
        log_init();
        let summarizer = Summarizer::default_params().unwrap();
        let resp = summarizer.summarize(&DUMMY).await.unwrap();
        println!("{resp}")
    }

    #[tokio::test]
    #[ignore = "Requires mocking openai response"]
    async fn summarize_youtube_small() {
        log_init();
        let content = Youtube::link("https://www.youtube.com/watch?v=WYNRt-AwoUg")
            .content()
            .await
            .unwrap();
        let summarizer = Summarizer::default_params().unwrap();
        let resp = summarizer.summarize(&content).await.unwrap();
        println!("{resp}")
    }

    #[tokio::test]
    #[ignore = "Requires mocking openai response"]
    async fn summarize_youtube_1hr() {
        log_init();
        let content = Youtube::link("https://www.youtube.com/watch?v=sBH-ngpL0zo")
            .content()
            .await
            .unwrap();
        let summarizer = Summarizer::default_params().unwrap();
        let resp = summarizer.summarize(&content).await.unwrap();
        println!("{resp}")
    }
}
