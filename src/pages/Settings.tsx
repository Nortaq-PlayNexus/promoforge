import { useState, useEffect } from "react";
import { api } from "../lib/api";
import type { AppSettings } from "../lib/types";
import { Settings as SettingsIcon, Save, Key, AlertCircle } from "lucide-react";

export function Settings() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const data = await api.getSettings();
      setSettings(data);
    } catch (e) {
      console.error("Failed to load settings:", e);
    }
  };

  const handleSave = async () => {
    if (!settings) return;
    try {
      setSaving(true);
      await api.updateSettings(settings);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      console.error("Failed to save settings:", e);
    } finally {
      setSaving(false);
    }
  };

  const updateProvider = (
    key: string,
    field: string,
    value: any
  ) => {
    if (!settings) return;
    setSettings({
      ...settings,
      ai_providers: {
        ...settings.ai_providers,
        [key]: {
          ...settings.ai_providers[key],
          [field]: value,
        },
      },
    });
  };

  if (!settings) {
    return (
      <div className="p-6 max-w-4xl mx-auto">
        <div className="card text-center py-12">
          <p className="text-surface-5">Loading settings...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 max-w-4xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Settings</h1>
          <p className="text-surface-5 text-sm mt-1">
            Configure AI providers and application preferences
          </p>
        </div>
        <button
          onClick={handleSave}
          disabled={saving}
          className="btn-primary flex items-center gap-2"
        >
          <Save className="w-4 h-4" />
          {saving ? "Saving..." : saved ? "Saved!" : "Save Settings"}
        </button>
      </div>

      {/* AI Providers */}
      <div className="card space-y-6">
        <div className="flex items-center gap-2">
          <Key className="w-5 h-5 text-brand-400" />
          <h2 className="text-lg font-semibold">AI Providers</h2>
        </div>

        <div className="space-y-4">
          {Object.entries(settings.ai_providers).map(([key, provider]) => (
            <div
              key={key}
              className={`p-4 rounded-lg border ${
                provider.enabled
                  ? "bg-surface-3 border-brand-600/30"
                  : "bg-surface-2 border-surface-4"
              }`}
            >
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center gap-3">
                  <h3 className="font-medium capitalize">{provider.provider}</h3>
                  {provider.enabled && (
                    <span className="badge-green">Active</span>
                  )}
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={provider.enabled}
                    onChange={(e) =>
                      updateProvider(key, "enabled", e.target.checked)
                    }
                    className="sr-only peer"
                  />
                  <div className="w-9 h-5 bg-surface-4 peer-focus:ring-2 peer-focus:ring-brand-600 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-brand-600"></div>
                </label>
              </div>

              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="text-xs text-surface-5 mb-1 block">
                    API Key
                  </label>
                  <input
                    type="password"
                    value={provider.api_key || ""}
                    onChange={(e) =>
                      updateProvider(key, "api_key", e.target.value || null)
                    }
                    placeholder="sk-..."
                    className="input w-full"
                  />
                </div>
                <div>
                  <label className="text-xs text-surface-5 mb-1 block">
                    Model
                  </label>
                  <input
                    type="text"
                    value={provider.model}
                    onChange={(e) =>
                      updateProvider(key, "model", e.target.value)
                    }
                    className="input w-full"
                  />
                </div>
              </div>

              <div className="grid grid-cols-2 gap-3 mt-3">
                <div>
                  <label className="text-xs text-surface-5 mb-1 block">
                    Max Tokens
                  </label>
                  <input
                    type="number"
                    value={provider.max_tokens}
                    onChange={(e) =>
                      updateProvider(key, "max_tokens", Number(e.target.value))
                    }
                    className="input w-full"
                  />
                </div>
                <div>
                  <label className="text-xs text-surface-5 mb-1 block">
                    Temperature
                  </label>
                  <input
                    type="number"
                    value={provider.temperature}
                    onChange={(e) =>
                      updateProvider(
                        key,
                        "temperature",
                        Number(e.target.value)
                      )
                    }
                    step="0.1"
                    min="0"
                    max="2"
                    className="input w-full"
                  />
                </div>
              </div>

              {!provider.api_key && provider.enabled && (
                <div className="mt-3 flex items-center gap-2 text-xs text-amber-400">
                  <AlertCircle className="w-3 h-3" />
                  API key required for this provider to work
                </div>
              )}
            </div>
          ))}
        </div>
      </div>

      {/* Autonomy Levels */}
      <div className="card space-y-4">
        <h2 className="text-lg font-semibold">Autonomy Levels</h2>
        <p className="text-xs text-surface-5">
          Configure how much control AI agents have over each action type.
        </p>
        <div className="grid grid-cols-2 gap-4">
          {Object.entries(settings.default_autonomy).map(([key, level]) => (
            <div key={key} className="flex items-center justify-between p-3 bg-surface-3 rounded-lg">
              <span className="text-sm capitalize">
                {key.replace(/_/g, " ")}
              </span>
              <select
                value={level}
                onChange={(e) =>
                  setSettings({
                    ...settings,
                    default_autonomy: {
                      ...settings.default_autonomy,
                      [key]: e.target.value,
                    },
                  })
                }
                className="input text-sm py-1 w-32"
              >
                <option value="auto">Auto</option>
                <option value="supervised">Supervised</option>
                <option value="batch_review">Batch Review</option>
                <option value="manual">Manual</option>
              </select>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
