use serenity::framework::standard::{macros::{command}, CommandError, CommandResult};

#[command]
#[description = "Register yourself"]
#[only_in(dms)]
pub fn register() -> CommandResult {
    Err(CommandError("Unimplemented".to_owned()))
}