use serenity::{
    framework::{standard::StandardFramework, Framework},
    model::channel::Message,
    prelude::*,
};
use threadpool::ThreadPool;

pub struct StandardFrameworkWrapper {
    framework: StandardFramework,
}
impl StandardFrameworkWrapper {
    pub fn wrap(framework: StandardFramework) -> StandardFrameworkWrapper {
        StandardFrameworkWrapper { framework }
    }
}
impl Framework for StandardFrameworkWrapper {
    fn dispatch(&mut self, ctx: Context, mut msg: Message, threadpool: &ThreadPool) {
        msg.content = msg
            .content
            .chars()
            .map(|c| match c {
                '“' | '‟' | '”' => '"',
                _ => c,
            })
            .collect();

        self.framework.dispatch(ctx, msg, threadpool);
    }
}
