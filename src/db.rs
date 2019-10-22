use log::{debug, error};
use prettytable::{Cell, Row, Table};
use rusqlite::{Connection, Result as SQLResult, NO_PARAMS};
use std::env;
use std::fmt::{Display, Formatter};
use std::path::Path;

static CATEGORY_SQL: &[u8] = include_bytes!("sql/category.sql");
static CLASS_SQL: &[u8] = include_bytes!("sql/class.sql");

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path> + Display>(file: P) -> Database {
        match Connection::open(&file) {
            Ok(connection) => {
                debug!("Connected to database: {}", file);

                // ENABLE FOREIGN KEYS
                connection
                    .execute("PRAGMA foreign_keys = ON;", NO_PARAMS)
                    .unwrap();
                debug!("Enabled Foreign Keys on database: {}", file);

                // CREATE TABLES IF NOT EXIST
                connection
                    .execute(&String::from_utf8_lossy(CATEGORY_SQL), NO_PARAMS)
                    .unwrap();
                connection
                    .execute(&String::from_utf8_lossy(CLASS_SQL), NO_PARAMS)
                    .unwrap();

                return Database { connection };
            }
            Err(e) => {
                let message = format!("Error connecting to database {}: {:?}", file, e);
                error!("{}", message);
                panic!("{}", message);
            }
        }
    }
}

impl Display for Database {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut select = self
            .connection
            .prepare("SELECT name FROM sqlite_master WHERE type='table';")
            .unwrap();

        let tables = select
            .query_map(NO_PARAMS, |row| {
                let mut table = Table::new();

                let name = row.get::<_, String>(0)?;

                let mut stmt = self
                    .connection
                    .prepare(&format!("SELECT * FROM {}", name))
                    .unwrap();
                let header_row = Row::new(
                    stmt.columns()
                        .into_iter()
                        .map(|column| Cell::new(column.name()))
                        .collect::<Vec<_>>(),
                );
                let rows = stmt
                    .query_map(NO_PARAMS, |row| {
                        let mut cells = Vec::new();

                        for column_num in 0..row.column_count() {
                            cells.push(Cell::new(&format!("{:?}", row.get::<_, String>(column_num))));
                        }

                        Ok(Row::new(cells))
                    })
                    .unwrap();

                table.add_row(header_row);
                for row in rows {
                    match row {
                        Ok(row) => {
                            table.add_row(row);
                        }
                        Err(e) => return Err(e)
                    }
                }

                Ok((name.clone(), table))
            })
            .unwrap();

        for table in tables {
            match table {
                Ok((name, table)) => writeln!(formatter, "table: {}\n{}", name, table),
                Err(e) => writeln!(formatter, "ERR: {}", e),
            }
            .unwrap();
        }

        Ok(())
    }
}
