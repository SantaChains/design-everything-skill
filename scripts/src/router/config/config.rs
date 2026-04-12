use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    pub ambiguity_threshold: f32,
    pub research_priority: f32,
    pub brainstorming_priority: f32,
    pub self_improving_priority: f32,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            ambiguity_threshold: 0.7,
            research_priority: 0.8,
            brainstorming_priority: 0.9,
            self_improving_priority: 0.6,
        }
    }
}

impl RouterConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(threshold) = env::var("SKILL_ROUTER_AMBIGUITY_THRESHOLD") {
            if let Ok(value) = threshold.parse::<f32>() {
                if (0.0..=1.0).contains(&value) {
                    config.ambiguity_threshold = value;
                }
            }
        }

        if let Ok(priority) = env::var("SKILL_ROUTER_RESEARCH_PRIORITY") {
            if let Ok(value) = priority.parse::<f32>() {
                config.research_priority = value;
            }
        }

        if let Ok(priority) = env::var("SKILL_ROUTER_BRAINSTORMING_PRIORITY") {
            if let Ok(value) = priority.parse::<f32>() {
                config.brainstorming_priority = value;
            }
        }

        if let Ok(priority) = env::var("SKILL_ROUTER_SELF_IMPROVING_PRIORITY") {
            if let Ok(value) = priority.parse::<f32>() {
                config.self_improving_priority = value;
            }
        }

        config
    }

    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RouterConfig::default();
        assert_eq!(config.ambiguity_threshold, 0.7);
        assert_eq!(config.research_priority, 0.8);
        assert_eq!(config.brainstorming_priority, 0.9);
        assert_eq!(config.self_improving_priority, 0.6);
    }

    #[test]
    fn test_config_serialization() {
        let config = RouterConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: RouterConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.ambiguity_threshold, deserialized.ambiguity_threshold);
    }
}
