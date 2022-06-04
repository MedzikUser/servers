use async_trait::async_trait;

use crate::{command_handler::Command, CommandManagerType};

pub struct CommandHelp;

#[async_trait]
impl Command for CommandHelp {
    fn name(&self) -> &'static str {
        "/help"
    }

    fn help(&self) -> &'static str {
        "show help"
    }

    async fn execute(
        &self,
        client: &mut crate::Client,
        _args: Vec<&str>,
        commands: &CommandManagerType,
    ) {
        for command in commands {
            client
                .send(&format!("{} - {}", command.name(), command.help()))
                .await
                .expect("send message");
        }
    }
}