use serenity::framework::standard::macros::group; 

group!({
    name: "Groups",
    options: {
        description: "Group management commands",
        prefixes: ["groups", "g", "gr"],
        // default_command: list
    },
    commands: [/* list, add, remove, edit, mine, join, leave */],
});