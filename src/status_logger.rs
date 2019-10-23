use log::{error, info, trace, warn};
use serenity::{
    http::Http, model::channel::Message, model::id::ChannelId, utils::Colour,
    Result as SerenityResult,
};
use std::fmt::Display;

pub struct StatusLogger {
    channel: ChannelId,
}

impl StatusLogger {
    pub fn new(channel: ChannelId) -> StatusLogger {
        StatusLogger { channel }
    }

    pub fn info(&self, ctx: impl AsRef<Http>, title: impl Display, message: impl Display) -> SerenityResult<()> {
        self.channel.send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Colour::DARKER_GREY)
                    .title(&title)
                    .description(&message)
            })
        })?;

        info!("[{}] {}", title, message);

        Ok(())
    }

    pub fn success() {}

    pub fn warn(&self, ctx: impl AsRef<Http>, title: impl Display, message: impl Display) -> SerenityResult<()> {
        self.channel.send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Colour::GOLD)
                    .title(&title)
                    .description(&message)
            })
        })?;

        warn!("[{}] {}", title, message);

        Ok(())
    }

    pub fn error() {}
}
