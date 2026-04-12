use crate::models::{DesignState, SkillTrigger, SkillType, Stage};
use crate::router::{AmbiguityCalculator, ResearchDetector, RouterConfig};

pub struct SkillRouter {
    config: RouterConfig,
}

impl SkillRouter {
    pub fn new() -> Self {
        Self {
            config: RouterConfig::from_env(),
        }
    }

    pub fn with_config(config: RouterConfig) -> Self {
        Self { config }
    }

    pub fn route(&self, stage: &Stage, state: &DesignState) -> Option<SkillTrigger> {
        match stage {
            Stage::Analyze => self.route_analyze(state),
            Stage::Propose => self.route_propose(state),
            Stage::Complete => self.route_complete(),
            _ => None,
        }
    }

    fn route_analyze(&self, state: &DesignState) -> Option<SkillTrigger> {
        let ambiguity = AmbiguityCalculator::calculate(state);
        
        if ambiguity > self.config.ambiguity_threshold {
            return Some(
                SkillTrigger::new(
                    SkillType::Brainstorming,
                    format!("需求模糊度 {:.1} > {:.1}", ambiguity, self.config.ambiguity_threshold),
                    self.config.brainstorming_priority,
                )
            );
        }

        if ResearchDetector::needs_research(state) {
            let keywords = ResearchDetector::detect_research_keywords(state);
            return Some(
                SkillTrigger::new(
                    SkillType::ActiveResearch,
                    format!("检测到研究需求: {}", keywords.join(", ")),
                    self.config.research_priority,
                )
            );
        }

        None
    }

    fn route_propose(&self, state: &DesignState) -> Option<SkillTrigger> {
        if state.needs_style_research.unwrap_or(false) {
            return Some(
                SkillTrigger::new(
                    SkillType::ActiveResearch,
                    "需要研究风格的历史和案例".to_string(),
                    self.config.research_priority,
                )
            );
        }
        None
    }

    fn route_complete(&self) -> Option<SkillTrigger> {
        Some(
            SkillTrigger::new(
                SkillType::SelfImproving,
                "记录设计经验到长期记忆".to_string(),
                self.config.self_improving_priority,
            )
            .with_always(true)
        )
    }
}

impl Default for SkillRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_to_brainstorming() {
        let router = SkillRouter::new();
        let state = DesignState::new();
        let trigger = router.route(&Stage::Analyze, &state);
        
        assert!(trigger.is_some());
        let trigger = trigger.unwrap();
        assert_eq!(trigger.skill, SkillType::Brainstorming);
        assert!(trigger.priority >= 0.9);
    }

    #[test]
    fn test_route_to_active_research() {
        let router = SkillRouter::new();
        let state = DesignState {
            keywords: vec!["2025最新框架".to_string()],
            emotion: "科技".to_string(),
            scene: "网页".to_string(),
            quality_score: None,
            needs_style_research: None,
        };
        let trigger = router.route(&Stage::Analyze, &state);
        
        assert!(trigger.is_some());
        let trigger = trigger.unwrap();
        assert_eq!(trigger.skill, SkillType::ActiveResearch);
    }

    #[test]
    fn test_route_to_self_improving_on_complete() {
        let router = SkillRouter::new();
        let state = DesignState::new();
        let trigger = router.route(&Stage::Complete, &state);
        
        assert!(trigger.is_some());
        let trigger = trigger.unwrap();
        assert_eq!(trigger.skill, SkillType::SelfImproving);
        assert_eq!(trigger.always, Some(true));
    }

    #[test]
    fn test_no_route_for_complete_state() {
        let router = SkillRouter::new();
        let state = DesignState {
            keywords: vec!["科技".to_string(), "配色".to_string(), "网页".to_string()],
            emotion: "信任".to_string(),
            scene: "企业官网".to_string(),
            quality_score: None,
            needs_style_research: None,
        };
        let trigger = router.route(&Stage::Analyze, &state);
        assert!(trigger.is_none());
    }

    #[test]
    fn test_custom_threshold() {
        let config = RouterConfig {
            ambiguity_threshold: 0.5,
            ..Default::default()
        };
        let router = SkillRouter::with_config(config);
        
        let state = DesignState {
            keywords: vec!["配色".to_string()],
            emotion: String::new(),
            scene: String::new(),
            quality_score: None,
            needs_style_research: None,
        };
        
        let trigger = router.route(&Stage::Analyze, &state);
        assert!(trigger.is_some());
        
        let trigger = trigger.unwrap();
        assert_eq!(trigger.skill, SkillType::Brainstorming);
        assert!(trigger.reason.contains("0.5"));
    }
}
