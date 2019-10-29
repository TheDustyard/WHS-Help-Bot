use serenity::framework::standard::macros::group;

mod classes;
mod groups;
mod help;
mod say;

use super::checks::ADMIN_CHECK;
use say::SAY_COMMAND;

pub use classes::CLASSES_GROUP;
pub use groups::GROUPS_GROUP;
pub use help::HELP_COMMAND;

group!({
    name: "Admin",
    options: {
        description: "Administrative commands",
        checks: [Admin],
    },
    commands: [say],
});
