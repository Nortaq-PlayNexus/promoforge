use promoforge_core::*;

pub fn select_playbook(project_type: &ProjectType) -> Playbook {
    match project_type {
        ProjectType::SteamGame => steam_game_playbook(),
        ProjectType::CliTool | ProjectType::OpenSourceLibrary => dev_tools_playbook(),
        ProjectType::MobileApp => mobile_app_playbook(),
        ProjectType::SaasPlatform | ProjectType::WebApplication => saas_playbook(),
        ProjectType::DesktopApplication => desktop_app_playbook(),
        _ => general_playbook(),
    }
}

fn steam_game_playbook() -> Playbook {
    Playbook {
        id: "steam-game-v1".into(),
        name: "Steam/Indie Game Launch".into(),
        description: "Comprehensive Steam game launch strategy covering wishlisting, Next Fest, launch week, and post-launch sustain.".into(),
        project_type: ProjectType::SteamGame,
        phases: vec![
            PlaybookPhase {
                name: "Wishlisting".into(),
                duration: "Weeks -16 to -8".into(),
                objectives: vec![
                    "Build initial wishlist base".into(),
                    "Establish social presence".into(),
                    "Begin community building".into(),
                ],
                channels: vec!["Steam".into(), "Twitter".into(), "Reddit".into(), "Discord".into(), "YouTube".into()],
                key_tactics: vec![
                    "Steam store page creation with AIDA description".into(),
                    "Capsule art creation (3 variants each)".into(),
                    "First trailer (60-90s, hook in first 3s)".into(),
                    "Genre community engagement on Reddit".into(),
                    "Devlog content series".into(),
                    "Cross-promotion with similar indie games".into(),
                ],
            },
            PlaybookPhase {
                name: "Pre-Launch".into(),
                duration: "Weeks -8 to -1".into(),
                objectives: vec![
                    "Accelerate wishlist velocity".into(),
                    "Secure press/creator coverage".into(),
                    "Finalize Next Fest strategy".into(),
                ],
                channels: vec!["Steam".into(), "Twitter".into(), "Discord".into(), "YouTube".into(), "Email".into()],
                key_tactics: vec![
                    "Next Fest demo (20-40 min)".into(),
                    "Daily dev streams during Next Fest".into(),
                    "Press kit distribution".into(),
                    "Feature reveal content series".into(),
                    "Countdown content".into(),
                    "Influencer outreach with game keys".into(),
                ],
            },
            PlaybookPhase {
                name: "Launch Week".into(),
                duration: "Days -3 to +7".into(),
                objectives: vec![
                    "Maximize launch visibility".into(),
                    "Convert wishlists to purchases".into(),
                    "Manage community sentiment".into(),
                ],
                channels: vec!["Steam".into(), "Twitter".into(), "Discord".into(), "Reddit".into(), "Email".into()],
                key_tactics: vec![
                    "Multi-channel launch announcement".into(),
                    "Monitor and respond to reviews within 4h".into(),
                    "Share launch milestones".into(),
                    "Real-time community engagement".into(),
                    "Press follow-up".into(),
                    "Bug/crisis monitoring".into(),
                ],
            },
            PlaybookPhase {
                name: "Post-Launch".into(),
                duration: "Weeks 2-12".into(),
                objectives: vec![
                    "Sustain momentum".into(),
                    "Build long-term community".into(),
                    "Plan future updates".into(),
                ],
                channels: vec!["Steam".into(), "Twitter".into(), "Discord".into(), "Reddit".into()],
                key_tactics: vec![
                    "Regular content updates".into(),
                    "Community highlights and UGC".into(),
                    "Steam sale participation".into(),
                    "Achievement updates".into(),
                    "Transparent bug communication".into(),
                    "Long-tail SEO content".into(),
                ],
            },
        ],
        tactics: vec![
            PlaybookTactic {
                name: "Steam Store Page Optimization".into(),
                description: "AIDA framework description, 5-8 tags, 5-8 screenshots with text overlay, capsule art variants".into(),
                phase: "Wishlisting".into(),
                effort: "High".into(),
                expected_impact: "Critical - primary conversion point".into(),
                prerequisites: vec![],
                kpis: vec!["Wishlist count".into(), "Store page conversion rate".into()],
            },
            PlaybookTactic {
                name: "Devlog Content Series".into(),
                description: "Behind-the-scenes development updates showing progress, challenges, and wins".into(),
                phase: "Wishlisting".into(),
                effort: "Medium".into(),
                expected_impact: "High - builds community and wishlist velocity".into(),
                prerequisites: vec![],
                kpis: vec!["Wishlist velocity".into(), "Community engagement".into()],
            },
            PlaybookTactic {
                name: "Reddit Community Engagement".into(),
                description: "Value-first participation in relevant subreddits (r/indiegaming, r/gamedev, genre-specific)".into(),
                phase: "Wishlisting".into(),
                effort: "Medium".into(),
                expected_impact: "High - direct access to target audience".into(),
                prerequisites: vec![],
                kpis: vec!["Upvotes".into(), "Comments".into(), "Wishlist conversions".into()],
            },
            PlaybookTactic {
                name: "Next Fest Demo Strategy".into(),
                description: "20-40 min polished demo with wishlist prompt at end, daily dev streams".into(),
                phase: "Pre-Launch".into(),
                effort: "High".into(),
                expected_impact: "Critical - major wishlist driver".into(),
                prerequisites: vec!["Playable demo".into()],
                kpis: vec!["Demo downloads".into(), "Play time".into(), "Wishlists from demo".into()],
            },
            PlaybookTactic {
                name: "Press Kit Distribution".into(),
                description: "Professional press kit with screenshots, trailers, fact sheet, founder bio, keys".into(),
                phase: "Pre-Launch".into(),
                effort: "Medium".into(),
                expected_impact: "Medium-High - secures coverage".into(),
                prerequisites: vec!["Press kit assets".into()],
                kpis: vec!["Press coverage".into(), "Creator videos".into()],
            },
            PlaybookTactic {
                name: "Launch Day Multi-Channel Blitz".into(),
                description: "Coordinated announcement across all channels with real-time engagement".into(),
                phase: "Launch Week".into(),
                effort: "High".into(),
                expected_impact: "Critical - launch momentum".into(),
                prerequisites: vec!["Store page live".into(), "All assets ready".into()],
                kpis: vec!["Launch day sales".into(), "Review count".into(), "Social reach".into()],
            },
            PlaybookTactic {
                name: "Review Response System".into(),
                description: "Respond to every review within 4 hours, address concerns publicly".into(),
                phase: "Launch Week".into(),
                effort: "High".into(),
                expected_impact: "High - builds trust and improves review score".into(),
                prerequisites: vec![],
                kpis: vec!["Review score".into(), "Response time".into()],
            },
        ],
        asset_requirements: vec![
            "Steam capsule art (header, small, large)".into(),
            "Steam screenshots (5-8)".into(),
            "Launch trailer (60-90s)".into(),
            "Social media graphics".into(),
            "Press kit".into(),
            "Discord server setup".into(),
            "Devlog content".into(),
        ],
    }
}

fn dev_tools_playbook() -> Playbook {
    Playbook {
        id: "dev-tools-v1".into(),
        name: "Developer Tools / OSS Launch".into(),
        description: "Strategy for developer tools and open-source libraries focusing on GitHub community, HN launch, and organic virality.".into(),
        project_type: ProjectType::CliTool,
        phases: vec![
            PlaybookPhase {
                name: "Foundation".into(),
                duration: "Weeks -4 to -1".into(),
                objectives: vec![
                    "Polish README and docs".into(),
                    "Build initial community".into(),
                    "Prepare launch assets".into(),
                ],
                channels: vec!["GitHub".into(), "Twitter".into(), "Discord".into()],
                key_tactics: vec![
                    "README excellence (AIDA, badges, hero GIF, quick start)".into(),
                    "Contributing guidelines".into(),
                    "CI/CD setup with badges".into(),
                    "Community Discord/Slack".into(),
                    "Twitter developer presence".into(),
                ],
            },
            PlaybookPhase {
                name: "Launch".into(),
                duration: "Weeks 1-2".into(),
                objectives: vec![
                    "Hacker News /Show HN traction".into(),
                    "GitHub star velocity".into(),
                    "npm/PyPI download spike".into(),
                ],
                channels: vec!["Hacker News".into(), "Twitter".into(), "Reddit".into(), "Dev.to".into()],
                key_tactics: vec![
                    "Show HN post (Tue-Thu 9-11 AM EST)".into(),
                    "Reddit r/programming, r/rust, r/python posts".into(),
                    "Dev.to / Hashnode article".into(),
                    "Twitter thread with code demos".into(),
                    "Respond to every HN comment in first 2h".into(),
                ],
            },
            PlaybookPhase {
                name: "Growth".into(),
                duration: "Weeks 3-12".into(),
                objectives: vec![
                    "Sustained community growth".into(),
                    "Regular release cadence".into(),
                    "Integration partnerships".into(),
                ],
                channels: vec!["GitHub".into(), "Twitter".into(), "Dev.to".into(), "YouTube".into()],
                key_tactics: vec![
                    "Bi-weekly releases with changelogs".into(),
                    "Technical blog posts".into(),
                    "Conference talk proposals".into(),
                    "Awesome-list submissions".into(),
                    "Newsletter features".into(),
                    "Integration documentation".into(),
                ],
            },
        ],
        tactics: vec![
            PlaybookTactic {
                name: "README Excellence".into(),
                description: "One-liner with badges, hero GIF, quick start (<60s), features, comparison, contributing guide".into(),
                phase: "Foundation".into(),
                effort: "High".into(),
                expected_impact: "Critical - first impression and conversion point".into(),
                prerequisites: vec![],
                kpis: vec!["GitHub stars".into(), "Fork rate".into()],
            },
            PlaybookTactic {
                name: "Show HN Launch".into(),
                description: "Technical, honest Show HN post. Short title, live demo URL, respond to every comment.".into(),
                phase: "Launch".into(),
                effort: "Medium".into(),
                expected_impact: "High - viral potential in developer community".into(),
                prerequisites: vec!["Working demo".into()],
                kpis: vec!["HN points".into(), "Comments".into(), "GitHub stars from HN".into()],
            },
            PlaybookTactic {
                name: "Technical Content Marketing".into(),
                description: "In-depth blog posts solving real problems, showing how the tool helps".into(),
                phase: "Growth".into(),
                effort: "Medium".into(),
                expected_impact: "Medium-High - long-tail organic discovery".into(),
                prerequisites: vec![],
                kpis: vec!["Page views".into(), "npm downloads".into(), "GitHub stars".into()],
            },
            PlaybookTactic {
                name: "Directory & Awesome List Submissions".into(),
                description: "Submit to relevant curated lists, awesome-*, and developer directories".into(),
                phase: "Growth".into(),
                effort: "Low".into(),
                expected_impact: "Medium - compounding discovery over time".into(),
                prerequisites: vec!["Polished README".into()],
                kpis: vec!["Referral traffic".into(), "Stars from lists".into()],
            },
        ],
        asset_requirements: vec![
            "README with badges and hero GIF".into(),
            "Demo GIF/video".into(),
            "Twitter/X thread content".into(),
            "Blog post / tutorial".into(),
            "Press kit / fact sheet".into(),
            "npm/PyPI package description".into(),
        ],
    }
}

fn mobile_app_playbook() -> Playbook {
    Playbook {
        id: "mobile-app-v1".into(),
        name: "Mobile App ASO Launch".into(),
        description: "App Store Optimization and mobile growth strategy.".into(),
        project_type: ProjectType::MobileApp,
        phases: vec![
            PlaybookPhase {
                name: "ASO Foundation".into(),
                duration: "Weeks -4 to -1".into(),
                objectives: vec!["Keyword optimization".into(), "Screenshot optimization".into()],
                channels: vec!["App Store".into(), "Play Store".into(), "Social".into()],
                key_tactics: vec!["Keyword research".into(), "Screenshot A/B testing".into(), "Description optimization".into()],
            },
            PlaybookPhase {
                name: "Launch".into(),
                duration: "Weeks 1-2".into(),
                objectives: vec!["Download velocity".into(), "Review collection".into()],
                channels: vec!["App Store".into(), "Play Store".into(), "Social".into(), "Reddit".into()],
                key_tactics: vec!["Launch announcement".into(), "Review prompts".into(), "Social campaign".into()],
            },
            PlaybookPhase {
                name: "Growth".into(),
                duration: "Weeks 3-12".into(),
                objectives: vec!["Organic growth".into(), "Retention improvement".into()],
                channels: vec!["App Store".into(), "Play Store".into(), "Social".into()],
                key_tactics: vec!["Localisation".into(), "Custom Product Pages".into(), "Update marketing".into()],
            },
        ],
        tactics: vec![
            PlaybookTactic {
                name: "Keyword Optimization".into(),
                description: "Research and optimize title, subtitle, keyword field, description for search ranking".into(),
                phase: "ASO Foundation".into(),
                effort: "High".into(),
                expected_impact: "Critical - primary organic discovery channel".into(),
                prerequisites: vec![],
                kpis: vec!["Keyword rankings".into(), "Search impressions".into()],
            },
        ],
        asset_requirements: vec![
            "App Store screenshots (5-10)".into(),
            "App preview video".into(),
            "App icon variants".into(),
            "Social media graphics".into(),
        ],
    }
}

fn saas_playbook() -> Playbook {
    Playbook {
        id: "saas-v1".into(),
        name: "SaaS / Product Launch".into(),
        description: "Product Hunt launch, landing page optimization, and PLG growth strategy.".into(),
        project_type: ProjectType::SaasPlatform,
        phases: vec![
            PlaybookPhase {
                name: "Pre-Launch".into(),
                duration: "Weeks -4 to -1".into(),
                objectives: vec!["Build waitlist".into(), "Find Product Hunt hunter".into()],
                channels: vec!["Product Hunt".into(), "Twitter".into(), "Email".into()],
                key_tactics: vec!["Landing page".into(), "Waitlist referral loop".into(), "Community building".into()],
            },
            PlaybookPhase {
                name: "Launch".into(),
                duration: "Week 1".into(),
                objectives: vec!["Product Hunt top 5".into(), "MRR target".into()],
                channels: vec!["Product Hunt".into(), "Hacker News".into(), "Twitter".into(), "Email".into()],
                key_tactics: vec!["PH launch at 12:01 AM PST".into(), "HN post".into(), "Email blast".into()],
            },
            PlaybookPhase {
                name: "Growth".into(),
                duration: "Weeks 2-12".into(),
                objectives: vec!["Sustainable MRR growth".into(), "Reduce churn".into()],
                channels: vec!["Content".into(), "Social".into(), "Email".into(), "Partnerships".into()],
                key_tactics: vec!["Case studies".into(), "PLG optimization".into(), "Integration partnerships".into()],
            },
        ],
        tactics: vec![
            PlaybookTactic {
                name: "Product Hunt Launch".into(),
                description: "Full PH strategy: hunter, maker comment, supporter briefing, milestone sharing".into(),
                phase: "Launch".into(),
                effort: "High".into(),
                expected_impact: "High - major visibility spike".into(),
                prerequisites: vec!["Hunter secured".into(), "Assets ready".into()],
                kpis: vec!["PH rank".into(), "Upvotes".into(), "Signups from PH".into()],
            },
        ],
        asset_requirements: vec![
            "Product Hunt assets".into(),
            "Landing page".into(),
            "Demo video".into(),
            "Social media kit".into(),
        ],
    }
}

fn desktop_app_playbook() -> Playbook {
    saas_playbook() // Desktop apps follow similar SaaS-like launch patterns
}

fn general_playbook() -> Playbook {
    Playbook {
        id: "general-v1".into(),
        name: "General Software Launch".into(),
        description: "General-purpose software launch strategy covering social media, content marketing, and community building.".into(),
        project_type: ProjectType::Unknown,
        phases: vec![
            PlaybookPhase {
                name: "Foundation".into(),
                duration: "Weeks -2 to -1".into(),
                objectives: vec!["Establish online presence".into(), "Create core messaging".into()],
                channels: vec!["Twitter".into(), "LinkedIn".into(), "Website".into()],
                key_tactics: vec!["Brand identity".into(), "Social profiles".into(), "Landing page".into()],
            },
            PlaybookPhase {
                name: "Launch".into(),
                duration: "Week 1".into(),
                objectives: vec!["Generate initial buzz".into(), "Get first users".into()],
                channels: vec!["Twitter".into(), "Reddit".into(), "LinkedIn".into(), "Email".into()],
                key_tactics: vec!["Multi-channel announcement".into(), "Community posts".into(), "Direct outreach".into()],
            },
            PlaybookPhase {
                name: "Growth".into(),
                duration: "Weeks 2-8".into(),
                objectives: vec!["Sustain growth".into(), "Build community".into()],
                channels: vec!["Twitter".into(), "Blog".into(), "Email".into()],
                key_tactics: vec!["Content marketing".into(), "User testimonials".into(), "Feature updates".into()],
            },
        ],
        tactics: vec![
            PlaybookTactic {
                name: "Multi-Channel Launch Announcement".into(),
                description: "Coordinated launch across all connected social channels with tailored messaging".into(),
                phase: "Launch".into(),
                effort: "Medium".into(),
                expected_impact: "High - initial visibility spike".into(),
                prerequisites: vec![],
                kpis: vec!["Reach".into(), "Engagement".into(), "Conversions".into()],
            },
        ],
        asset_requirements: vec![
            "Social media graphics".into(),
            "Launch announcement copy".into(),
            "Press release".into(),
        ],
    }
}
