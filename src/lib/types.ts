export interface Project {
  id: string;
  name: string;
  path: string;
  project_type: string;
  status: string;
  created_at: string;
  updated_at: string;
  pkb_version: number | null;
}

export interface ProductKnowledgeBase {
  project_id: string;
  version: number;
  identity: {
    inferred_name: string;
    tagline_candidates: string[];
    elevator_pitches: string[];
    category: string;
    maturity_signals: {
      has_tests: boolean;
      has_docs: boolean;
      has_ci: boolean;
      commit_frequency: string;
      contributor_count: number;
      test_coverage: number | null;
    };
  };
  features: Array<{
    id: string;
    name: string;
    description: string;
    code_references: string[];
    differentiation_score: number;
    marketing_ready: boolean;
  }>;
  personas: Array<{
    id: string;
    name: string;
    description: string;
    pain_points: string[];
    value_propositions: string[];
    preferred_channels: string[];
    confidence: number;
  }>;
  technical: {
    languages: Array<{
      name: string;
      file_count: number;
      line_count: number;
      confidence: number;
    }>;
    framework: string | null;
    architecture_summary: string;
    tech_stack: string[];
    quality_assessment: {
      test_coverage: number | null;
      documentation_score: number;
      code_quality_score: number;
      type_safety_score: number;
      has_ci: boolean;
      overall_score: number;
    };
    dependencies: Array<{
      name: string;
      version: string;
      category: string;
    }>;
  };
  brand: {
    inferred_voice: {
      tone: string[];
      complexity: string;
      personality_traits: string[];
    };
    vocabulary: {
      preferred: string[];
      avoid: string[];
    };
  };
  competitive: {
    competitors: any[];
    positioning_opportunities: string[];
    market_gaps: string[];
  };
  knowledge_graph: {
    nodes: Array<{
      id: string;
      node_type: string;
      label: string;
    }>;
    edges: Array<{
      id: string;
      source: string;
      target: string;
      edge_type: string;
    }>;
  };
}

export interface Strategy {
  id: string;
  project_id: string;
  campaign_id: string;
  executive_summary: string;
  objectives: Array<{
    description: string;
    target: string;
    timeline: string;
    confidence: number;
  }>;
  phases: Array<{
    name: string;
    duration: string;
    channels: Array<{
      channel: string;
      platforms: string[];
      tactics: Array<{
        name: string;
        description: string;
        effort: string;
        expected_impact: string;
      }>;
      frequency: string;
    }>;
    milestones: string[];
  }>;
  asset_requirements: {
    total_count: number;
    by_type: Record<string, number>;
    production_schedule: string[];
  };
  risks: Array<{
    description: string;
    probability: string;
    impact: string;
    mitigation: string;
  }>;
}

export interface ContentAsset {
  id: string;
  project_id: string;
  campaign_id: string | null;
  asset_type: string;
  subtype: string;
  platform: string;
  status: string;
  content: {
    text: string | null;
    file_path: string | null;
    metadata: Record<string, any>;
  };
  performance: {
    impressions: number;
    engagement: number;
    clicks: number;
  };
  created_at: string;
}

export interface AppSettings {
  ai_providers: Record<
    string,
    {
      provider: string;
      api_key: string | null;
      model: string;
      max_tokens: number;
      temperature: number;
      enabled: boolean;
    }
  >;
  default_autonomy: Record<string, string>;
  theme: string;
}

export interface Campaign {
  id: string;
  project_id: string;
  name: string;
  status: string;
  phases: any[];
  total_budget: number;
  spent_budget: number;
  created_at: string;
}
