use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSVItem {
    pub title: String,
    pub content: String,
    pub tags: String,
    pub metadata: String,
}

pub struct TSVReader {
    tsv_dir: PathBuf,
}

impl TSVReader {
    pub fn new<P: AsRef<Path>>(tsv_dir: P) -> Result<Self> {
        let tsv_dir = tsv_dir.as_ref().to_path_buf();
        
        if !tsv_dir.exists() {
            std::fs::create_dir_all(&tsv_dir)?;
        }

        Ok(Self { tsv_dir })
    }

    pub fn default_path() -> Result<PathBuf> {
        let exe_dir = std::env::current_exe()?;
        let project_root = exe_dir
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .ok_or_else(|| anyhow::anyhow!("Cannot find project root"))?;
        
        let tsv_path = project_root.join("data").join("tsv");
        
        if tsv_path.exists() {
            return Ok(tsv_path);
        }
        
        let cwd = std::env::current_dir()?;
        let cwd_root = cwd
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot find project root from cwd"))?;
        
        Ok(cwd_root.join("data").join("tsv"))
    }

    pub fn read_tsv(&self, filename: &str) -> Result<Vec<TSVItem>> {
        let filepath = self.tsv_dir.join(filename);
        
        if !filepath.exists() {
            return Err(anyhow::anyhow!("TSV file not found: {:?}", filepath));
        }

        let content = std::fs::read_to_string(&filepath)?;
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(true)
            .from_reader(content.as_bytes());

        let mut items = Vec::new();
        for result in reader.deserialize() {
            let item: TSVItem = result?;
            items.push(item);
        }

        Ok(items)
    }

    pub fn search_in_tsv(&self, filename: &str, query: &str) -> Result<Vec<TSVItem>> {
        let items = self.read_tsv(filename)?;
        let query_lower = query.to_lowercase();

        let results: Vec<TSVItem> = items
            .into_iter()
            .filter(|item| {
                item.title.to_lowercase().contains(&query_lower)
                    || item.content.to_lowercase().contains(&query_lower)
            })
            .collect();

        Ok(results)
    }

    pub fn search_all(&self, query: &str) -> Result<HashMap<String, Vec<TSVItem>>> {
        let mut results = HashMap::new();

        if !self.tsv_dir.exists() {
            return Ok(results);
        }

        for entry in std::fs::read_dir(&self.tsv_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("tsv") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if let Ok(module_results) = self.search_in_tsv(filename, query) {
                        if !module_results.is_empty() {
                            let module_slug = filename.trim_end_matches(".tsv");
                            results.insert(module_slug.to_string(), module_results);
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    pub fn get_module_list(&self) -> Result<Vec<String>> {
        let mut modules = Vec::new();

        if !self.tsv_dir.exists() {
            return Ok(modules);
        }

        for entry in std::fs::read_dir(&self.tsv_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("tsv") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    modules.push(filename.trim_end_matches(".tsv").to_string());
                }
            }
        }

        modules.sort();
        Ok(modules)
    }

    pub fn count_items(&self, filename: &str) -> Result<usize> {
        let items = self.read_tsv(filename)?;
        Ok(items.len())
    }

    pub fn get_all_counts(&self) -> Result<HashMap<String, usize>> {
        let mut counts = HashMap::new();

        if !self.tsv_dir.exists() {
            return Ok(counts);
        }

        for entry in std::fs::read_dir(&self.tsv_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("tsv") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if let Ok(count) = self.count_items(filename) {
                        let module_slug = filename.trim_end_matches(".tsv");
                        counts.insert(module_slug.to_string(), count);
                    }
                }
            }
        }

        Ok(counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_read_tsv() {
        let temp_dir = TempDir::new().unwrap();
        let tsv_path = temp_dir.path().join("test.tsv");

        let mut file = std::fs::File::create(&tsv_path).unwrap();
        writeln!(file, "title\tcontent\ttags\tmetadata").unwrap();
        writeln!(file, "测试标题\t测试内容\t测试标签\tformat=list").unwrap();

        let reader = TSVReader::new(temp_dir.path()).unwrap();
        let items = reader.read_tsv("test.tsv").unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "测试标题");
        assert_eq!(items[0].content, "测试内容");
    }

    #[test]
    fn test_search_in_tsv() {
        let temp_dir = TempDir::new().unwrap();
        let tsv_path = temp_dir.path().join("test.tsv");

        let mut file = std::fs::File::create(&tsv_path).unwrap();
        writeln!(file, "title\tcontent\ttags\tmetadata").unwrap();
        writeln!(file, "配色方案\t蓝色系配色\t配色,蓝色\tstyle=code").unwrap();
        writeln!(file, "构图技巧\t三分法构图\t构图,技巧\tstyle=code").unwrap();

        let reader = TSVReader::new(temp_dir.path()).unwrap();
        let results = reader.search_in_tsv("test.tsv", "配色").unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "配色方案");
    }

    #[test]
    fn test_get_module_list() {
        let temp_dir = TempDir::new().unwrap();

        let tsv1 = temp_dir.path().join("module1.tsv");
        let tsv2 = temp_dir.path().join("module2.tsv");

        std::fs::File::create(&tsv1).unwrap();
        std::fs::File::create(&tsv2).unwrap();

        let reader = TSVReader::new(temp_dir.path()).unwrap();
        let modules = reader.get_module_list().unwrap();

        assert_eq!(modules.len(), 2);
        assert!(modules.contains(&"module1".to_string()));
        assert!(modules.contains(&"module2".to_string()));
    }
}
