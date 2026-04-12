use crate::models::DesignState;
use crate::router::RouterConfig;

pub struct AmbiguityCalculator;

impl AmbiguityCalculator {
    pub fn calculate(state: &DesignState) -> f32 {
        let mut score: f32 = 0.0;

        if state.keywords.len() < 3 {
            score += 0.3;
        }

        if state.emotion.is_empty() {
            score += 0.3;
        }

        if state.scene.is_empty() {
            score += 0.4;
        }

        score.min(1.0)
    }

    pub fn is_high_ambiguity(state: &DesignState, config: &RouterConfig) -> bool {
        Self::calculate(state) > config.ambiguity_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_state_has_high_ambiguity() {
        let state = DesignState::new();
        let config = RouterConfig::default();
        assert!(AmbiguityCalculator::is_high_ambiguity(&state, &config));
        assert_eq!(AmbiguityCalculator::calculate(&state), 1.0);
    }

    #[test]
    fn test_complete_state_has_low_ambiguity() {
        let state = DesignState {
            keywords: vec!["科技".to_string(), "配色".to_string(), "网页".to_string()],
            emotion: "信任".to_string(),
            scene: "企业官网".to_string(),
            quality_score: None,
            needs_style_research: None,
        };
        let config = RouterConfig::default();
        assert!(!AmbiguityCalculator::is_high_ambiguity(&state, &config));
        assert_eq!(AmbiguityCalculator::calculate(&state), 0.0);
    }

    #[test]
    fn test_partial_state_has_medium_ambiguity() {
        let state = DesignState {
            keywords: vec!["配色".to_string()],
            emotion: String::new(),
            scene: "网页".to_string(),
            quality_score: None,
            needs_style_research: None,
        };
        let score = AmbiguityCalculator::calculate(&state);
        assert!(score > 0.3 && score < 0.7);
    }
}
