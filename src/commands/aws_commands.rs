use serenity::builder::{CreateCommand, CreateEmbed, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::model::Timestamp;

// use aws_config;
// use aws_sdk_ec2;

// struct Aws {
//   pub region: String,
//   pub command: String,
//   pub action: String,
//   pub number: i8,
// }

// impl Aws {
//   async fn ec2_list() {
    
//   }
// }

// struct AwsClient {
//   pub config: aws_config::ConfigLoader,
//   pub instance: aws_sdk_ec2::Client,
// }

// pub fn response(_options: &[ResolvedOption]) {
//   let embed = CreateEmbed::new().title("").description("description")
//       .fields(vec![
//         ("", "field", true),
//         ("", "field", true),
//       ])
//       .timestamp(Timestamp::now());
// }

// Feture layout : Maincommand -> Subcommandgroup -> Subcommand -> Options
pub fn request() -> CreateCommand {
  CreateCommand::new("aws").description("AWS service access control")
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "region", "Current Region"))
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "ec2", "EC2 Instance Command")
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "list", "Instance list")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "filter", "Instance scale filter")
        .add_string_choice("all", "All Instance")
        .add_string_choice("running", "Running Instance")
        .add_string_choice("stopped", "Stopped Instance")
        .add_string_choice("terminated", "Terminated Instance")
        .required(true)
      )
    )
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "status", "Instance Status")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::Number, "instance_number", "Instance Number")
        .required(true)
      )
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "actions", "Instance Actions")
        .add_string_choice("start", "Start Instance")
        .add_string_choice("stop", "Stop Instance")
        .add_string_choice("reboot", "Reboot Instance")
        .add_string_choice("terminate", "Terminate Instance")
        .required(true)
      )
    )
  )
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "s3", "S3 Bucket Command")
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "list", "S3 Bucket list"))
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "create", "S3 Bucket Create")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "bucketname", "S3 Bucket name create").required(true))
    )
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "update", "S3 Bucket update")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "bucketname", "S3 Bucket name to update").required(true))
    )
    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommand, "delete", "S3 Bucket delete")
      .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "bucketname", "S3 Bucket name to delete").required(true))
    )
  )
  .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "ecr", "ECR Registry Command")
  )
}