use serenity::framework::standard::macros::group;

mod checks;
mod classes;
mod help;
mod say;

use checks::ADMIN_CHECK;
use say::SAY_COMMAND;

pub use classes::CLASS_GROUP;
pub use help::HELP_COMMAND;

group!({
    name: "Admin",
    options: {
        description: "Administrative commands",
        checks: [Admin],
    },
    commands: [say],
});
