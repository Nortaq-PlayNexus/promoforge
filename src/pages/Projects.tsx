import { useState, useEffect } from "react";
import { api } from "../lib/api";
import type { Project, ProductKnowledgeBase } from "../lib/types";
import { open } from "@tauri-apps/plugin-dialog";
import {
  FolderOpen,
  Plus,
  Trash2,
  RefreshCw,
  Check,
  Loader2,
} from "lucide-react";

interface ProjectsProps {
  selectedProject: string | null;
  onSelectProject: (id: string) => void;
}

export function Projects({ selectedProject, onSelectProject }: ProjectsProps) {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(false);
  const [adding, setAdding] = useState(false);
  const [analyzingId, setAnalyzingId] = useState<string | null>(null);
  const [pkbs, setPkbs] = useState<Record<string, ProductKnowledgeBase>>({});

  useEffect(() => {
    loadProjects();
  }, []);

  const loadProjects = async () => {
    try {
      setLoading(true);
      const data = await api.getProjects();
      setProjects(data);

      // Try loading PKBs
      for (const p of data) {
        try {
          const pkb = await api.getPkb(p.id);
          setPkbs((prev) => ({ ...prev, [p.id]: pkb }));
        } catch {
          // No PKB yet
        }
      }
    } catch (e) {
      console.error("Failed to load projects:", e);
    } finally {
      setLoading(false);
    }
  };

  const handleAddProject = async () => {
    try {
      setAdding(true);
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Project Folder",
      });

      if (selected) {
        const path = typeof selected === "string" ? selected : selected;
        const project = await api.createProject(path as string);
        setProjects((prev) => [project, ...prev]);
        onSelectProject(project.id);
      }
    } catch (e) {
      console.error("Failed to add project:", e);
    } finally {
      setAdding(false);
    }
  };

  const handleAnalyze = async (projectId: string) => {
    try {
      setAnalyzingId(projectId);
      const pkb = await api.analyzeProject(projectId);
      setPkbs((prev) => ({ ...prev, [projectId]: pkb }));
      loadProjects();
    } catch (e) {
      console.error("Analysis failed:", e);
    } finally {
      setAnalyzingId(null);
    }
  };

  const projectTypeIcons: Record<string, string> = {
    steam_game: "🎮",
    mobile_app: "📱",
    web_application: "🌐",
    desktop_application: "🖥️",
    cli_tool: "⚡",
    open_source_library: "📚",
    saas_platform: "☁️",
    api_service: "🔌",
    browser_extension: "🧩",
    roblox_experience: "🎲",
    unknown: "📦",
  };

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Projects</h1>
          <p className="text-surface-5 text-sm mt-1">
            Manage your software projects for promotion
          </p>
        </div>
        <button
          onClick={handleAddProject}
          disabled={adding}
          className="btn-primary flex items-center gap-2"
        >
          {adding ? (
            <Loader2 className="w-4 h-4 animate-spin" />
          ) : (
            <Plus className="w-4 h-4" />
          )}
          Add Project
        </button>
      </div>

      {projects.length === 0 && !loading ? (
        <div className="card text-center py-16">
          <FolderOpen className="w-16 h-16 mx-auto mb-4 text-surface-5 opacity-30" />
          <h3 className="text-lg font-medium mb-2">No Projects Yet</h3>
          <p className="text-surface-5 mb-4 max-w-md mx-auto">
            Add a project folder to start analyzing your code and generating
            promotion strategies.
          </p>
          <button
            onClick={handleAddProject}
            className="btn-primary"
          >
            Add Your First Project
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {projects.map((project) => {
            const pkb = pkbs[project.id];
            const isAnalyzing = analyzingId === project.id;
            const isSelected = selectedProject === project.id;

            return (
              <div
                key={project.id}
                onClick={() => onSelectProject(project.id)}
                className={`card-hover ${
                  isSelected ? "border-brand-600 bg-brand-600/5" : ""
                }`}
              >
                <div className="flex items-start gap-3">
                  <div className="text-2xl">
                    {projectTypeIcons[project.project_type] || "📦"}
                  </div>
                  <div className="flex-1 min-w-0">
                    <h3 className="font-medium truncate">{project.name}</h3>
                    <p className="text-xs text-surface-5 truncate">
                      {project.project_type.replace(/_/g, " ")}
                    </p>
                  </div>
                  <span
                    className={
                      project.status === "ready"
                        ? "badge-green"
                        : project.status === "analyzing"
                          ? "badge-yellow"
                          : "badge-gray"
                    }
                  >
                    {project.status}
                  </span>
                </div>

                <p className="text-xs text-surface-5 mt-3 truncate">
                  {project.path}
                </p>

                {pkb && (
                  <div className="mt-3 grid grid-cols-2 gap-2 text-xs">
                    <div className="bg-surface-3 rounded p-2">
                      <span className="text-surface-5">Features</span>
                      <p className="font-medium">{pkb.features.length}</p>
                    </div>
                    <div className="bg-surface-3 rounded p-2">
                      <span className="text-surface-5">Personas</span>
                      <p className="font-medium">{pkb.personas.length}</p>
                    </div>
                    <div className="bg-surface-3 rounded p-2">
                      <span className="text-surface-5">Languages</span>
                      <p className="font-medium">
                        {pkb.technical.languages.length}
                      </p>
                    </div>
                    <div className="bg-surface-3 rounded p-2">
                      <span className="text-surface-5">Quality</span>
                      <p className="font-medium">
                        {Math.round(
                          pkb.technical.quality_assessment.overall_score * 100
                        )}
                        %
                      </p>
                    </div>
                  </div>
                )}

                <div className="mt-3 flex gap-2">
                  {!pkb ? (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleAnalyze(project.id);
                      }}
                      disabled={isAnalyzing}
                      className="btn-primary text-xs flex-1 flex items-center justify-center gap-1"
                    >
                      {isAnalyzing ? (
                        <>
                          <Loader2 className="w-3 h-3 animate-spin" />
                          Analyzing...
                        </>
                      ) : (
                        <>
                          <RefreshCw className="w-3 h-3" />
                          Analyze
                        </>
                      )}
                    </button>
                  ) : (
                    <div className="flex items-center gap-1 text-xs text-emerald-400">
                      <Check className="w-3 h-3" />
                      Analyzed
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
