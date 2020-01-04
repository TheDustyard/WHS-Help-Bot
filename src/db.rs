use crate::{
    model::{Class, Group},
    status_logger::StatusLogger,
};
use include_sql::include_sql;
use log::{debug, error, trace};
use rusqlite::{
    types::Value, vtab::array, Connection, Error as SQLError, Result as SQLResult, Row, NO_PARAMS,
};
use serenity::{
    http::Http,
    model::id::{ChannelId, GuildId, RoleId},
    Error as SerenityError, Result as SerenityResult,
};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;
use std::rc::Rc;

include_sql!("src/sql/class.sql", "?");
include_sql!("src/sql/group.sql", "?");

pub struct Database {
    connection: Connection,
}

/// Constructors
impl Database {
    pub fn open<P: AsRef<Path> + Display>(file: P) -> SQLResult<Database> {
        match Connection::open(&file) {
            Ok(connection) => {
                trace!("Connected to database: {}", file);

                // ENABLE FOREIGN KEYS
                connection.execute("PRAGMA foreign_keys = ON;", NO_PARAMS)?;
                trace!("Enabled Foreign Keys on database: {}", file);

                array::load_module(&connection)?;
                trace!("Enabled carray() on database: {}", file);

                // CREATE TABLES IF NOT EXIST
                connection.execute(CREATE_GROUP_TABLE, NO_PARAMS)?;
                connection.execute(CREATE_CLASS_TABLE, NO_PARAMS)?;

                return Ok(Database { connection });
            }
            Err(e) => {
                let message = format!("Error connecting to database {}: {:?}", file, e);
                error!("{}", message);
                panic!("{}", message);
            }
        }
    }
}

/// Static Helpers
impl Database {
    /// Helper function to convert a string to a u64 simply
    ///
    /// # Panics
    /// When the string provided cannot be parsed as a u64
    fn asu64(string: String) -> u64 {
        string.parse().unwrap()
    }

    /// A helper function to transform a row using the following schema
    /// ```
    ///     `group`.`id`,
    ///     `group`.`name`,
    ///     `group`.`channel_group`,
    ///     `group`.`vc`
    /// ```
    fn get_group_from_row(row: &Row) -> SQLResult<Group> {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            channel_group: ChannelId(Self::asu64(row.get(2)?)),
            vc: ChannelId(Self::asu64(row.get(3)?)),
        })
    }

    /// A helper function to transform a row using the following schema
    /// ```
    ///     `class`.`id`,
    ///     `class`.`name`,
    ///     `class`.`role`,
    ///     `class`.`channel`,
    ///     `group`.`id`,
    ///     `group`.`name`,
    ///     `group`.`channel_group`,
    ///     `group`.`vc`
    /// ```
    fn get_class_with_group_from_row(row: &Row) -> SQLResult<Class> {
        Ok(Class {
            id: row.get(0)?,
            name: row.get(1)?,
            role: RoleId(Self::asu64(row.get(2)?)),
            channel: ChannelId(Self::asu64(row.get(3)?)),
            group: if let Some(id) = row.get::<_, Option<i64>>(4)? {
                Some(Group {
                    id,
                    name: row.get(5)?,
                    channel_group: ChannelId(Self::asu64(row.get(6)?)),
                    vc: ChannelId(Self::asu64(row.get(7)?)),
                })
            } else {
                None
            },
        })
    }
}

/// Class functions
impl Database {
    /// A helper function to fetch all of the classes from the database
    pub fn get_all_classes(&self) -> SQLResult<Vec<Class>> {
        let mut stmt = self.connection.prepare_cached(GET_ALL_CLASSES)?;

        let all = stmt
            .query_map(NO_PARAMS, |row| Self::get_class_with_group_from_row(row))?
            .collect();

        all
    }

    pub fn classes_count(&self) -> SQLResult<u32> {
        let mut stmt = self.connection.prepare_cached(COUNT_ALL_CLASSES)?;

        stmt.query_row(NO_PARAMS, |row| row.get(0))
    }

    // TODO: OPTOMIZE
    pub fn map_classes_by_group(classes: &[Class]) -> BTreeMap<Option<Group>, Vec<&Class>> {
        let mut map = BTreeMap::new();

        for class in classes {
            map.entry(class.group.clone())
                .or_insert_with(Vec::new)
                .push(class);
        }

        map
    }

    /// A helper function to fetch all of the classes from the database that fit a search term
    pub fn search_classes(&self, search_term: &str) -> SQLResult<Vec<Class>> {
        let mut stmt = self.connection.prepare_cached(SEARCH_CLASSES)?;

        let classes = stmt
            .query_map(&[format!("%{}%", search_term)], |row| {
                Self::get_class_with_group_from_row(row)
            })?
            .collect();

        classes
    }

    /// A helper function to fetch all of the classes from the database that fit a search term
    pub fn filter_classes_by_roles(&self, roles: &[RoleId]) -> SQLResult<Vec<Class>> {
        let mut stmt = self.connection.prepare_cached(FILTER_CLASSES_BY_ROLES)?;

        let roles = roles
            .into_iter()
            .map(|x| x.to_string())
            .map(Value::from)
            .collect();
        let ptr = Rc::new(roles);

        let classes = stmt
            .query_map(&[&ptr], |row| Self::get_class_with_group_from_row(row))?
            .collect();

        classes
    }
}

/// Group functions
impl Database {
    /// A helper function to fetch all of the groups from the database
    pub fn get_all_groups(&self) -> SQLResult<Vec<Group>> {
        let mut stmt = self.connection.prepare_cached(GET_ALL_GROUPS)?;

        let all = stmt
            .query_map(NO_PARAMS, |row| Self::get_group_from_row(row))?
            .collect();

        all
    }

    pub fn groups_count(&self) -> SQLResult<u32> {
        let mut stmt = self.connection.prepare_cached(COUNT_ALL_GROUPS)?;

        stmt.query_row(NO_PARAMS, |row| row.get(0))
    }

    /// A helper function to fetch all of the classes from the database that fit a search term
    pub fn search_groups(&self, search_term: &str) -> SQLResult<Vec<Group>> {
        let mut stmt = self.connection.prepare_cached(SEARCH_GROUPS)?;

        let groups = stmt
            .query_map(&[format!("%{}%", search_term)], |row| {
                Self::get_group_from_row(row)
            })?
            .collect();

        groups
    }

    pub fn insert_group(
        &self,
        name: &str,
        channel_group: ChannelId,
        vc: ChannelId,
    ) -> SQLResult<Group> {
        let mut stmt = self.connection.prepare_cached(INSERT_GROUP)?;
        stmt.execute(&[name, &channel_group.to_string(), &vc.to_string()])?;

        let id = self.connection.last_insert_rowid();

        Ok(Group {
            id,
            name: name.to_owned(),
            channel_group,
            vc,
        })
    }
}

/// Synchronization
impl Database {
    pub fn sync(
        &self,
        logger: &StatusLogger,
        guild: GuildId,
        ctx: &impl AsRef<Http>,
    ) -> Result<(), SQLSerenityError> {
        trace!("Starting database sync");

        trace!("Starting groups sync");

        let channels = guild.channels(ctx)?;
        let groups = self.get_all_groups()?;

        for (channelid, channel) in channels {
            debug!(
                "{} - {}",
                channelid,
                groups.iter().any(|x| x.channel_group == channelid)
            );
        }

        trace!("Finished groups sync");

        Ok(())
    }
}

pub enum SQLSerenityError {
    SQLError(SQLError),
    SerenityError(SerenityError),
}

impl From<SQLError> for SQLSerenityError {
    fn from(error: SQLError) -> Self {
        Self::SQLError(error)
    }
}

impl From<SerenityError> for SQLSerenityError {
    fn from(error: SerenityError) -> Self {
        Self::SerenityError(error)
    }
}
