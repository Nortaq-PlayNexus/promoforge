import { useState, useEffect } from "react";
import { api } from "../lib/api";
import type { Project, ProductKnowledgeBase } from "../lib/types";
import type { Page } from "../App";
import { FolderOpen, Zap, Target, ArrowRight, AlertCircle } from "lucide-react";

interface DashboardProps {
  selectedProject: string | null;
  onSelectProject: (id: string) => void;
  onNavigate: (page: Page) => void;
}

export function Dashboard({
  selectedProject,
  onSelectProject,
  onNavigate,
}: DashboardProps) {
  const [projects, setProjects] = useState<Project[]>([]);
  const [pkb, setPkb] = useState<ProductKnowledgeBase | null>(null);
  const [loading, setLoading] = useState(false);
  const [analyzing, setAnalyzing] = useState(false);

  useEffect(() => {
    loadProjects();
  }, []);

  useEffect(() => {
    if (selectedProject) {
      loadPkb(selectedProject);
    }
  }, [selectedProject]);

  const loadProjects = async () => {
    try {
      setLoading(true);
      const data = await api.getProjects();
      setProjects(data);
      if (data.length > 0 && !selectedProject) {
        onSelectProject(data[0].id);
      }
    } catch (e) {
      console.error("Failed to load projects:", e);
    } finally {
      setLoading(false);
    }
  };

  const loadPkb = async (projectId: string) => {
    try {
      const data = await api.getPkb(projectId);
      setPkb(data);
    } catch {
      setPkb(null);
    }
  };

  const handleAnalyze = async () => {
    if (!selectedProject) return;
    try {
      setAnalyzing(true);
      const data = await api.analyzeProject(selectedProject);
      setPkb(data);
      loadProjects();
    } catch (e) {
      console.error("Analysis failed:", e);
    } finally {
      setAnalyzing(false);
    }
  };

  const project = projects.find((p) => p.id === selectedProject);

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <p className="text-surface-5 text-sm mt-1">
          Overview of your promotion operations
        </p>
      </div>

      {/* Quick Stats */}
      <div className="grid grid-cols-4 gap-4">
        <div className="metric-card">
          <span className="metric-label">Projects</span>
          <span className="metric-value">{projects.length}</span>
        </div>
        <div className="metric-card">
          <span className="metric-label">Active Campaigns</span>
          <span className="metric-value">0</span>
        </div>
        <div className="metric-card">
          <span className="metric-label">Generated Assets</span>
          <span className="metric-value">0</span>
        </div>
        <div className="metric-card">
          <span className="metric-label">Health Score</span>
          <span className="metric-value">
            {pkb
              ? `${Math.round(pkb.technical.quality_assessment.overall_score * 100)}%`
              : "—"}
          </span>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-6">
        {/* Project Overview */}
        <div className="col-span-2 card space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-semibold">Current Project</h2>
            {projects.length === 0 && (
              <button
                onClick={() => onNavigate("projects")}
                className="btn-primary text-sm flex items-center gap-2"
              >
                <FolderOpen className="w-4 h-4" />
                Add Project
              </button>
            )}
          </div>

          {project ? (
            <div className="space-y-4">
              <div className="flex items-center gap-3 p-3 bg-surface-3 rounded-lg">
                <div className="w-10 h-10 bg-brand-600/20 rounded-lg flex items-center justify-center">
                  <Zap className="w-5 h-5 text-brand-400" />
                </div>
                <div className="flex-1">
                  <h3 className="font-medium">{project.name}</h3>
                  <p className="text-xs text-surface-5">
                    {project.project_type.replace(/_/g, " ")} · {project.path}
                  </p>
                </div>
                <span
                  className={
                    project.status === "ready" ? "badge-green" : "badge-yellow"
                  }
                >
                  {project.status}
                </span>
              </div>

              {!pkb && (
                <div className="text-center py-6">
                  <p className="text-surface-5 mb-3">
                    This project hasn't been analyzed yet.
                  </p>
                  <button
                    onClick={handleAnalyze}
                    disabled={analyzing}
                    className="btn-primary"
                  >
                    {analyzing ? "Analyzing..." : "Run Deep Analysis"}
                  </button>
                </div>
              )}

              {pkb && (
                <div className="space-y-3">
                  <div className="grid grid-cols-2 gap-3">
                    <div className="p-3 bg-surface-3 rounded-lg">
                      <p className="text-xs text-surface-5 mb-1">Category</p>
                      <p className="text-sm font-medium">
                        {pkb.identity.category}
                      </p>
                    </div>
                    <div className="p-3 bg-surface-3 rounded-lg">
                      <p className="text-xs text-surface-5 mb-1">
                        Tech Stack
                      </p>
                      <p className="text-sm font-medium">
                        {pkb.technical.tech_stack.slice(0, 3).join(", ") ||
                          "Unknown"}
                      </p>
                    </div>
                    <div className="p-3 bg-surface-3 rounded-lg">
                      <p className="text-xs text-surface-5 mb-1">Features</p>
                      <p className="text-sm font-medium">
                        {pkb.features.length} detected
                      </p>
                    </div>
                    <div className="p-3 bg-surface-3 rounded-lg">
                      <p className="text-xs text-surface-5 mb-1">Personas</p>
                      <p className="text-sm font-medium">
                        {pkb.personas.length} inferred
                      </p>
                    </div>
                  </div>

                  <div className="flex gap-2">
                    <button
                      onClick={() => onNavigate("strategy")}
                      className="btn-primary text-sm flex items-center gap-2"
                    >
                      <Target className="w-4 h-4" />
                      Generate Strategy
                      <ArrowRight className="w-3 h-3" />
                    </button>
                    <button
                      onClick={() => onNavigate("assets")}
                      className="btn-secondary text-sm"
                    >
                      View PKB
                    </button>
                  </div>
                </div>
              )}
            </div>
          ) : (
            <div className="text-center py-8 text-surface-5">
              <FolderOpen className="w-12 h-12 mx-auto mb-3 opacity-30" />
              <p>No projects yet. Add a project to get started.</p>
            </div>
          )}
        </div>

        {/* Quick Actions */}
        <div className="card space-y-4">
          <h2 className="text-lg font-semibold">Quick Actions</h2>
          <div className="space-y-2">
            <button
              onClick={() => onNavigate("projects")}
              className="w-full text-left p-3 bg-surface-3 rounded-lg hover:bg-surface-4 transition-colors"
            >
              <p className="text-sm font-medium">Add Project</p>
              <p className="text-xs text-surface-5">
                Analyze a new codebase
              </p>
            </button>
            <button
              onClick={() => onNavigate("strategy")}
              className="w-full text-left p-3 bg-surface-3 rounded-lg hover:bg-surface-4 transition-colors"
            >
              <p className="text-sm font-medium">Generate Strategy</p>
              <p className="text-xs text-surface-5">
                AI-powered campaign plan
              </p>
            </button>
            <button
              onClick={() => onNavigate("assets")}
              className="w-full text-left p-3 bg-surface-3 rounded-lg hover:bg-surface-4 transition-colors"
            >
              <p className="text-sm font-medium">Generate Content</p>
              <p className="text-xs text-surface-5">
                Social posts, store listings, press kits
              </p>
            </button>
            <button
              onClick={() => onNavigate("settings")}
              className="w-full text-left p-3 bg-surface-3 rounded-lg hover:bg-surface-4 transition-colors"
            >
              <p className="text-sm font-medium">Configure AI</p>
              <p className="text-xs text-surface-5">
                Set up API keys and providers
              </p>
            </button>
          </div>

          {/* Tips */}
          <div className="mt-4 p-3 bg-brand-600/10 border border-brand-600/20 rounded-lg">
            <div className="flex gap-2">
              <AlertCircle className="w-4 h-4 text-brand-400 mt-0.5 shrink-0" />
              <div>
                <p className="text-xs font-medium text-brand-300">Getting Started</p>
                <p className="text-xs text-surface-5 mt-1">
                  Add a project folder, run analysis to build the Product Knowledge
                  Base, then generate a strategy and content.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
