use rusqlite::{Connection, Result as SQLResult, Row, Statement, NO_PARAMS};
use serenity::model::id::{ChannelId, RoleId};
use crate::db::Migrateable;

static CATEGORY_COLUMNS: usize = 3;
#[derive(Debug)]
pub struct Category {
    /// The internal ID of the category
    pub id: u32,
    /// The name of the category, linked to the name of the channel_group
    pub name: String,
    /// The channel group to put all the channels into
    pub channel_group: ChannelId,
}

impl Migrateable for Category {
    fn migrate_up(conn: &Connection) -> SQLResult<()> {
        conn.execute("
            CREATE TABLE IF NOT EXISTS `category` (
                `id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                `name`	TEXT NOT NULL,
                `channel_group`	TEXT NOT NULL UNIQUE
            );
        ", NO_PARAMS)?;

        Ok(())
    }

    fn migrate_down(conn: &Connection) -> SQLResult<()> {
        conn.execute("
            DROP TABLE `category`
        ", NO_PARAMS)?;

        Ok(())
    }
}

impl Category {
    pub fn from_row(row: &Row) -> SQLResult<Category> {
        if row.column_count() != CATEGORY_COLUMNS {
            panic!(
                "The amount of columns provided did not match the amount required: {} is not {}",
                row.column_count(),
                CATEGORY_COLUMNS
            );
        }

        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            channel_group: ChannelId(row.get::<_, String>(2)?.parse::<u64>().unwrap()),
        })
    }
}
