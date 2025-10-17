use anyhow::Result;
use owen::{
    constants::{DEFAULT_DATABASE_NAME, DEFAULT_TABLE_NAME},
    output_generator::DdexMessage,
};
use rusqlite::{params, Connection};
use std::collections::HashMap;

pub struct MessageDatabase {
    connection: Connection,
    table_name: String,
}

impl MessageDatabase {
    pub fn build() -> Self {
        let database_name =
            std::env::var("DATABASE_NAME").unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());

        let table_name =
            std::env::var("TABLE_NAME").unwrap_or_else(|_| DEFAULT_TABLE_NAME.to_string());

        let connection =
            Connection::open(format!("{database_name}.db")).expect("Could not open database");

        connection
            .execute(
                format!(
                    "CREATE TABLE IF NOT EXISTS {table_name} (
                        id INTEGER PRIMARY KEY,
                        message_folder TEXT NOT NULL,
                        status TEXT NOT NULL,
                        reason TEXT,
                        timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
                    )"
                )
                .as_str(),
                [],
            )
            .map_err(|err| format!("Create database table error: {}", err))
            .expect("Create database table: unknown error");

        Self {
            connection,
            table_name,
        }
    }

    pub fn save_message_folders(
        &self,
        local_to_s3_folder_mapping: HashMap<String, String>,
        ddex_messages: Vec<DdexMessage>,
        message_folders: &Vec<String>,
    ) -> Result<HashMap<String, DdexMessage>> {
        let mut s3_folder_to_ddex_message_map: HashMap<String, DdexMessage> = HashMap::new();

        for ddex_message in ddex_messages {
            let s3_path = local_to_s3_folder_mapping
                .get(&ddex_message.message_dir_path)
                .expect(
                    format!(
                        "Could not retrieve s3 path from mapping to local folder. Local folder: {}",
                        ddex_message.message_dir_path
                    )
                    .as_str(),
                );

            s3_folder_to_ddex_message_map.insert(s3_path.clone(), ddex_message);
        }
        for s3_message_folder in message_folders {
            if let Some(ddex_message) = s3_folder_to_ddex_message_map.get(s3_message_folder) {
                if &ddex_message.excluded == &true {
                    let reason = ddex_message.reason.as_deref().unwrap_or("");
                    self.connection.execute(
                        format!(
                            "INSERT INTO {} (message_folder, status, reason) VALUES (?1, ?2, ?3)",
                            &self.table_name
                        )
                        .as_str(),
                        params![&s3_message_folder, "rejected", reason],
                    )?;
                } else {
                    self.connection.execute(
                        format!(
                            "INSERT INTO {} (message_folder, status) VALUES (?1, ?2)",
                            &self.table_name
                        )
                        .as_str(),
                        params![&s3_message_folder, "processed"],
                    )?;
                }
            } else {
                self.connection.execute(
                    format!(
                        "INSERT INTO {} (message_folder, status, reason) VALUES (?1, ?2, ?3)",
                        &self.table_name
                    )
                    .as_str(),
                    params![&s3_message_folder, "unprocessed", "unknown"],
                )?;
            }
        }
        Ok(s3_folder_to_ddex_message_map)
    }

    pub fn save_message_folders_with_status(
        &mut self,
        message_folders: &Vec<String>,
        status: String,
        reason: Option<String>,
    ) -> Result<()> {
        let reason_string = reason.unwrap_or_else(|| "unknown".to_string());

        let entries: Vec<(String, String)> = message_folders
            .iter()
            .map(|folder| (folder.clone(), status.clone()))
            .collect();
        let tx = self.connection.transaction()?;
        {
            let mut statement = tx.prepare(
                format!(
                    "INSERT INTO {} (message_folder, status, reason) VALUES (?1, ?2, ?3)",
                    &self.table_name
                )
                .as_str(),
            )?;
            for entry in entries {
                statement.execute(params![&entry.0, &entry.1, &reason_string])?;
            }
        }
        tx.commit()?;

        Ok(())
    }
}
