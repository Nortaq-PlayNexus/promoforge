import { useState, useEffect } from "react";
import { api } from "../lib/api";
import type { Strategy, ProductKnowledgeBase } from "../lib/types";
import type { Page } from "../App";
import {
  Target,
  Loader2,
  AlertTriangle,
  CheckCircle,
  Calendar,
  BarChart3,
  Layers,
} from "lucide-react";

interface StrategyViewProps {
  selectedProject: string | null;
  onNavigate: (page: Page) => void;
}

export function StrategyView({ selectedProject, onNavigate }: StrategyViewProps) {
  const [strategy, setStrategy] = useState<Strategy | null>(null);
  const [pkb, setPkb] = useState<ProductKnowledgeBase | null>(null);
  const [generating, setGenerating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (selectedProject) {
      loadStrategy();
      loadPkb();
    }
  }, [selectedProject]);

  const loadPkb = async () => {
    if (!selectedProject) return;
    try {
      const data = await api.getPkb(selectedProject);
      setPkb(data);
    } catch {
      setPkb(null);
    }
  };

  const loadStrategy = async () => {
    if (!selectedProject) return;
    try {
      const campaigns = await api.listCampaigns(selectedProject);
      if (campaigns.length > 0) {
        // Strategy would be associated with a campaign
      }
    } catch {
      // No strategy yet
    }
  };

  const handleGenerate = async () => {
    if (!selectedProject) return;
    try {
      setGenerating(true);
      setError(null);
      const data = await api.generateStrategy(selectedProject);
      setStrategy(data);
    } catch (e: any) {
      setError(e.toString());
    } finally {
      setGenerating(false);
    }
  };

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Strategy</h1>
          <p className="text-surface-5 text-sm mt-1">
            AI-generated promotion strategy
          </p>
        </div>
        <button
          onClick={handleGenerate}
          disabled={generating || !selectedProject}
          className="btn-primary flex items-center gap-2"
        >
          {generating ? (
            <Loader2 className="w-4 h-4 animate-spin" />
          ) : (
            <Target className="w-4 h-4" />
          )}
          {strategy ? "Regenerate Strategy" : "Generate Strategy"}
        </button>
      </div>

      {!selectedProject && (
        <div className="card text-center py-12">
          <Target className="w-12 h-12 mx-auto mb-3 text-surface-5 opacity-30" />
          <p className="text-surface-5">
            Select a project first to generate a strategy.
          </p>
          <button
            onClick={() => onNavigate("projects")}
            className="btn-secondary mt-3"
          >
            Go to Projects
          </button>
        </div>
      )}

      {selectedProject && !pkb && !generating && (
        <div className="card text-center py-12">
          <AlertTriangle className="w-12 h-12 mx-auto mb-3 text-amber-400 opacity-50" />
          <h3 className="font-medium mb-2">Analysis Required</h3>
          <p className="text-surface-5 mb-3">
            Run project analysis first to build the Product Knowledge Base.
          </p>
          <button
            onClick={() => onNavigate("projects")}
            className="btn-secondary"
          >
            Go to Projects
          </button>
        </div>
      )}

      {error && (
        <div className="card border-red-700 bg-red-900/10">
          <div className="flex items-center gap-2 text-red-400">
            <AlertTriangle className="w-4 h-4" />
            <p className="text-sm">{error}</p>
          </div>
        </div>
      )}

      {generating && (
        <div className="card text-center py-12">
          <Loader2 className="w-12 h-12 mx-auto mb-3 text-brand-400 animate-spin" />
          <h3 className="font-medium mb-2">Generating Strategy...</h3>
          <p className="text-surface-5 text-sm">
            Analyzing your product and creating a comprehensive promotion plan
          </p>
        </div>
      )}

      {strategy && (
        <div className="space-y-6">
          {/* Executive Summary */}
          <div className="card">
            <h2 className="text-lg font-semibold mb-3">Executive Summary</h2>
            <p className="text-sm text-surface-5 leading-relaxed">
              {strategy.executive_summary}
            </p>
          </div>

          {/* Objectives */}
          <div className="card">
            <h2 className="text-lg font-semibold mb-3 flex items-center gap-2">
              <BarChart3 className="w-5 h-5 text-brand-400" />
              Objectives
            </h2>
            <div className="space-y-3">
              {strategy.objectives.map((obj, i) => (
                <div key={i} className="p-3 bg-surface-3 rounded-lg">
                  <div className="flex items-center justify-between">
                    <p className="text-sm font-medium">{obj.description}</p>
                    <span className="badge-blue">{obj.timeline}</span>
                  </div>
                  <div className="flex items-center gap-4 mt-2 text-xs text-surface-5">
                    <span>Target: {obj.target}</span>
                    <span>
                      Confidence: {Math.round(obj.confidence * 100)}%
                    </span>
                  </div>
                  <div className="mt-2 h-1.5 bg-surface-4 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-brand-600 rounded-full"
                      style={{ width: `${obj.confidence * 100}%` }}
                    />
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Phases */}
          <div className="card">
            <h2 className="text-lg font-semibold mb-3 flex items-center gap-2">
              <Layers className="w-5 h-5 text-brand-400" />
              Campaign Phases
            </h2>
            <div className="space-y-4">
              {strategy.phases.map((phase, i) => (
                <div key={i} className="p-4 bg-surface-3 rounded-lg">
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="font-medium">{phase.name}</h3>
                    <span className="badge-gray">{phase.duration}</span>
                  </div>

                  <div className="flex flex-wrap gap-2 mb-3">
                    {phase.channels.map((ch, j) => (
                      <span key={j} className="badge-blue text-[10px]">
                        {ch.channel}
                        {ch.tactics.length > 0 &&
                          ` (${ch.tactics.length} tactics)`}
                      </span>
                    ))}
                  </div>

                  {phase.milestones.length > 0 && (
                    <div className="space-y-1">
                      <p className="text-xs text-surface-5">Milestones:</p>
                      {phase.milestones.map((m, j) => (
                        <div key={j} className="flex items-center gap-2 text-xs">
                          <CheckCircle className="w-3 h-3 text-emerald-400" />
                          <span>{m}</span>
                        </div>
                      ))}
                    </div>
                  )}

                  {phase.channels.map((ch, j) =>
                    ch.tactics.length > 0 ? (
                      <div key={j} className="mt-3">
                        <p className="text-xs text-surface-5 mb-2">
                          Tactics for {ch.channel}:
                        </p>
                        <div className="space-y-2">
                          {ch.tactics.map((tactic, k) => (
                            <div
                              key={k}
                              className="p-2 bg-surface-4 rounded text-xs"
                            >
                              <div className="flex items-center justify-between">
                                <span className="font-medium">
                                  {tactic.name}
                                </span>
                                <span className="badge-gray text-[9px]">
                                  {tactic.effort}
                                </span>
                              </div>
                              <p className="text-surface-5 mt-1">
                                {tactic.description}
                              </p>
                              <p className="text-surface-5 mt-1">
                                Impact: {tactic.expected_impact}
                              </p>
                            </div>
                          ))}
                        </div>
                      </div>
                    ) : null
                  )}
                </div>
              ))}
            </div>
          </div>

          {/* Risks */}
          <div className="card">
            <h2 className="text-lg font-semibold mb-3 flex items-center gap-2">
              <AlertTriangle className="w-5 h-5 text-amber-400" />
              Risks
            </h2>
            <div className="space-y-2">
              {strategy.risks.map((risk, i) => (
                <div key={i} className="p-3 bg-surface-3 rounded-lg">
                  <div className="flex items-center gap-3">
                    <div
                      className={`w-2 h-2 rounded-full ${
                        risk.probability === "High"
                          ? "bg-red-400"
                          : risk.probability === "Medium"
                            ? "bg-amber-400"
                            : "bg-emerald-400"
                      }`}
                    />
                    <div className="flex-1">
                      <p className="text-sm font-medium">{risk.description}</p>
                      <p className="text-xs text-surface-5">
                        {risk.mitigation}
                      </p>
                    </div>
                    <div className="text-right text-xs">
                      <div className="text-surface-5">
                        P: {risk.probability}
                      </div>
                      <div className="text-surface-5">I: {risk.impact}</div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Asset Requirements */}
          <div className="card">
            <h2 className="text-lg font-semibold mb-3 flex items-center gap-2">
              <Calendar className="w-5 h-5 text-brand-400" />
              Asset Requirements
            </h2>
            <div className="grid grid-cols-3 gap-3 mb-4">
              {Object.entries(strategy.asset_requirements.by_type).map(
                ([type, count]) => (
                  <div key={type} className="p-3 bg-surface-3 rounded-lg text-center">
                    <p className="text-2xl font-bold">{count}</p>
                    <p className="text-xs text-surface-5">{type}</p>
                  </div>
                )
              )}
            </div>
            {strategy.asset_requirements.production_schedule.length > 0 && (
              <div className="space-y-1">
                <p className="text-xs text-surface-5 mb-2">Production Schedule:</p>
                {strategy.asset_requirements.production_schedule.map(
                  (step, i) => (
                    <div key={i} className="flex items-center gap-2 text-xs">
                      <div className="w-1.5 h-1.5 bg-brand-400 rounded-full" />
                      <span>{step}</span>
                    </div>
                  )
                )}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
