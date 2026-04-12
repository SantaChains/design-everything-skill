use anyhow::Result;
use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category_id: Option<i32>,
    pub keywords: String,
    pub tsv_file: String,
    pub item_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
}

pub struct IndexDB {
    db_path: PathBuf,
}

impl IndexDB {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path = db_path.as_ref().to_path_buf();
        
        if !db_path.exists() {
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let db = Self { db_path };
        db.init_schema()?;
        Ok(db)
    }

    pub fn default_path() -> Result<PathBuf> {
        let exe_dir = std::env::current_exe()?;
        let project_root = exe_dir
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .ok_or_else(|| anyhow::anyhow!("Cannot find project root"))?;
        
        let db_path = project_root.join("data").join("modules_index.db");
        
        if db_path.parent().map(|p| p.exists()).unwrap_or(false) {
            return Ok(db_path);
        }
        
        let cwd = std::env::current_dir()?;
        let cwd_root = cwd
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot find project root from cwd"))?;
        
        Ok(cwd_root.join("data").join("modules_index.db"))
    }

    fn get_connection(&self) -> Result<Connection> {
        Ok(Connection::open(&self.db_path)?)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                slug TEXT NOT NULL UNIQUE,
                description TEXT
            );

            CREATE TABLE IF NOT EXISTS modules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                slug TEXT NOT NULL UNIQUE,
                description TEXT,
                category_id INTEGER,
                keywords TEXT,
                tsv_file TEXT,
                item_count INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (category_id) REFERENCES categories(id)
            );

            CREATE TABLE IF NOT EXISTS module_tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                module_id INTEGER NOT NULL,
                tag TEXT NOT NULL,
                FOREIGN KEY (module_id) REFERENCES modules(id)
            );

            CREATE INDEX IF NOT EXISTS idx_modules_slug ON modules(slug);
            CREATE INDEX IF NOT EXISTS idx_modules_category ON modules(category_id);
            CREATE INDEX IF NOT EXISTS idx_tags_module ON module_tags(module_id);
            CREATE INDEX IF NOT EXISTS idx_tags_tag ON module_tags(tag);
            "#,
        )?;

        Ok(())
    }

    pub fn register_category(&self, name: &str, slug: &str, description: &str) -> Result<i32> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "INSERT OR IGNORE INTO categories (name, slug, description) VALUES (?1, ?2, ?3)",
            (name, slug, description),
        )?;

        let id: i32 = conn.query_row(
            "SELECT id FROM categories WHERE slug = ?1",
            [slug],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    pub fn register_module(
        &self,
        name: &str,
        slug: &str,
        description: &str,
        category_slug: &str,
        keywords: &str,
        tsv_file: &str,
    ) -> Result<i32> {
        let conn = self.get_connection()?;
        
        conn.execute(
            r#"
            INSERT OR REPLACE INTO modules (name, slug, description, category_id, keywords, tsv_file, updated_at)
            VALUES (?1, ?2, ?3, 
                (SELECT id FROM categories WHERE slug = ?4),
                ?5, ?6, CURRENT_TIMESTAMP)
            "#,
            (name, slug, description, category_slug, keywords, tsv_file),
        )?;

        let id: i32 = conn.query_row(
            "SELECT id FROM modules WHERE slug = ?1",
            [slug],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    pub fn get_module_by_slug(&self, slug: &str) -> Result<Option<Module>> {
        let conn = self.get_connection()?;
        
        let result = conn.query_row(
            r#"
            SELECT id, name, slug, description, category_id, keywords, tsv_file, item_count
            FROM modules
            WHERE slug = ?1
            "#,
            [slug],
            |row| self.row_to_module(row),
        );

        match result {
            Ok(module) => Ok(Some(module)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_all_modules(&self) -> Result<Vec<Module>> {
        let conn = self.get_connection()?;
        
        let mut stmt = conn.prepare(
            r#"
            SELECT id, name, slug, description, category_id, keywords, tsv_file, item_count
            FROM modules
            ORDER BY name
            "#,
        )?;

        let modules = stmt
            .query_map([], |row| self.row_to_module(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(modules)
    }

    pub fn search_modules(&self, query: &str) -> Result<Vec<Module>> {
        let conn = self.get_connection()?;
        
        let mut stmt = conn.prepare(
            r#"
            SELECT id, name, slug, description, category_id, keywords, tsv_file, item_count
            FROM modules
            WHERE name LIKE ?1 OR description LIKE ?1 OR keywords LIKE ?1
            ORDER BY
                CASE WHEN name LIKE ?2 THEN 0 ELSE 1 END,
                name
            "#,
        )?;

        let modules = stmt
            .query_map(
                rusqlite::params![format!("%{}%", query), format!("{}%", query)],
                |row| self.row_to_module(row),
            )?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(modules)
    }

    pub fn get_all_categories(&self) -> Result<Vec<Category>> {
        let conn = self.get_connection()?;
        
        let mut stmt = conn.prepare(
            "SELECT id, name, slug, description FROM categories ORDER BY name",
        )?;

        let categories = stmt
            .query_map([], |row| {
                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    slug: row.get(2)?,
                    description: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn update_module_item_count(&self, slug: &str, count: i32) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "UPDATE modules SET item_count = ?1 WHERE slug = ?2",
            (count, slug),
        )?;

        Ok(())
    }

    fn row_to_module(&self, row: &Row) -> rusqlite::Result<Module> {
        Ok(Module {
            id: row.get(0)?,
            name: row.get(1)?,
            slug: row.get(2)?,
            description: row.get(3)?,
            category_id: row.get(4)?,
            keywords: row.get(5)?,
            tsv_file: row.get(6)?,
            item_count: row.get(7)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_db() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = IndexDB::new(temp_file.path()).unwrap();
        
        let categories = db.get_all_categories().unwrap();
        assert!(categories.is_empty());
    }

    #[test]
    fn test_register_category() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = IndexDB::new(temp_file.path()).unwrap();
        
        let id = db.register_category("界面设计", "interface", "UI/UX 设计模块").unwrap();
        assert!(id > 0);

        let categories = db.get_all_categories().unwrap();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].name, "界面设计");
    }

    #[test]
    fn test_register_module() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = IndexDB::new(temp_file.path()).unwrap();
        
        db.register_category("界面设计", "interface", "UI/UX 设计模块").unwrap();
        
        let id = db.register_module(
            "UI Components",
            "ui",
            "按钮、输入框、卡片等组件",
            "interface",
            "ui,components,按钮",
            "ui.tsv",
        ).unwrap();
        
        assert!(id > 0);

        let module = db.get_module_by_slug("ui").unwrap().unwrap();
        assert_eq!(module.name, "UI Components");
        assert_eq!(module.tsv_file, "ui.tsv");
    }
}
