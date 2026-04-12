use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillType {
    Brainstorming,
    ActiveResearch,
    SelfImproving,
}

impl fmt::Display for SkillType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillType::Brainstorming => write!(f, "brainstorming"),
            SkillType::ActiveResearch => write!(f, "active-research"),
            SkillType::SelfImproving => write!(f, "self-improving"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTrigger {
    pub skill: SkillType,
    pub reason: String,
    pub priority: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always: Option<bool>,
}

impl SkillTrigger {
    pub fn new(skill: SkillType, reason: String, priority: f32) -> Self {
        Self {
            skill,
            reason,
            priority,
            always: None,
        }
    }

    pub fn with_always(mut self, always: bool) -> Self {
        self.always = Some(always);
        self
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
