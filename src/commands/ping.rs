use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
  "".to_string()
}

pub fn register() -> CreateCommand {
  CreateCommand::new("ping").description("First discord bot command")
}