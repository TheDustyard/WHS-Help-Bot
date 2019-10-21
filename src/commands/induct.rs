use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[owners_only]
#[num_args(1)]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx, "NOT IMPLEMENTED YET")?;

    Ok(())
}
