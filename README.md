# Wellesley High School Discord Help Server Administration Bot
WHS Help Bot for short

# Table of contents
* [Tutorial](#Tutorial)
    * [Using The Bot](#Using-The-Bot)
        * [Command Syntax](#Command-Syntax)
        * [The Help Command](#The-Help-Command)
            * [Listing All Commands](#Listing-All-Commands)
            * [Viewing Information About A Command](#Viewing-Information-About-A-Command)
    * [Student Commands](#Student-Commands)
        * [Joining Classes](#Joining-Classes)
        * [Leaving Classes](#Leaving-Classes)
        * [Viewing Your Classes](#Viewing-Your-Classes)
        * [Viewing All Classes](#Viewing-All-Classes)
    * [Admin Commands](#Admin-Commands)
* [FAQ](#FAQ)
* [Support](#Support)
* [License](#License)

# Tutorial
This tutorial is aimed to help people get started with the use of the WHS help bot.
Please read each section carefully and feel free to submit a pull request (a request for changes)
[here](https://github.com/DusterTheFirst/WHS-Help-Bot/edit/master/README.md)
if you have the ability to do so and have found a mistake in the documentation.

With that out of the way let us get started.

## Using The Bot
To start using the bot, the first thing you need to understand is how to send
commands.

### Command Syntax
> You may have to read through the entirety of the tutorial a few times before it starts to make any sense

The syntax to send a command is:

    !<command> [..subcommand] [..options]

If this makes no sense to you, let me break it down. This type of syntax will be used much
more throughout the tutorial.

Anything in these "triangular brakets" (`<>`) is **required**.
The text inside of them tell you what would go in place of them.
On the other hand, anything surrounded in "square brackets" (`[]`) is optional.
The `..` in front of a placeholder tells you that there can me one or more of
them, as you will see later in the tutorial.

As an example, let us say we have a bot with a prefix `!` and we are trying to run the command `help`.
To run this command we would type

    !help

Let us break this apart, The `!` tells the bot the command it needs to carry out
and the `help` where the `<command>` was. The command has no subcommands nor
arguments so we can ignore those.

That was simple right? Well lets do another. We now have the sub command
`join` in the category `classes`. This command also takes a single argument
`class`. In order to run this command, first we need to write it out.
To start out, you write the prefix `!`, next you can add the command (or categoy)
name. So far we have:

    !classes

Next we need to add the subcommand, `join`.
In order to tell the bot that we are running a subcommand, we need to add a space and then
the name leving us with:

    !classes join

If we were to run this command right now, it would give us an error:

    This command requires 1 argument(s) while it was only given 0

This would give an inkling as to what we forgot: the class we want to join. To add the class,
we can just add a space and the class we want to join, for example `ACP History`. Doing so
would leave us with:

    !classes join ACP History

But this would **not** work. We would be given a different error this time:

    The command can only take up to 1 argument(s), but was given 2 argument(s).

This tells us that we gave it 2 arguments, but we only gave it one, didnt we? Well not exactly,
the bot is not smart enough to tell that `ACP History` is one argument it sees it as 2: `ACP` and
`History`. In order to tell it that it is one argument just with a space in it we can surround it
in qoutation marks (`""`). Our last and final iteration of the command would be

    !classes join "ACP History"

At this point, if you were to send this message, the bot would add you into the `ACP History` class
and you would be all done.

If you wanted to know more about another command, or to understand its syntax, you can do so with the
help command.

### Using The Help Command
The first command that was explained was the `help` command this command will be the backbone of you
learning to use the bot to its fullest.

This command can be used in 3 ways.
1. List out all of the commands that you can run
2. View more information about a command group
3. View more information about a subcommand

The last 2 are very simalar so they will be explained together. But first lets explain the first use
of the command, to list all of the commands that you can run

#### Listing All Commands
To list all of the commands that you can run you can run the command

    !help

This command will cause the bot to send an embed (A cool formatted block of text) which will have multiple
columns of text. These columns may look similar to the image below.

![Help Embed](!!URL!!)

These columns tell you the group that the command is in, these are not neccicarally important, they are more
for users to better understand the use of the command. What is more important is the line below the group name,
if it exists. The line would start with `Prefix: .., .., ..` Where the `..` are replaced with prefixes for the
commands. In the previous example, the `Prefix:` section for the add command would look something like
`Prefix: classes, c, cl`. The command could also be used with the prefix `c` or `cl` since they are also in the list.
These shortened prefixes allow for quicker typing of commands; `!c join` rather than `!classes join`. These prefixes
are mandatory for the command to run correctly, for it is a different command than juts `!join`

Viewing all of the commands is nice, but sometimes you would like to see more information on the specific usage of a
command you can use the second form of the help command.

#### Viewing Information About A Command
In order to view the usage or other information about a command, you could use the `help` command and provide the arguments.
The argument layout for the `help` command is

    !help [..command|subcommand]

# FAQ

# Support
If you have found an issue with the bot, please report it to @DusterTheFirst
on discord or if you have a GitHub accont, please file an issue
[here](https://github.com/DusterTheFirst/WHS-Help-Bot/issues).

If you need assistance in using the bot, feel free to ask @DusterTheFirst or other
moderators in the server

# License
        A bot designed for the WHS help server
        Copyright (C) 2019  Zachary Kohnen

        This program is free software: you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation, either version 3 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program.  If not, see <https://www.gnu.org/licenses/>.