use crate::{
    ProcessingStatus,
    constants::{DATABASE_FILE_NAME, DB_TABLE_NAME},
};
use anyhow::Result;
use rusqlite::{Connection, params};

#[derive(Debug)]
pub struct DatabaseRecord {
    cid: String,
    status: ProcessingStatus,
    timestamp: String,
}

pub struct DatabaseManager {
    connection: Connection,
}

impl DatabaseManager {
    pub fn build() -> Self {
        let connection = Connection::open(DATABASE_FILE_NAME).expect("Could not open database");

        connection
            .execute(
                format!(
                    "CREATE TABLE IF NOT EXISTS {DB_TABLE_NAME} (
                        cid TEXT PRIMARY KEY,
                        status TEXT NOT NULL,
                        timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
                    )"
                )
                .as_str(),
                [],
            )
            .map_err(|err| format!("Create database table error: {}", err))
            .expect("Create database table: unknown error");

        Self { connection }
    }

    pub fn read_cid(&self, cid: &String) -> Result<Option<DatabaseRecord>> {
        let mut stmt = self.connection.prepare(&format!(
            "SELECT cid, status, timestamp FROM {} WHERE cid = ?1",
            DB_TABLE_NAME
        ))?;
        let result = stmt.query_row(params![cid], |row| {
            let status_str: String = row.get(1)?;
            let status = ProcessingStatus::from_str(&status_str).unwrap();

            Ok(DatabaseRecord {
                cid: row.get(0)?,
                status,
                timestamp: row.get(2)?, // or parse into chrono if you prefer
            })
        });

        match result {
            Ok(entry) => Ok(Some(entry)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(_) => panic!("database error"),
        }
    }

    pub fn insert_cid(&self, cid: &String, status: ProcessingStatus) -> Result<()> {
        println!("Inserting cid: {cid}");
        self.connection.execute(
            format!(
                "INSERT INTO {} (cid, status) VALUES (?1, ?2)",
                DB_TABLE_NAME
            )
            .as_str(),
            params![&cid, status.to_string()],
        )?;
        Ok(())
    }

    pub fn update_cid(&self, cid: &String, status: &ProcessingStatus) -> Result<()> {
        println!("Updating cid: {cid} - {status}");
        self.connection.execute(
            format!("UPDATE {} SET status = ?1 WHERE cid = ?2", DB_TABLE_NAME).as_str(),
            params![status.to_string(), cid],
        )?;
        Ok(())
    }
}
