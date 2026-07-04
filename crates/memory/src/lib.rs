use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.initialize_tables()?;
        Ok(store)
    }

    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.initialize_tables()?;
        Ok(store)
    }

    fn initialize_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS conversation (
                id TEXT PRIMARY KEY,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS preference (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn save_message(&self, id: &str, role: &str, content: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO conversation (id, role, content, timestamp) VALUES (?1, ?2, ?3, ?4)",
            params![id, role, content, now],
        )?;
        Ok(())
    }

    pub fn get_preference(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM preference WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            let val: String = row.get(0)?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    pub fn set_preference(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO preference (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_store_in_memory() {
        let store = MemoryStore::new_in_memory().unwrap();
        
        // Test preferences
        store.set_preference("theme", "dark").unwrap();
        let theme = store.get_preference("theme").unwrap();
        assert_eq!(theme, Some("dark".to_string()));

        // Test non-existent preference
        let missing = store.get_preference("non-existent").unwrap();
        assert_eq!(missing, None);

        // Test conversation history
        store.save_message("msg-1", "user", "hello").unwrap();
    }
}
