use crate::default;
use crate::error::Serror;
use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs,
        CreateChatCompletionResponse, Role,
    },
    Client,
};

pub trait Summarize {
    fn description(&self) -> &str;
}

pub struct Summarizer;

impl Summarizer {
    pub async fn summarize(x: &impl Summarize) -> Result<CreateChatCompletionResponse, Serror> {
        let client = Client::new();
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
        Ok(client.chat().create(request).await?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
        let resp = Summarizer::summarize(&DUMMY).await.unwrap();
        let content = resp.choices.into_iter().next().unwrap().message.content;
        println!("{content}")
    }
}
