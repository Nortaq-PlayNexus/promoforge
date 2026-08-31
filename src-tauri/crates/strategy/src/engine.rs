use promoforge_core::*;
use crate::playbooks;

pub struct StrategyEngine;

impl StrategyEngine {
    pub fn generate_strategy(
        pkb: &ProductKnowledgeBase,
        project: &Project,
    ) -> Result<(Campaign, Strategy)> {
        tracing::info!("Generating strategy for: {}", pkb.identity.inferred_name);

        let playbook = playbooks::select_playbook(&project.project_type);
        tracing::info!("Selected playbook: {}", playbook.name);

        let campaign = Campaign {
            id: new_id(),
            project_id: project.id.clone(),
            name: format!("{} Launch Campaign", pkb.identity.inferred_name),
            status: CampaignStatus::Planning,
            strategy_version: 1,
            phases: Self::build_phases(pkb, &playbook),
            total_budget: 0.0,
            spent_budget: 0.0,
            created_at: now(),
            updated_at: now(),
        };

        let strategy = Strategy {
            id: new_id(),
            project_id: project.id.clone(),
            campaign_id: campaign.id.clone(),
            executive_summary: Self::generate_executive_summary(pkb, &playbook),
            objectives: Self::generate_objectives(pkb, &project.project_type),
            phases: Self::build_strategy_phases(pkb, &playbook),
            asset_requirements: Self::calculate_asset_requirements(pkb, &playbook),
            budget_breakdown: BudgetBreakdown {
                total: 0.0,
                generation_api: 0.0,
                ad_spend: 0.0,
                tools: 0.0,
                human_review: 0.0,
            },
            risks: Self::identify_risks(pkb),
            assumptions: Self::list_assumptions(pkb),
            generated_at: now(),
        };

        tracing::info!("Strategy generated with {} phases", strategy.phases.len());
        Ok((campaign, strategy))
    }

    fn generate_executive_summary(pkb: &ProductKnowledgeBase, playbook: &Playbook) -> String {
        let feature_count = pkb.features.len();
        let persona_count = pkb.personas.len();
        let tech_stack = pkb.technical.tech_stack.join(", ");

        format!(
            "Promotion strategy for {} ({}) — a {} built with {}. \
             This plan targets {} user personas across {} phases. \
             Playbook: {}. Key focus areas: {}, {}, and {}.",
            pkb.identity.inferred_name,
            pkb.identity.category,
            pkb.identity.category.to_lowercase(),
            if tech_stack.is_empty() { "multiple technologies" } else { &tech_stack },
            persona_count,
            playbook.phases.len(),
            playbook.name,
            playbook.phases.first().map(|p| p.name.as_str()).unwrap_or("awareness"),
            playbook.phases.get(1).map(|p| p.name.as_str()).unwrap_or("engagement"),
            playbook.phases.last().map(|p| p.name.as_str()).unwrap_or("conversion"),
        )
    }

    fn generate_objectives(pkb: &ProductKnowledgeBase, project_type: &ProjectType) -> Vec<StrategyObjective> {
        let mut objectives = vec![StrategyObjective {
            description: format!("Build awareness for {}", pkb.identity.inferred_name),
            target: "10,000 impressions".into(),
            timeline: "Weeks 1-4".into(),
            confidence: 0.7,
        }];

        match project_type {
            ProjectType::SteamGame => {
                objectives.push(StrategyObjective {
                    description: "Build Steam wishlists".into(),
                    target: "1,000 wishlists".into(),
                    timeline: "Weeks 1-12".into(),
                    confidence: 0.6,
                });
                objectives.push(StrategyObjective {
                    description: "Secure press/creator coverage".into(),
                    target: "5 articles/videos".into(),
                    timeline: "Weeks 4-12".into(),
                    confidence: 0.5,
                });
            }
            ProjectType::CliTool | ProjectType::OpenSourceLibrary => {
                objectives.push(StrategyObjective {
                    description: "Grow GitHub community".into(),
                    target: "500 stars".into(),
                    timeline: "Weeks 1-8".into(),
                    confidence: 0.6,
                });
                objectives.push(StrategyObjective {
                    description: "Build developer adoption".into(),
                    target: "100 weekly downloads".into(),
                    timeline: "Weeks 4-12".into(),
                    confidence: 0.5,
                });
            }
            ProjectType::SaasPlatform | ProjectType::WebApplication => {
                objectives.push(StrategyObjective {
                    description: "Drive signups".into(),
                    target: "500 signups".into(),
                    timeline: "Weeks 1-8".into(),
                    confidence: 0.6,
                });
                objectives.push(StrategyObjective {
                    description: "Convert to paid users".into(),
                    target: "5% conversion".into(),
                    timeline: "Weeks 4-16".into(),
                    confidence: 0.5,
                });
            }
            _ => {
                objectives.push(StrategyObjective {
                    description: "Build initial user base".into(),
                    target: "200 users".into(),
                    timeline: "Weeks 1-8".into(),
                    confidence: 0.5,
                });
            }
        }

        objectives
    }

    fn build_phases(pkb: &ProductKnowledgeBase, playbook: &Playbook) -> Vec<CampaignPhase> {
        playbook
            .phases
            .iter()
            .map(|phase| CampaignPhase {
                id: new_id(),
                name: phase.name.clone(),
                status: "pending".into(),
                start_date: None,
                end_date: None,
                objectives: phase.objectives.clone(),
                channels: phase
                    .channels
                    .iter()
                    .map(|ch| ChannelPlan {
                        channel: ch.clone(),
                        platforms: vec![ch.clone()],
                        tactics: Vec::new(),
                        content_types: vec!["social".into(), "blog".into()],
                        frequency: "daily".into(),
                        budget: 0.0,
                    })
                    .collect(),
                budget_allocation: 0.0,
                kpi_targets: HashMap::new(),
            })
            .collect()
    }

    fn build_strategy_phases(pkb: &ProductKnowledgeBase, playbook: &Playbook) -> Vec<StrategyPhase> {
        playbook
            .phases
            .iter()
            .map(|phase| StrategyPhase {
                name: phase.name.clone(),
                duration: phase.duration.clone(),
                channels: phase
                    .channels
                    .iter()
                    .map(|ch| ChannelPlan {
                        channel: ch.clone(),
                        platforms: vec![ch.clone()],
                        tactics: playbook
                            .tactics
                            .iter()
                            .filter(|t| t.phase == phase.name)
                            .map(|t| Tactic {
                                name: t.name.clone(),
                                description: t.description.clone(),
                                effort: t.effort.clone(),
                                expected_impact: t.expected_impact.clone(),
                                kpis: t.kpis.clone(),
                            })
                            .collect(),
                        content_types: vec!["social".into(), "blog".into(), "email".into()],
                        frequency: "3-5x per week".into(),
                        budget: 0.0,
                    })
                    .collect(),
                content_calendar: Vec::new(),
                milestones: phase.objectives.clone(),
                kpi_targets: HashMap::new(),
            })
            .collect()
    }

    fn calculate_asset_requirements(pkb: &ProductKnowledgeBase, playbook: &Playbook) -> AssetRequirements {
        let text_count = playbook.tactics.len() * 3;
        let image_count = playbook.tactics.len() * 2;
        let total = text_count + image_count;

        let mut by_type = HashMap::new();
        by_type.insert("text".into(), text_count);
        by_type.insert("image".into(), image_count);

        let mut by_priority = HashMap::new();
        by_priority.insert("high".into(), total / 3);
        by_priority.insert("medium".into(), total / 3);
        by_priority.insert("low".into(), total - (total / 3) * 2);

        AssetRequirements {
            total_count: total,
            by_type,
            by_priority,
            production_schedule: vec![
                "Week 1: Core messaging & brand assets".into(),
                "Week 2: Social media templates".into(),
                "Week 3: Channel-specific content".into(),
                "Week 4: Launch assets".into(),
            ],
        }
    }

    fn identify_risks(pkb: &ProductKnowledgeBase) -> Vec<Risk> {
        let mut risks = vec![
            Risk {
                description: "Low initial awareness".into(),
                probability: "High".into(),
                impact: "Medium".into(),
                mitigation: "Multi-channel launch strategy with pre-launch buzz building".into(),
            },
            Risk {
                description: "Content quality inconsistency".into(),
                probability: "Medium".into(),
                impact: "High".into(),
                mitigation: "Brand guardian agent + human review gates for all public content".into(),
            },
        ];

        if pkb.competitive.competitors.len() > 5 {
            risks.push(Risk {
                description: "Highly competitive market".into(),
                probability: "High".into(),
                impact: "High".into(),
                mitigation: "Focus on unique differentiators identified from code analysis".into(),
            });
        }

        if pkb.identity.maturity_signals.test_coverage.is_none() {
            risks.push(Risk {
                description: "No test coverage detected - quality perception risk".into(),
                probability: "Medium".into(),
                impact: "Medium".into(),
                mitigation: "Don't emphasize technical metrics; focus on user-facing value".into(),
            });
        }

        risks
    }

    fn list_assumptions(pkb: &ProductKnowledgeBase) -> Vec<String> {
        let mut assumptions = vec![
            "Target audience is reachable through recommended channels".into(),
            "Product is ready for external feedback".into(),
            "Budget and timeline constraints will be provided by user".into(),
        ];

        for persona in &pkb.personas {
            assumptions.push(format!(
                "Persona '{}' is accurately identified (confidence: {:.0}%)",
                persona.name,
                persona.confidence * 100.0
            ));
        }

        assumptions
    }
}

use std::collections::HashMap;
