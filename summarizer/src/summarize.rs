use crate::default;
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use std::error::Error;
use tokio::runtime::Runtime;
pub(crate) trait Summarize {
    fn description(&self) -> &str;

    fn summarize(&self) -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        println!("{}", std::env::var("OPENAI_API_KEY")?);
        let request = CreateChatCompletionRequestArgs::default()
            .model(default::GPT_MODEL)
            .messages([
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::System)
                    .content(default::SYSTEM_PROMPT)
                    .build()?,
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(self.description())
                    .build()?,
            ])
            .build()?;
        let runtime = Runtime::new()?;
        let response = runtime.block_on(client.chat().create(request))?;
        Ok(format!("{:?}", response))
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
    #[test]
    fn summarize() {
        let op = DUMMY.summarize().unwrap();
        println!("{op}")
    }
}
