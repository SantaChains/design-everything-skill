use crate::models::DesignState;

pub struct ResearchDetector;

impl ResearchDetector {
    const RESEARCH_TRIGGERS: &'static [&'static str] = &[
        "最新", "2025", "2026", "技术选型", "框架", "对比", "调研",
        "最新技术", "新技术", "趋势", "前沿", "研究", "分析",
    ];

    pub fn needs_research(state: &DesignState) -> bool {
        let combined_text = state.keywords.join(" ");
        Self::RESEARCH_TRIGGERS
            .iter()
            .any(|trigger| combined_text.contains(trigger))
    }

    pub fn detect_research_keywords(state: &DesignState) -> Vec<String> {
        let combined_text = state.keywords.join(" ");
        Self::RESEARCH_TRIGGERS
            .iter()
            .filter(|trigger| combined_text.contains(*trigger))
            .map(|s| s.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_research_need() {
        let state = DesignState {
            keywords: vec!["2025最新框架".to_string()],
            emotion: String::new(),
            scene: String::new(),
            quality_score: None,
            needs_style_research: None,
        };
        assert!(ResearchDetector::needs_research(&state));
    }

    #[test]
    fn test_no_research_need() {
        let state = DesignState {
            keywords: vec!["配色".to_string(), "科技感".to_string()],
            emotion: String::new(),
            scene: String::new(),
            quality_score: None,
            needs_style_research: None,
        };
        assert!(!ResearchDetector::needs_research(&state));
    }

    #[test]
    fn test_extract_research_keywords() {
        let state = DesignState {
            keywords: vec!["2025最新技术选型".to_string()],
            emotion: String::new(),
            scene: String::new(),
            quality_score: None,
            needs_style_research: None,
        };
        let keywords = ResearchDetector::detect_research_keywords(&state);
        assert!(keywords.contains(&"2025".to_string()));
        assert!(keywords.contains(&"最新".to_string()));
        assert!(keywords.contains(&"技术选型".to_string()));
    }
}
