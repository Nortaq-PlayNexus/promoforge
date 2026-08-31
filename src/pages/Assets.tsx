import { useState, useEffect } from "react";
import { api } from "../lib/api";
import type {
  ProductKnowledgeBase,
  ContentAsset,
} from "../lib/types";
import {
  FileText,
  Image,
  Loader2,
  Check,
  X,
  ChevronDown,
  ChevronUp,
} from "lucide-react";

interface AssetsProps {
  selectedProject: string | null;
}

type Tab = "pkb" | "generate" | "library";

export function Assets({ selectedProject }: AssetsProps) {
  const [tab, setTab] = useState<Tab>("pkb");
  const [pkb, setPkb] = useState<ProductKnowledgeBase | null>(null);
  const [assets, setAssets] = useState<ContentAsset[]>([]);
  const [expandedFeature, setExpandedFeature] = useState<string | null>(null);

  // Generation state
  const [genType, setGenType] = useState("social");
  const [genPlatform, setGenPlatform] = useState("twitter");
  const [genCount, setGenCount] = useState(5);
  const [generating, setGenerating] = useState(false);

  useEffect(() => {
    if (selectedProject) {
      loadPkb();
      loadAssets();
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

  const loadAssets = async () => {
    // Assets are loaded through the API
  };

  const handleGenerate = async () => {
    if (!selectedProject) return;
    try {
      setGenerating(true);
      const newAssets = await api.generateTextAssets(
        selectedProject,
        genType,
        genPlatform,
        genCount
      );
      setAssets((prev) => [...newAssets, ...prev]);
    } catch (e) {
      console.error("Generation failed:", e);
    } finally {
      setGenerating(false);
    }
  };

  const tabs: { key: Tab; label: string }[] = [
    { key: "pkb", label: "Product Knowledge Base" },
    { key: "generate", label: "Generate Content" },
    { key: "library", label: "Asset Library" },
  ];

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6">
      <h1 className="text-2xl font-bold">Assets</h1>

      {/* Tabs */}
      <div className="flex gap-1 bg-surface-1 p-1 rounded-lg w-fit">
        {tabs.map((t) => (
          <button
            key={t.key}
            onClick={() => setTab(t.key)}
            className={`px-4 py-2 text-sm rounded-md transition-colors ${
              tab === t.key
                ? "bg-surface-3 text-white"
                : "text-surface-5 hover:text-white"
            }`}
          >
            {t.label}
          </button>
        ))}
      </div>

      {/* PKB Tab */}
      {tab === "pkb" && (
        <div className="space-y-6">
          {!pkb && (
            <div className="card text-center py-12">
              <p className="text-surface-5">
                No Product Knowledge Base available. Run analysis first.
              </p>
            </div>
          )}

          {pkb && (
            <>
              {/* Identity */}
              <div className="card">
                <h2 className="text-lg font-semibold mb-3">Identity</h2>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-xs text-surface-5 mb-1">Name</p>
                    <p className="font-medium">{pkb.identity.inferred_name}</p>
                  </div>
                  <div>
                    <p className="text-xs text-surface-5 mb-1">Category</p>
                    <p className="font-medium">{pkb.identity.category}</p>
                  </div>
                </div>
                {pkb.identity.tagline_candidates.length > 0 && (
                  <div className="mt-3">
                    <p className="text-xs text-surface-5 mb-1">
                      Tagline Candidates
                    </p>
                    <div className="space-y-1">
                      {pkb.identity.tagline_candidates.map((t, i) => (
                        <p key={i} className="text-sm italic text-surface-5">
                          "{t}"
                        </p>
                      ))}
                    </div>
                  </div>
                )}
              </div>

              {/* Features */}
              <div className="card">
                <h2 className="text-lg font-semibold mb-3">
                  Features ({pkb.features.length})
                </h2>
                <div className="space-y-2">
                  {pkb.features.map((feature) => (
                    <div key={feature.id} className="bg-surface-3 rounded-lg">
                      <button
                        onClick={() =>
                          setExpandedFeature(
                            expandedFeature === feature.id
                              ? null
                              : feature.id
                          )
                        }
                        className="w-full flex items-center justify-between p-3"
                      >
                        <div className="flex items-center gap-3">
                          <div className="w-2 h-2 bg-brand-400 rounded-full" />
                          <span className="text-sm font-medium">
                            {feature.name}
                          </span>
                          <span className="text-xs text-surface-5">
                            {feature.code_references.length} files
                          </span>
                        </div>
                        <div className="flex items-center gap-2">
                          <div className="w-16 h-1.5 bg-surface-4 rounded-full overflow-hidden">
                            <div
                              className="h-full bg-brand-600 rounded-full"
                              style={{
                                width: `${feature.differentiation_score * 100}%`,
                              }}
                            />
                          </div>
                          {expandedFeature === feature.id ? (
                            <ChevronUp className="w-4 h-4 text-surface-5" />
                          ) : (
                            <ChevronDown className="w-4 h-4 text-surface-5" />
                          )}
                        </div>
                      </button>
                      {expandedFeature === feature.id && (
                        <div className="px-3 pb-3 border-t border-surface-4">
                          <p className="text-xs text-surface-5 mt-2">
                            {feature.description}
                          </p>
                          {feature.code_references.length > 0 && (
                            <div className="mt-2">
                              <p className="text-xs text-surface-5 mb-1">
                                Code References:
                              </p>
                              {feature.code_references
                                .slice(0, 5)
                                .map((ref, i) => (
                                  <p
                                    key={i}
                                    className="text-xs font-mono text-surface-5"
                                  >
                                    {ref}
                                  </p>
                                ))}
                            </div>
                          )}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </div>

              {/* Personas */}
              <div className="card">
                <h2 className="text-lg font-semibold mb-3">
                  Target Personas ({pkb.personas.length})
                </h2>
                <div className="grid grid-cols-2 gap-3">
                  {pkb.personas.map((persona) => (
                    <div key={persona.id} className="p-3 bg-surface-3 rounded-lg">
                      <div className="flex items-center justify-between mb-2">
                        <h3 className="text-sm font-medium">{persona.name}</h3>
                        <span className="badge-blue text-[10px]">
                          {Math.round(persona.confidence * 100)}%
                        </span>
                      </div>
                      <p className="text-xs text-surface-5 mb-2">
                        {persona.description}
                      </p>
                      {persona.pain_points.length > 0 && (
                        <div className="mb-2">
                          <p className="text-[10px] text-surface-5">
                            Pain Points:
                          </p>
                          {persona.pain_points.map((p, i) => (
                            <p key={i} className="text-xs text-red-400">
                              · {p}
                            </p>
                          ))}
                        </div>
                      )}
                      <div>
                        <p className="text-[10px] text-surface-5">
                          Channels:
                        </p>
                        <div className="flex flex-wrap gap-1 mt-1">
                          {persona.preferred_channels.map((ch, i) => (
                            <span key={i} className="badge-gray text-[9px]">
                              {ch}
                            </span>
                          ))}
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Technical */}
              <div className="card">
                <h2 className="text-lg font-semibold mb-3">Technical</h2>
                <div className="grid grid-cols-2 gap-4 mb-4">
                  <div>
                    <p className="text-xs text-surface-5 mb-1">
                      Architecture
                    </p>
                    <p className="text-sm">{pkb.technical.architecture_summary}</p>
                  </div>
                  <div>
                    <p className="text-xs text-surface-5 mb-1">
                      Framework
                    </p>
                    <p className="text-sm">
                      {pkb.technical.framework || "None detected"}
                    </p>
                  </div>
                </div>

                <div className="mb-4">
                  <p className="text-xs text-surface-5 mb-2">Tech Stack</p>
                  <div className="flex flex-wrap gap-2">
                    {pkb.technical.tech_stack.map((tech, i) => (
                      <span key={i} className="badge-blue">
                        {tech}
                      </span>
                    ))}
                  </div>
                </div>

                <div>
                  <p className="text-xs text-surface-5 mb-2">Languages</p>
                  <div className="space-y-2">
                    {pkb.technical.languages.map((lang, i) => (
                      <div key={i} className="flex items-center gap-3">
                        <span className="text-sm w-24">{lang.name}</span>
                        <div className="flex-1 h-1.5 bg-surface-4 rounded-full overflow-hidden">
                          <div
                            className="h-full bg-brand-600 rounded-full"
                            style={{
                              width: `${lang.confidence * 100}%`,
                            }}
                          />
                        </div>
                        <span className="text-xs text-surface-5 w-20 text-right">
                          {lang.file_count} files ·{" "}
                          {lang.line_count.toLocaleString()} lines
                        </span>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Quality */}
                <div className="mt-4 p-3 bg-surface-3 rounded-lg">
                  <p className="text-xs text-surface-5 mb-2">
                    Quality Assessment
                  </p>
                  <div className="grid grid-cols-4 gap-3 text-center">
                    <div>
                      <p className="text-lg font-bold">
                        {Math.round(
                          pkb.technical.quality_assessment.overall_score * 100
                        )}
                        %
                      </p>
                      <p className="text-[10px] text-surface-5">Overall</p>
                    </div>
                    <div>
                      <p className="text-lg font-bold">
                        {pkb.technical.quality_assessment.has_ci ? "✓" : "✗"}
                      </p>
                      <p className="text-[10px] text-surface-5">CI/CD</p>
                    </div>
                    <div>
                      <p className="text-lg font-bold">
                        {pkb.identity.maturity_signals.has_tests ? "✓" : "✗"}
                      </p>
                      <p className="text-[10px] text-surface-5">Tests</p>
                    </div>
                    <div>
                      <p className="text-lg font-bold">
                        {pkb.identity.maturity_signals.has_docs ? "✓" : "✗"}
                      </p>
                      <p className="text-[10px] text-surface-5">Docs</p>
                    </div>
                  </div>
                </div>
              </div>

              {/* Brand */}
              <div className="card">
                <h2 className="text-lg font-semibold mb-3">Brand Voice</h2>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-xs text-surface-5 mb-1">Tone</p>
                    <div className="flex flex-wrap gap-1">
                      {pkb.brand.inferred_voice.tone.map((t, i) => (
                        <span key={i} className="badge-blue">
                          {t}
                        </span>
                      ))}
                    </div>
                  </div>
                  <div>
                    <p className="text-xs text-surface-5 mb-1">Complexity</p>
                    <p className="text-sm">
                      {pkb.brand.inferred_voice.complexity}
                    </p>
                  </div>
                  <div>
                    <p className="text-xs text-surface-5 mb-1">Personality</p>
                    <div className="flex flex-wrap gap-1">
                      {pkb.brand.inferred_voice.personality_traits.map(
                        (t, i) => (
                          <span key={i} className="badge-gray">
                            {t}
                          </span>
                        )
                      )}
                    </div>
                  </div>
                  <div>
                    <p className="text-xs text-surface-5 mb-1">Avoid</p>
                    <div className="flex flex-wrap gap-1">
                      {pkb.brand.vocabulary.avoid.map((v, i) => (
                        <span key={i} className="badge-red">
                          {v}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            </>
          )}
        </div>
      )}

      {/* Generate Tab */}
      {tab === "generate" && (
        <div className="space-y-4">
          <div className="card">
            <h2 className="text-lg font-semibold mb-3">Generate Content</h2>
            {!pkb && (
              <p className="text-surface-5 text-sm">
                Run project analysis first to enable content generation.
              </p>
            )}
            {pkb && (
              <div className="space-y-4">
                <div className="grid grid-cols-3 gap-4">
                  <div>
                    <label className="text-xs text-surface-5 mb-1 block">
                      Content Type
                    </label>
                    <select
                      value={genType}
                      onChange={(e) => setGenType(e.target.value)}
                      className="input w-full"
                    >
                      <option value="social">Social Media Posts</option>
                      <option value="store">Store Metadata</option>
                      <option value="press_kit">Press Kit</option>
                      <option value="aeo">AEO Content</option>
                    </select>
                  </div>
                  <div>
                    <label className="text-xs text-surface-5 mb-1 block">
                      Platform
                    </label>
                    <select
                      value={genPlatform}
                      onChange={(e) => setGenPlatform(e.target.value)}
                      className="input w-full"
                    >
                      <option value="twitter">Twitter/X</option>
                      <option value="reddit">Reddit</option>
                      <option value="linkedin">LinkedIn</option>
                      <option value="discord">Discord</option>
                      <option value="product_hunt">Product Hunt</option>
                      <option value="steam">Steam</option>
                      <option value="ios">iOS App Store</option>
                      <option value="android">Google Play</option>
                    </select>
                  </div>
                  <div>
                    <label className="text-xs text-surface-5 mb-1 block">
                      Count
                    </label>
                    <input
                      type="number"
                      value={genCount}
                      onChange={(e) => setGenCount(Number(e.target.value))}
                      min={1}
                      max={20}
                      className="input w-full"
                    />
                  </div>
                </div>
                <button
                  onClick={handleGenerate}
                  disabled={generating}
                  className="btn-primary flex items-center gap-2"
                >
                  {generating ? (
                    <Loader2 className="w-4 h-4 animate-spin" />
                  ) : (
                    <FileText className="w-4 h-4" />
                  )}
                  {generating ? "Generating..." : "Generate"}
                </button>
              </div>
            )}
          </div>

          {/* Generated Assets */}
          {assets.length > 0 && (
            <div className="space-y-3">
              <h3 className="font-medium">Generated Content</h3>
              {assets.map((asset) => (
                <div key={asset.id} className="card">
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-2">
                      <span className="badge-blue">{asset.platform}</span>
                      <span className="badge-gray">{asset.subtype}</span>
                    </div>
                    <div className="flex gap-1">
                      <button
                        onClick={async () => {
                          await api.approveContent(asset.id, true);
                          setAssets((prev) =>
                            prev.map((a) =>
                              a.id === asset.id
                                ? { ...a, status: "approved" }
                                : a
                            )
                          );
                        }}
                        className="btn-ghost text-emerald-400"
                      >
                        <Check className="w-4 h-4" />
                      </button>
                      <button
                        onClick={async () => {
                          await api.approveContent(asset.id, false);
                          setAssets((prev) =>
                            prev.map((a) =>
                              a.id === asset.id
                                ? { ...a, status: "rejected" }
                                : a
                            )
                          );
                        }}
                        className="btn-ghost text-red-400"
                      >
                        <X className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                  <p className="text-sm whitespace-pre-wrap">
                    {asset.content.text}
                  </p>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {/* Library Tab */}
      {tab === "library" && (
        <div className="card text-center py-12">
          <Image className="w-12 h-12 mx-auto mb-3 text-surface-5 opacity-30" />
          <p className="text-surface-5">
            Asset library will show all generated and approved content.
          </p>
        </div>
      )}
    </div>
  );
}
