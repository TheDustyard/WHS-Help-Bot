use serenity::framework::standard::macros::group; 

use crate::commands::ADMIN_CHECK;

group!({
    name: "Groups",
    options: {
        description: "Group management commands",
        prefixes: ["groups", "g", "gr"],
        // default_command: list
        checks: [Admin]
    },
    commands: [/* list, add, remove, edit, mine, join, leave */],
});