use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[owners_only]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx, args.rest())?;
    msg.delete(&ctx)?;

    Ok(())
}
