use crate::db::Migrateable;
use rusqlite::{Connection, Result as SQLResult, Row, Statement, NO_PARAMS};
use serenity::model::id::{ChannelId, RoleId};

#[derive(Debug)]
pub struct CategoryId(u32);

static CLASS_COLUMNS: usize = 5;
#[derive(Debug)]
pub struct Class {
    /// The internal ID to use for FKs
    pub id: u32,
    /// The name of the class, linked to the name of the role
    pub name: String,
    /// The role id to link this class to
    pub role: RoleId,
    /// The category that the class is in
    pub category: CategoryId,
    /// The channel to link this class to
    pub channel: ChannelId,
}

impl Migrateable for Class {
    fn migrate_up(conn: &Connection) -> SQLResult<()> {
        conn.execute("
            CREATE TABLE IF NOT EXISTS `class` (
                `id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                `name`	TEXT NOT NULL,
                `role`	TEXT NOT NULL UNIQUE,
                `category`	TEXT NOT NULL,
                `channel`	TEXT NOT NULL UNIQUE,
                FOREIGN KEY(`category`)
                REFERENCES `category`(`id`)
                    ON UPDATE CASCADE
                    ON DELETE CASCADE
            );
        ", NO_PARAMS)?;

        Ok(())
    }

    fn migrate_down(conn: &Connection) -> SQLResult<()> {
        conn.execute("
            DROP TABLE `class`
        ", NO_PARAMS)?;

        Ok(())
    }
}

impl Class {
    pub fn from_row(row: &Row) -> SQLResult<Class> {
        if row.column_count() != CLASS_COLUMNS {
            panic!(
                "The amount of columns provided did not match the amount required: {} is not {}",
                row.column_count(),
                CLASS_COLUMNS
            );
        }

        Ok(Class {
            id: row.get(0)?,
            name: row.get(1)?,
            role: RoleId(row.get::<_, String>(2)?.parse::<u64>().unwrap()),
            category: CategoryId(row.get(3)?),
            channel: ChannelId(row.get::<_, String>(4)?.parse::<u64>().unwrap()),
        })
    }

    // pub fn insert(conn: &Connection) {
    //     let stmt = conn.prepare("INSERT INTO classes ")
    // }

    // pub fn update() {

    // }
}
