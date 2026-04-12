use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    Analyze,
    Propose,
    Color,
    Layout,
    Elements,
    Avoid,
    Complete,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stage::Analyze => write!(f, "ANALYZE"),
            Stage::Propose => write!(f, "PROPOSE"),
            Stage::Color => write!(f, "COLOR"),
            Stage::Layout => write!(f, "LAYOUT"),
            Stage::Elements => write!(f, "ELEMENTS"),
            Stage::Avoid => write!(f, "AVOID"),
            Stage::Complete => write!(f, "COMPLETE"),
        }
    }
}

impl std::str::FromStr for Stage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ANALYZE" => Ok(Stage::Analyze),
            "PROPOSE" => Ok(Stage::Propose),
            "COLOR" => Ok(Stage::Color),
            "LAYOUT" => Ok(Stage::Layout),
            "ELEMENTS" => Ok(Stage::Elements),
            "AVOID" => Ok(Stage::Avoid),
            "COMPLETE" => Ok(Stage::Complete),
            _ => Err(format!("Unknown stage: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignState {
    pub keywords: Vec<String>,
    pub emotion: String,
    pub scene: String,
    pub quality_score: Option<f32>,
    pub needs_style_research: Option<bool>,
}

impl DesignState {
    pub fn new() -> Self {
        Self {
            keywords: Vec::new(),
            emotion: String::new(),
            scene: String::new(),
            quality_score: None,
            needs_style_research: None,
        }
    }

    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

impl Default for DesignState {
    fn default() -> Self {
        Self::new()
    }
}
