use crate::default;
use crate::error::Serror;
use crate::tokenize::{OpenAI, Tokenizer};
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::future::join_all;

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
        Ok(Self {
            openai: OpenAI::default_params()?,
            client: Client::new(),
        })
    }
}

impl Summarizer {
    pub async fn summarize(&self, x: &impl Summarize) -> Result<String, Serror> {
        let client = &self.client;
        let chat = client.chat();
        let mut interm: String;
        let mut content = x.description();
        loop {
            let segments = self
                .openai
                .tokenize_in_max_tokenlimit(content)?
                .detokenize_inarray()?;
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
                let futures = all_requests.into_iter().filter_map(|x| x.ok());
                interm = join_all(futures)
                    .await
                    .into_iter()
                    // .filter_map(|x| Some(x.ok()?.choices.into_iter().next()?.message.content))
                    .filter_map(|x| Some(x.unwrap().choices.into_iter().next()?.message.content))
                    .collect::<String>();
                content = &interm;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Youtube;
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
        let summarizer = Summarizer::default_params().unwrap();
        let resp = summarizer.summarize(&DUMMY).await.unwrap();
        println!("{resp}")
    }

    #[tokio::test]
    #[ignore = "Requires mocking openai response"]
    async fn summarize_youtube_small() {
        let content = Youtube::link("https://www.youtube.com/watch?v=GJLlxj_dtq8")
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
        let content = Youtube::link("https://www.youtube.com/watch?v=sBH-ngpL0zo")
            .content()
            .await
            .unwrap();
        let summarizer = Summarizer::default_params().unwrap();
        let resp = summarizer.summarize(&content).await.unwrap();
        println!("{resp}")
    }
}
