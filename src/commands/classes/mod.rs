use serenity::framework::standard::macros::group;

mod list;
mod add;
mod remove;
mod edit;
mod mine;
mod join;
mod leave;

use list::LIST_COMMAND;
use add::ADD_COMMAND;
use remove::REMOVE_COMMAND;
use edit::EDIT_COMMAND;
use mine::MINE_COMMAND;
use join::JOIN_COMMAND;
use leave::LEAVE_COMMAND;

group!({
    name: "Class",
    options: {
        description: "Class management commands",
        prefixes: ["classes", "c", "cl"],
        default_command: list
    },
    commands: [list, add, remove, edit, mine, join, leave],
});